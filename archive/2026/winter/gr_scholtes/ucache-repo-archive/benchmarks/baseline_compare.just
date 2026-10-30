# CACHE_PCTS is redefined below, over the one in bench_params.just.
set allow-duplicate-variables

import? "../justfile"
# THREADS, REPEAT, CACHE_PCT, ... and the cache/VM sizing helpers.
import "bench_params.just"

# Original uCache vs this tree, on the same DuckDB TPC-H queries under the parameters of
# cachefs_compare.just (bench_params.just). Three systems, each its own OSv image:
#   ucache_upstream  the upstream tree at the commits pinned below, kept outside this
#                    repo in UPSTREAM_DIR. It carries patches/upstream_duckdb_app.diff,
#                    which only adds what the harness needs (untimed metadata warm-up,
#                    the @@ROW lines, a line naming the system).
#   ucache_base      this tree, ablation step 1
#   ucache           this tree, ablation step 6 (everything on)
# One fresh OSv boot per query and system, REPEAT repetitions inside it: repetition 1 is
# the cold run, the later ones are warm.
#   just -f benchmarks/baseline_compare.just build
#   just -f benchmarks/baseline_compare.just QUERIES=06 CACHE_PCTS=10 sweep
#   just -f benchmarks/baseline_compare.just sweep
#   just -f benchmarks/baseline_compare.just SYSTEMS="ucache_base ucache" CACHE_PCT=20 run
#   just -f benchmarks/baseline_compare.just plot

self := proot + "/benchmarks/baseline_compare.just"
result_file := results_dir + "/baseline_compare.csv"
provenance_file := results_dir + "/baseline_provenance.txt"
images := proot + "/osv/build/ablation"
patch := proot + "/benchmarks/patches/upstream_duckdb_app.diff"

export SYSTEMS := env("SYSTEMS", "ucache_upstream ucache_base ucache")
# TPC-H minus 15, as in cachefs_compare.just.
export QUERIES := env("QUERIES", "01 02 03 04 05 06 07 08 09 10 11 12 13 14 16 17 18 19 20 21 22")
export CACHE_PCTS := env("CACHE_PCTS", "5 10 20 50")

export UPSTREAM_DIR := env("UPSTREAM_DIR", "/scratch/" + user + "/ucache-baseline/osv")
UPSTREAM_URL := "https://github.com/Meandres/osv.git"
UPSTREAM_BRANCH := "kite"
# osv, its benchmarks submodule, and the duckdb submodule of that.
UPSTREAM_OSV_REV := "eb8a6f66ede6d1548b10eb14a7cec6e9fdb66660"
UPSTREAM_BENCH_REV := "9142a6df963f33612a0d731e6d16004d9493d289"
UPSTREAM_DUCKDB_REV := "d5a1b699e90f0f43c7826113149765b9d8650709"

# The columns of cachefs_compare.csv, so the two files can be concatenated.
# block_size/policy/scope/chunking are cache_httpfs knobs and "na" here.
header := "system,scale,cache_pct,cache_bytes,threads,block_size,evict_batch,prefetch_batch,policy,scope,chunking,sim_latency_us,sim_bandwidth_gbps,sim_unit,query,repetition,time,metadata_time,read_bytes,prefetch_used,peak_used,min_huge_blocks"

default:
    @just -f {{self}} --list

# Clone the upstream tree into UPSTREAM_DIR at the pinned commits, patch its app for the
# harness and build it. Safe to run again: it only does what is still missing.
build-upstream:
    #!/usr/bin/env bash
    set -euo pipefail
    up="{{UPSTREAM_DIR}}"
    # The submodule URLs are ssh ones; fetch them over https instead.
    https=(-c "url.https://github.com/.insteadOf=git@github.com:")
    if [ ! -d "$up/.git" ]; then
        echo "==> [baseline] cloning {{UPSTREAM_URL}} ({{UPSTREAM_BRANCH}}) into $up"
        mkdir -p "$(dirname "$up")"
        git clone -b {{UPSTREAM_BRANCH}} {{UPSTREAM_URL}} "$up"
    fi
    if [ "$(git -C "$up" rev-parse HEAD)" != "{{UPSTREAM_OSV_REV}}" ]; then
        git -C "$up" checkout --detach {{UPSTREAM_OSV_REV}}
    fi
    git -C "$up" "${https[@]}" submodule update --init
    git -C "$up/benchmarks" "${https[@]}" submodule update --init duckdb/duckdb
    for pin in ".:{{UPSTREAM_OSV_REV}}" "benchmarks:{{UPSTREAM_BENCH_REV}}" "benchmarks/duckdb/duckdb:{{UPSTREAM_DUCKDB_REV}}"; do
        dir="$up/${pin%%:*}"; rev="${pin#*:}"
        if [ "$(git -C "$dir" rev-parse HEAD)" != "$rev" ]; then
            echo "    !! $dir is at $(git -C "$dir" rev-parse HEAD), expected $rev"
            exit 1
        fi
    done
    if git -C "$up/benchmarks" apply --reverse --check "{{patch}}" 2>/dev/null; then
        echo "    -- harness patch already applied"
    else
        echo "==> [baseline] applying $(basename "{{patch}}")"
        git -C "$up/benchmarks" apply "{{patch}}"
    fi
    echo "==> [baseline] building ucache_upstream"
    (cd "$up" && just build-duckdb)

# Build the three images: ablation steps 1 and 6 of this tree, and the upstream tree.
build:
    just -f {{proot}}/benchmarks/ablation.just STEPS="1 6" build
    just -f {{self}} build-upstream

# The folder holding loader.img and loader-stripped.elf of a system.
_image system:
    #!/usr/bin/env bash
    set -euo pipefail
    case "{{system}}" in
        ucache_upstream) echo "{{UPSTREAM_DIR}}/build/release.x64" ;;
        ucache_base)     echo "{{images}}/step1" ;;
        ucache)          echo "{{images}}/step6" ;;
        *) echo "unknown system {{system}}: ucache_upstream, ucache_base or ucache" >&2; exit 1 ;;
    esac

# What a system's app prints at boot; a log without it came from another image.
_marker system:
    #!/usr/bin/env bash
    set -euo pipefail
    case "{{system}}" in
        ucache_upstream) echo "[system] ucache_upstream" ;;
        ucache_base)     echo "[ablation] step=1 " ;;
        ucache)          echo "[ablation] step=6 " ;;
        *) echo "unknown system {{system}}" >&2; exit 1 ;;
    esac

# The config prefix stamped on every row. The simulated object-store cost is always off:
# the upstream tree has none.
_cfg system cache_bytes:
    #!/usr/bin/env bash
    set -euo pipefail
    scale="{{SCALE}}"; [ -n "$scale" ] || scale=$(basename "{{DATA_DIR}}")
    echo "{{system}},$scale,{{CACHE_PCT}},{{cache_bytes}},{{THREADS}},na,{{EVICT_BATCH}},{{PREFETCH_BATCH}},na,na,na,0,0,page+batch"

# Exit 0 if the CSV already holds a real timing for this (system, scale, cache_pct, query)
# at the current threads and batch sizes. A TIMEOUT/FAIL/ERROR row parses as 0, so a
# failed point is retried.
_done system query:
    #!/usr/bin/env bash
    set -euo pipefail
    [ -f "{{result_file}}" ] || exit 1
    scale="{{SCALE}}"; [ -n "$scale" ] || scale=$(basename "{{DATA_DIR}}")
    gawk -F, -v s="{{system}}" -v sc="$scale" -v p="{{CACHE_PCT}}" -v q="{{query}}" \
        -v th="{{THREADS}}" -v eb="{{EVICT_BATCH}}" -v pb="{{PREFETCH_BATCH}}" \
        '$1==s && $2==sc && $3==p && $5==th && $7==eb && $8==pb && $15==q && $17+0>0 { found=1 }
         END { exit !found }' "{{result_file}}"

# Which images and commits a run measured, appended once per `run`.
_provenance:
    #!/usr/bin/env bash
    set -uo pipefail
    {
        echo "== $(date -Is) cache_pct={{CACHE_PCT}} queries=\"{{QUERIES}}\""
        echo "threads={{THREADS}} repeat={{REPEAT}} ddb_mem={{DDB_MEM}} evict_batch={{EVICT_BATCH}} prefetch_batch={{PREFETCH_BATCH}} vm_headroom={{VM_HEADROOM}} cpuset={{CPUSET}} data_dir={{DATA_DIR}}"
        echo "this tree: $(git -C {{proot}} rev-parse HEAD 2>/dev/null || echo 'not a git checkout')"
        echo "upstream:  osv={{UPSTREAM_OSV_REV}} benchmarks={{UPSTREAM_BENCH_REV}} duckdb={{UPSTREAM_DUCKDB_REV}}"
        for s in {{SYSTEMS}}; do
            img=$(just -f {{self}} _image "$s") || continue
            for f in loader.img loader-stripped.elf; do
                if [ -f "$img/$f" ]; then
                    echo "$s $f sha256=$(sha256sum "$img/$f" | cut -d' ' -f1) mtime=$(date -Is -r "$img/$f")"
                else
                    echo "$s $f missing"
                fi
            done
        done
    } >> "{{provenance_file}}"

# Every system in SYSTEMS at the current CACHE_PCT. The systems are interleaved within
# each query, so a drift of the machine over time does not favour one of them.
run:
    #!/usr/bin/env bash
    set -uo pipefail
    # Ctrl-C aborts the sweep; it reaches this shell only while it sits in `wait` below.
    trap 'echo "    !! interrupted - now: pkill qemu-system"; exit 130' INT
    RF="{{result_file}}"; mkdir -p "{{results_dir}}"
    just -f {{self}} _csv_header "$RF" "{{header}}"
    cache=$(just -f {{self}} _cache_bytes) || exit 1
    mem=$(just -f {{self}} _vm_mem) || exit 1

    # Only the systems that have an image.
    systems=""
    for s in {{SYSTEMS}}; do
        img=$(just -f {{self}} _image "$s") || exit 1
        if [ ! -f "$img/loader.img" ] || [ ! -f "$img/loader-stripped.elf" ]; then
            echo "    !! $s has no image in $img, run: just -f {{self}} build"
            continue
        fi
        systems="$systems $s"
    done
    [ -n "$systems" ] || exit 1
    just -f {{self}} _provenance

    echo "==> [baseline]$systems, {{REPEAT}}x each of {{QUERIES}} (cache=$((cache / 1024 / 1024))MiB = {{CACHE_PCT}}%, mem=$mem)"
    for q in {{QUERIES}}; do
    for s in $systems; do
        if [ "{{RESUME}}" = "true" ] && just -f {{self}} _done "$s" "$q" 2>/dev/null; then
            echo "    -- $s q$q already recorded, skipping"
            continue
        fi
        img=$(just -f {{self}} _image "$s")
        marker=$(just -f {{self}} _marker "$s")
        cfg=$(just -f {{self}} _cfg "$s" "$cache")
        echo "==> [$s] q$q"
        # </dev/null is needed: `timeout` puts QEMU in its own process group, where
        # touching the controlling tty stops it on SIGTTIN. Log to a file, not a
        # variable, so a long query still prints and a Ctrl-C keeps what it wrote.
        log="{{results_dir}}/baseline_run.log"
        failed="failed_baseline_${s}_q$q.log"
        # Every system is booted by this tree's run-duckdb, given its image.
        # Backgrounded and waited on: a foreground child holds the INT trap until it
        # returns, and this one never sees the Ctrl-C itself.
        timeout {{TIMEOUT_S}} bash -c \
            "cd {{proot}}/osv && just run-duckdb ${q#0} {{REPEAT}} $mem {{THREADS}} $cache {{DDB_MEM}} {{EVICT_BATCH}} {{PREFETCH_BATCH}} '' 0 0 $img {{CPUSET}}" \
            > "$log" 2>&1 </dev/null &
        wait $!
        rc=$?
        if [ $rc -ne 0 ]; then
            [ $rc -eq 124 ] && st=TIMEOUT || st=FAIL
            echo "$cfg,$q,1,$st,na,na,na,na,na" >> "$RF"
            cp "$log" "{{results_dir}}/$failed"
            echo "    !! q$q $st, see $failed"
            continue
        fi
        if ! grep -qF -- "$marker" "$log"; then
            echo "$cfg,$q,1,ERROR,na,na,na,na,na" >> "$RF"
            cp "$log" "{{results_dir}}/$failed"
            echo "    !! q$q: the image in $img is not a $s build, rebuild it"
            continue
        fi
        # A run can also FAIL while exiting 0, in which case no @@ROW lines appear.
        n=$(gawk -f {{parse}}/osv_rows.awk "$log" \
            | gawk -F, -v pre="$cfg" '{ print pre "," $0 }' | tee -a "$RF" | wc -l)
        if [ "$n" -eq 0 ]; then
            echo "$cfg,$q,1,ERROR,na,na,na,na,na" >> "$RF"
            cp "$log" "{{results_dir}}/$failed"
            echo "    !! q$q produced no @@ROW lines, see $failed"
        elif [ "$n" -lt {{REPEAT}} ]; then
            echo "    !! q$q produced only $n of {{REPEAT}} repetitions"
        fi
        just check_downgraded_link || true
    done
    done

# Every system in SYSTEMS, over CACHE_PCTS.
sweep:
    #!/usr/bin/env bash
    set -euo pipefail
    for pct in {{CACHE_PCTS}}; do
        CACHE_PCT="$pct" just -f {{self}} run
    done

# Per-query bars into results/baseline_compare.pdf, totals against cache size into
# baseline_sweep.pdf, and the totals on the console.
plot:
    cd {{proot}}/benchmarks/plots && python3 baseline_compare.py

clean-results:
    rm -f {{result_file}} {{provenance_file}} {{results_dir}}/baseline_run.log {{results_dir}}/failed_baseline_*.log
