import? "../justfile"
# THREADS, REPEAT, CACHE_PCT, LATENCY_US, ... and the cache/VM sizing helpers.
import "bench_params.just"

# Ablation of the DuckDB-on-uCache design: the same queries on one OSv build per step,
# under the parameters of cachefs_compare.just (bench_params.just).
# A step adds one concept to the steps before it; they are listed in
# osv/benchmarks/duckdb/osv_ablation.hpp.
# One fresh OSv boot per query, REPEAT repetitions inside it: repetition 1 is the cold
# run, the later ones are warm. Each point is booted BOOTS times, to average out the
# boot-to-boot noise; a boot gives only one cold run.
#   just -f benchmarks/ablation.just build
#   just -f benchmarks/ablation.just sweep
#   just -f benchmarks/ablation.just STEPS="1 6" QUERIES=06 CACHE_PCT=10 BOOTS=5 run
#   just -f benchmarks/ablation.just LATENCY_US=1400 BANDWIDTH=0.7 sweep
#   just -f benchmarks/ablation.just plot

self := proot + "/benchmarks/ablation.just"
result_file := results_dir + "/ablation.csv"
images := proot + "/osv/build/ablation"

export STEPS := env("STEPS", "1 2 3 4 5 6")
export BOOTS := env("BOOTS", "3")
# A few queries only, to keep the run time reasonable.
export QUERIES := env("QUERIES", "06 12 19")

header := "step,scale,cache_pct,cache_bytes,threads,evict_batch,prefetch_batch,sim_latency_us,sim_bandwidth_gbps,boot,query,repetition,time,metadata_time,read_bytes,prefetch_used,peak_used,min_huge_blocks"

default:
    @just -f {{self}} --list

# Build every step in STEPS. Each image is kept in osv/build/ablation/step<N>.
build:
    #!/usr/bin/env bash
    set -euo pipefail
    for s in {{STEPS}}; do
        echo "==> [ablation] building step $s"
        (cd {{proot}}/osv && just build-duckdb release 1 0 "$s")
    done

# The config prefix stamped on every row.
_cfg step boot cache_bytes:
    #!/usr/bin/env bash
    set -euo pipefail
    scale="{{SCALE}}"; [ -n "$scale" ] || scale=$(basename "{{DATA_DIR}}")
    echo "{{step}},$scale,{{CACHE_PCT}},{{cache_bytes}},{{THREADS}},{{EVICT_BATCH}},{{PREFETCH_BATCH}},{{LATENCY_US}},{{BANDWIDTH}},{{boot}}"

# Exit 0 if the CSV already holds a real timing for this (step, boot, scale, cache_pct, query)
# at the current threads, batch sizes and penalty. A TIMEOUT/FAIL/ERROR row parses as 0, so a failed point is
# retried; rows measured at another LATENCY_US/BANDWIDTH are a different point.
_done step boot query:
    #!/usr/bin/env bash
    set -euo pipefail
    [ -f "{{result_file}}" ] || exit 1
    scale="{{SCALE}}"; [ -n "$scale" ] || scale=$(basename "{{DATA_DIR}}")
    gawk -F, -v s="{{step}}" -v b="{{boot}}" -v sc="$scale" -v p="{{CACHE_PCT}}" -v q="{{query}}" \
        -v lat="{{LATENCY_US}}" -v bw="{{BANDWIDTH}}" \
        -v th="{{THREADS}}" -v eb="{{EVICT_BATCH}}" -v pb="{{PREFETCH_BATCH}}" \
        '$1==s && $2==sc && $3==p && $5==th && $6==eb && $7==pb && $8==lat && $9==bw && $10==b && $11==q && $13+0>0 { found=1 }
         END { exit !found }' "{{result_file}}"

# Every step in STEPS at the current CACHE_PCT, BOOTS times. The steps are interleaved
# within each round of boots, so a drift of the machine over time does not favour a step.
run:
    #!/usr/bin/env bash
    set -uo pipefail
    # Ctrl-C aborts the sweep; it reaches this shell only while it sits in `wait` below.
    trap 'echo "    !! interrupted - now: pkill qemu-system"; exit 130' INT
    RF="{{result_file}}"; mkdir -p "{{results_dir}}"
    just -f {{self}} _csv_header "$RF" "{{header}}"
    cache=$(just -f {{self}} _cache_bytes) || exit 1
    mem=$(just -f {{self}} _vm_mem) || exit 1

    echo "==> [ablation] steps {{STEPS}}, {{BOOTS}} boot(s), {{REPEAT}}x each of {{QUERIES}} (cache=$((cache / 1024 / 1024))MiB = {{CACHE_PCT}}%, mem=$mem)"
    for b in $(seq {{BOOTS}}); do
    for s in {{STEPS}}; do
        img="{{images}}/step$s"
        if [ ! -f "$img/loader.img" ] || [ ! -f "$img/loader-stripped.elf" ]; then
            echo "    !! step $s has no image in $img, run: just -f {{self}} STEPS=$s build"
            continue
        fi
        cfg=$(just -f {{self}} _cfg "$s" "$b" "$cache")
        for q in {{QUERIES}}; do
            if [ "{{RESUME}}" = "true" ] && just -f {{self}} _done "$s" "$b" "$q" 2>/dev/null; then
                echo "    -- step $s boot $b q$q already recorded, skipping"
                continue
            fi
            echo "==> [step $s, boot $b] q$q"
            # </dev/null is needed: `timeout` puts QEMU in its own process group, where
            # touching the controlling tty stops it on SIGTTIN. Log to a file, not a
            # variable, so a long query still prints and a Ctrl-C keeps what it wrote.
            log="{{results_dir}}/ablation_run.log"
            # Backgrounded and waited on: a foreground child holds the INT trap until it
            # returns, and this one never sees the Ctrl-C itself.
            timeout {{TIMEOUT_S}} bash -c \
                "cd {{proot}}/osv && just run-duckdb ${q#0} {{REPEAT}} $mem {{THREADS}} $cache {{DDB_MEM}} {{EVICT_BATCH}} {{PREFETCH_BATCH}} '' {{LATENCY_US}} {{BANDWIDTH}} $img {{CPUSET}}" \
                > "$log" 2>&1 </dev/null &
            wait $!
            rc=$?
            if [ $rc -ne 0 ]; then
                [ $rc -eq 124 ] && st=TIMEOUT || st=FAIL
                echo "$cfg,$q,1,$st,na,na,na,na,na" >> "$RF"
                cp "$log" "{{results_dir}}/failed_ablation_step${s}_q$q.log"
                echo "    !! q$q $st, see failed_ablation_step${s}_q$q.log"
                continue
            fi
            # The app prints the step it was built as; a mismatch means a stale image.
            if ! grep -q "\[ablation\] step=$s " "$log"; then
                echo "$cfg,$q,1,ERROR,na,na,na,na,na" >> "$RF"
                cp "$log" "{{results_dir}}/failed_ablation_step${s}_q$q.log"
                echo "    !! q$q: the image in $img is not a step $s build, rebuild it"
                continue
            fi
            # A run can also FAIL while exiting 0, in which case no @@ROW lines appear.
            n=$(gawk -f {{parse}}/osv_rows.awk "$log" \
                | gawk -F, -v pre="$cfg" '{ print pre "," $0 }' | tee -a "$RF" | wc -l)
            if [ "$n" -eq 0 ]; then
                echo "$cfg,$q,1,ERROR,na,na,na,na,na" >> "$RF"
                cp "$log" "{{results_dir}}/failed_ablation_step${s}_q$q.log"
                echo "    !! q$q produced no @@ROW lines, see failed_ablation_step${s}_q$q.log"
            elif [ "$n" -lt {{REPEAT}} ]; then
                echo "    !! q$q produced only $n of {{REPEAT}} repetitions"
            fi
            just check_downgraded_link || true
        done
    done
    done

# Every step in STEPS, over CACHE_PCTS.
sweep:
    #!/usr/bin/env bash
    set -euo pipefail
    for pct in {{CACHE_PCTS}}; do
        CACHE_PCT="$pct" just -f {{self}} run
    done

# Bars per step into results/ablation_cold.pdf and ablation_warm.pdf, and the same numbers
# on the console.
plot:
    cd {{proot}}/benchmarks/plots && python3 ablation.py

clean-results:
    rm -f {{result_file}} {{results_dir}}/ablation_run.log {{results_dir}}/failed_ablation_step*.log
