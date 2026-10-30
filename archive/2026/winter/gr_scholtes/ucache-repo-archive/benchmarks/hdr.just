set allow-duplicate-variables
import? "../justfile"
# THREADS, REPEAT, DDB_MEM, TIMEOUT_S, ... and the CSV helper.
import "bench_params.just"

# Keeping the first frames of each page resident (DUCKDB_KEEP_HEADER_FRAMES, see
# osv/benchmarks/duckdb/osv_ablation.hpp), on Q6 over tpch300: ablation step 6 as shipped
# (step6) against step 6 keeping the first N frames of each page, the first holding its header
# (step6_hdr<N>, image build/ablation/step6_hdr<N>).
# Warm runs only: one fresh OSv boot per point, REPEAT repetitions inside it. Repetition 1 is
# the cold run that loads the cache and is not recorded; the REPEAT-1 warm ones are averaged.
# On step6_hdr<N>, the first N frames of every page Q6 touches are then already resident.
# Each point runs once per latency setting in LATENCIES, with and without the simulated
# object store.
#   just -f benchmarks/hdr.just build
#   just -f benchmarks/hdr.just run
#   just -f benchmarks/hdr.just summary
#   just -f benchmarks/hdr.just plot
#   just -f benchmarks/hdr.just CACHE_BYTES=<bytes> VARIANT=step6_hdr1 probe
#
# step6_hdr5 (2026-10-18) needed 1664 MiB (1.62%; 1536 MiB hangs) and saved 2.1% / 3.9% over
# step6 at 1.75%, without / with simulated latency: not worth the memory.
# Tuned for step6_hdr1 on irene 2026-10-18: DATA_BYTES=107974170230 (tpch300). CACHE_PCT=0.6%
# (~618 MiB), rounded up from the smallest strictly-passing size found, 629145600 (600 MiB,
# 0.583%); largest failing 524288000 (500 MiB, shortfall/calls=4568/5376). At 600 MiB:
# pinned_MiB=433.7, warm got/want=1.029, shortfall/calls=0/1655, peak_used=2.07 GiB.

self := proot + "/benchmarks/hdr.just"
result_file := results_dir + "/hdr.csv"
images := proot + "/osv/build/ablation"

# The host copy of the dataset on the OSv NVMe, for CACHE_PCT's denominator.
export DATA_DIR := env("DATA_DIR", "/scratch/ilya/tpch300")
# % of the dataset, fractional allowed. The smallest cache the step6_hdr<N> in VARIANTS run Q6 in.
export CACHE_PCT := env("CACHE_PCT", "0.6")
# Overrides CACHE_PCT with exact bytes, for probe.
export CACHE_BYTES := env("CACHE_BYTES", "")
export QUERIES := env("QUERIES", "06")
# 1 cold + 3 warm.
export REPEAT := env("REPEAT", "4")
export BOOTS := env("BOOTS", "1")
# Simulated object store per run, as latency_us:bandwidth_gbps. 0:0 = off;
# 1400:0.7 = MinIO over 100 GbE (measured, see bench_params.just).
export LATENCIES := env("LATENCIES", "0:0 1400:0.7")
export VARIANTS := env("VARIANTS", "step6 step6_hdr1")
export VARIANT := env("VARIANT", "step6_hdr1")
# A guest whose uCache runs out of frames hits an assertion and then idles without exiting.
# Kill it once its CPU time grows by under 1 s in HANG_S seconds, or the log shows an abort.
export HANG_S := env("HANG_S", "30")
hang_re := "aborting|Assertion failed|Out of memory|Unreasonable allocation"

header := "variant,scale,cache_pct,cache_bytes,threads,evict_batch,prefetch_batch,sim_latency_us,sim_bandwidth_gbps,boot,query,repetition,time,metadata_time,read_bytes,prefetch_used,peak_used,min_huge_blocks"

default:
    @just -f {{self}} --list

# The image of every variant in VARIANTS, in build/ablation/<variant>.
build:
    #!/usr/bin/env bash
    set -euo pipefail
    for v in {{VARIANTS}}; do
        n=$(just -f {{self}} _keep_frames "$v")
        (cd {{proot}}/osv && just build-duckdb release 1 0 6 0 "$n")
    done

# The frames per page a variant keeps resident: N for step6_hdr<N>, 0 for step6.
_keep_frames variant:
    #!/usr/bin/env bash
    set -euo pipefail
    case "{{variant}}" in
        step6) echo 0 ;;
        step6_hdr[1-9]*) v="{{variant}}"; echo "${v#step6_hdr}" ;;
        *) echo "unknown variant {{variant}}: step6 or step6_hdr<N>" >&2; exit 1 ;;
    esac

# The folder holding loader.img and loader-stripped.elf of a variant.
_image variant:
    @just -f {{self}} _keep_frames {{variant}} >/dev/null && echo "{{images}}/{{variant}}"

# What a variant's app prints at boot, as an extended regex; a log without it came from
# another image.
_marker variant:
    #!/usr/bin/env bash
    set -euo pipefail
    n=$(just -f {{self}} _keep_frames {{variant}})
    echo "\[ablation\] step=6 .*keep_header_frames=$n([^0-9]|\$)"

# The cache in bytes: CACHE_BYTES, else CACHE_PCT (fractional) of the dataset.
_hdr_cache_bytes:
    #!/usr/bin/env bash
    set -euo pipefail
    if [ -n "{{CACHE_BYTES}}" ]; then echo "{{CACHE_BYTES}}"; exit 0; fi
    if [ "{{CACHE_PCT}}" = TBD ]; then
        echo "CACHE_PCT is not tuned yet: set CACHE_PCT or CACHE_BYTES" >&2; exit 1
    fi
    data=$(just -f {{self}} _data_bytes)
    gawk -v d="$data" -v p="{{CACHE_PCT}}" 'BEGIN { printf "%d\n", d * p / 100 }'

# VM size: the cache, rounded up to whole GiB, plus VM_HEADROOM.
_hdr_vm_mem cache:
    @echo "$(( ({{cache}} + 1073741823) / 1073741824 + $(echo {{VM_HEADROOM}} | tr -d Gg) ))G"

# One boot of image variant running query q, REPEAT times, into log, at simulated latency lat
# (us) and bandwidth bw (GB/s). Prints PASS, HANG (killed by the watchdog), TIMEOUT or FAIL,
# and exits 0 only on PASS.
_run_one variant q cache log lat="0" bw="0":
    #!/usr/bin/env bash
    set -uo pipefail
    img=$(just -f {{self}} _image {{variant}}) || exit 1
    marker=$(just -f {{self}} _marker {{variant}}) || exit 1
    mem=$(just -f {{self}} _hdr_vm_mem {{cache}})
    # VFIO locks all of the VM's memory.
    if [ "$(ulimit -l)" != unlimited ]; then
        sudo -n prlimit --memlock=unlimited:unlimited --pid $$ || exit 1
    fi
    # A killed QEMU holds the SSD and the image until it exits.
    while pgrep -u "$(id -u)" qemu-system >/dev/null; do sleep 5; done
    q="{{q}}"
    # LD_LIBRARY_PATH from the nix shell can load a mismatched alsa-lib into QEMU.
    # </dev/null: QEMU under `timeout` stops on SIGTTIN when it touches the tty.
    timeout -k 10 {{TIMEOUT_S}} env -u LD_LIBRARY_PATH bash -c \
        "cd {{proot}}/osv && just run-duckdb ${q#0} {{REPEAT}} $mem {{THREADS}} {{cache}} {{DDB_MEM}} {{EVICT_BATCH}} {{PREFETCH_BATCH}} '' {{lat}} {{bw}} $img {{CPUSET}}" \
        > "{{log}}" 2>&1 </dev/null &
    pid=$!
    qpid=""; last=-1; idle=0; st=""
    while kill -0 $pid 2>/dev/null; do
        sleep 5
        if grep -Eq "{{hang_re}}" "{{log}}"; then st=HANG; break; fi
        [ -n "$qpid" ] || qpid=$(pgrep -u "$(id -u)" -n qemu-system) || continue
        # Only once the queries run: boot and metadata loading may idle on I/O.
        grep -q "=== RUN 1/\|=== TPC-H Q" "{{log}}" || continue
        # utime + stime of all QEMU threads, in clock ticks (100/s).
        cpu=$(sed 's/.*) //' /proc/$qpid/stat 2>/dev/null | gawk '{ print $12 + $13 }') || break
        [ -n "$cpu" ] || break
        if [ $last -ge 0 ] && [ $(( cpu - last )) -lt 20 ]; then
            idle=$(( idle + 5 ))
        else
            idle=0
        fi
        last=$cpu
        if [ $idle -ge {{HANG_S}} ]; then st=HANG; break; fi
    done
    if [ "$st" = HANG ]; then
        pkill -9 -u "$(id -u)" qemu-system; kill $pid 2>/dev/null; wait $pid
        while pgrep -u "$(id -u)" qemu-system >/dev/null; do sleep 2; done
        echo HANG; exit 1
    fi
    wait $pid; rc=$?
    if [ $rc -eq 124 ] || [ $rc -eq 137 ]; then echo TIMEOUT; exit 1; fi
    # A log without the variant's marker came from a stale image, one without the simulation
    # line from an image that ignores the latency. A [hdrkeep] line means the old tuning
    # telemetry is still in it.
    if [ $rc -ne 0 ] || ! grep -Eq "$marker" "{{log}}" \
        || { [ "{{lat}}:{{bw}}" != 0:0 ] && ! grep -q "\[ucache\] remote simulation" "{{log}}"; } \
        || grep -q "\[hdrkeep\]" "{{log}}" \
        || [ "$(gawk -f {{parse}}/osv_rows.awk "{{log}}" | wc -l)" -ne {{REPEAT}} ]; then
        echo FAIL; exit 1
    fi
    echo PASS

# One boot of VARIANT at the cache, for trying a cache size. Keeps the log in results/ and
# prints each repetition: a warm one reading 0 bytes means nothing was evicted.
probe:
    #!/usr/bin/env bash
    set -uo pipefail
    mkdir -p "{{results_dir}}"
    cache=$(just -f {{self}} _hdr_cache_bytes) || exit 1
    log="{{results_dir}}/hdr_probe_{{VARIANT}}_$cache.log"
    for q in {{QUERIES}}; do
        echo "==> [{{VARIANT}}] q$q cache=$((cache / 1048576)) MiB"
        just -f {{self}} _run_one {{VARIANT}} "$q" "$cache" "$log" {{LATENCY_US}} {{BANDWIDTH}}
        gawk -f {{parse}}/osv_rows.awk "$log" | gawk -F, \
            '{ printf "    rep %s  time %7.3f s  read %7.2f GiB  peak %6.2f GiB\n", $2, $3, $5 / 2^30, $7 / 2^30 }'
    done

# Every variant in VARIANTS at CACHE_PCT and every setting in LATENCIES, BOOTS times,
# interleaved per boot so a drift of the machine over time favours neither. Records the warm
# repetitions only.
run:
    #!/usr/bin/env bash
    set -uo pipefail
    trap 'echo "    !! interrupted - now: pkill qemu-system"; exit 130' INT
    RF="{{result_file}}"; mkdir -p "{{results_dir}}"
    just -f {{self}} _csv_header "$RF" "{{header}}"
    cache=$(just -f {{self}} _hdr_cache_bytes) || exit 1
    scale="{{SCALE}}"; [ -n "$scale" ] || scale=$(basename "{{DATA_DIR}}")
    log="{{results_dir}}/hdr_run.log"
    [ {{REPEAT}} -ge 2 ] || { echo "REPEAT must be >= 2: repetition 1 is the cold run" >&2; exit 1; }
    echo "==> [hdr] {{VARIANTS}} x latency {{LATENCIES}}, {{BOOTS}} boot(s), 1 cold + $(( {{REPEAT}} - 1 )) warm each of {{QUERIES}} (cache=$((cache / 1048576)) MiB = {{CACHE_PCT}}%)"
    for b in $(seq {{BOOTS}}); do
    for l in {{LATENCIES}}; do
    lat=${l%:*}; bw=${l#*:}
    for v in {{VARIANTS}}; do
        img=$(just -f {{self}} _image "$v") || exit 1
        if [ ! -f "$img/loader.img" ] || [ ! -f "$img/loader-stripped.elf" ]; then
            echo "    !! $v has no image in $img, run: just -f {{self}} build"; continue
        fi
        cfg="$v,$scale,{{CACHE_PCT}},$cache,{{THREADS}},{{EVICT_BATCH}},{{PREFETCH_BATCH}},$lat,$bw,$b"
        for q in {{QUERIES}}; do
            echo "==> [$v, latency $lat us, $bw GB/s, boot $b] q$q"
            st=$(just -f {{self}} _run_one "$v" "$q" "$cache" "$log" "$lat" "$bw")
            if [ "$st" != PASS ]; then
                echo "$cfg,$q,2,$st,na,na,na,na,na" >> "$RF"
                cp "$log" "{{results_dir}}/failed_hdr_${v}_l${lat}_q${q}_b$b.log"
                echo "    !! q$q $st, see failed_hdr_${v}_l${lat}_q${q}_b$b.log"
                continue
            fi
            # Warm repetitions only: field 2 of a parsed row is the repetition.
            gawk -f {{parse}}/osv_rows.awk "$log" | gawk -F, '$2 > 1' | sed "s/^/$cfg,/" >> "$RF"
        done
    done
    done
    done

# Per variant, latency setting and query: the mean of the warm repetitions.
summary:
    @gawk -F, 'NR > 1 && $12 > 1 && $13 + 0 > 0 { k = sprintf("%-10s lat %5s us %4s GB/s  q%s %s%%", $1, $8, $9, $11, $3); \
        w[k] += $13; n[k]++; r[k] += $15; p[k] += $17 } \
        END { for (k in w) printf "%s  warm %7.3f s  read %7.2f GiB  peak %6.2f GiB  (%d runs)\n", \
            k, w[k] / n[k], r[k] / n[k] / 2^30, p[k] / n[k] / 2^30, n[k] }' \
        {{result_file}} | sort

# Bars per variant and latency setting into results/hdr.pdf, and the same numbers on the
# console.
plot:
    cd {{proot}}/benchmarks/plots && python3 hdr.py

clean-results:
    rm -f {{result_file}} {{results_dir}}/hdr_run.log {{results_dir}}/hdr_probe_*.log {{results_dir}}/failed_hdr_*.log
