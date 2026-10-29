set allow-duplicate-variables
import? "../justfile"
# REPEAT, DDB_MEM, TIMEOUT_S, ... and the cache/VM sizing helpers.
import "bench_params.just"

# The TLB shootdown IPIs alone: ablation step 6 (osv/benchmarks/duckdb/osv_ablation.hpp), once
# as shipped (noipi) and once with DUCKDB_NO_TLB_SHOOTDOWN forced to 0 (ipi), over vCPU counts.
# An IPI waits for every other vCPU, so its cost grows with the vCPU count and with the number
# of eviction rounds: a small cache and a small EVICT_BATCH make it visible.
# One fresh OSv boot per point, REPEAT repetitions inside it, BOOTS boots per point.
#   just -f benchmarks/ipi.just build
#   just -f benchmarks/ipi.just run
#   just -f benchmarks/ipi.just plot
#   just -f benchmarks/ipi.just run-tight plot-tight
#   just -f benchmarks/ipi.just run-headroom plot-headroom
#   just -f benchmarks/ipi.just run-pinned plot-pinned

self := proot + "/benchmarks/ipi.just"
images := proot + "/osv/build/ablation"

export VCPUS := env("VCPUS", "8 16 32 48 64")
export BOOTS := env("BOOTS", "3")
export QUERY := env("QUERY", "06")
export CACHE_PCT := env("CACHE_PCT", "2")
export EVICT_BATCH := env("EVICT_BATCH", "64")
# Not below EVICT_BATCH: one eviction round must cover a whole prefetch burst.
export PREFETCH_BATCH := env("PREFETCH_BATCH", "64")
# Spare host cores on top of the vCPUs for run-headroom.
export CPU_HEADROOM := env("CPU_HEADROOM", "32")
# run-pinned: vCPU k on host CPU VCPU_BASE+k, QEMU's other threads on QEMU_CPUS. On irene,
# 32-95 are physical cores clear of the ones other users pin to.
export PIN_VCPUS := env("PIN_VCPUS", "8 16 32 64")
export VCPU_BASE := env("VCPU_BASE", "32")
export QEMU_CPUS := env("QEMU_CPUS", "24-31")
# One cold repetition, then warm ones.
export PIN_REPEAT := env("PIN_REPEAT", "4")
# A guest that hits an assertion hangs instead of exiting.
export PIN_TIMEOUT_S := env("PIN_TIMEOUT_S", "300")
# Empty: boots 1..BOOTS.
export PIN_BOOTS := env("PIN_BOOTS", "")
export PIN_VARIANTS := env("PIN_VARIANTS", "noipi ipi")
export PIN_RESULT := env("PIN_RESULT", "ipi_pinned")

header := "variant,vcpus,cache_pct,evict_batch,prefetch_batch,boot,query,repetition,time,metadata_time,read_bytes,prefetch_used,peak_used,min_huge_blocks"
pinned_header := "variant,vcpus,vcpu_cpus,qemu_cpus,cache_pct,evict_batch,prefetch_batch,boot,query,repetition,time,metadata_time,read_bytes,prefetch_used,peak_used,min_huge_blocks"
taskset_header := "variant,vcpus,cpuset,cache_pct,evict_batch,prefetch_batch,boot,query,repetition,time,metadata_time,read_bytes,prefetch_used,peak_used,min_huge_blocks,voluntary_ctxt_switches,nonvoluntary_ctxt_switches"

default:
    @just -f {{self}} --list

# Both images: build/ablation/step6 and build/ablation/step6_ipi.
build:
    cd {{proot}}/osv && just build-duckdb release 1 0 6 0
    cd {{proot}}/osv && just build-duckdb release 1 0 6 1

# Both variants at every vCPU count in VCPUS, BOOTS times, interleaved so a drift of the
# machine over time favours neither. A failed run leaves a FAIL row and its log.
run: (_sweep "ipi.csv" "")

# As run, with QEMU pinned to host cores 0..vcpus-1.
run-tight: (_sweep "ipi_taskset_tight.csv" "0")

# As run, with QEMU pinned to vcpus + CPU_HEADROOM host cores.
run-headroom: (_sweep ("ipi_taskset_headroom" + CPU_HEADROOM + ".csv") CPU_HEADROOM)

# As run over PIN_VCPUS, each vCPU thread pinned to its own host CPU. A run not pinned
# before its first repetition fails.
run-pinned:
    #!/usr/bin/env bash
    set -uo pipefail
    taskset -pc "{{QEMU_CPUS}}" $$ >/dev/null || exit 1
    # VFIO locks all of the VM's memory.
    if [ "$(ulimit -l)" != unlimited ]; then
        sudo -n prlimit --memlock=unlimited:unlimited --pid $$ || exit 1
    fi
    max=0; for c in {{PIN_VCPUS}}; do [ $c -gt $max ] && max=$c; done
    if [ $(( {{VCPU_BASE}} + max )) -gt "$(nproc --all)" ]; then
        echo "VCPU_BASE={{VCPU_BASE}} + $max vCPUs is past the last host CPU" >&2; exit 1
    fi
    for r in $(echo "{{QEMU_CPUS}}" | tr , ' '); do
        for x in $(seq ${r%-*} ${r#*-}); do
            if [ $x -ge {{VCPU_BASE}} ] && [ $x -lt $(( {{VCPU_BASE}} + max )) ]; then
                echo "QEMU_CPUS={{QEMU_CPUS}} overlaps the vCPUs' CPUs" >&2; exit 1
            fi
        done
    done
    RF="{{results_dir}}/{{PIN_RESULT}}.csv"; mkdir -p "{{results_dir}}"
    just -f {{self}} _csv_header "$RF" "{{pinned_header}}"
    cache=$(just -f {{self}} _cache_bytes) || exit 1
    mem=$(just -f {{self}} _vm_mem) || exit 1
    log="{{results_dir}}/ipi_run.log"
    q="{{QUERY}}"
    vcpu_re='^CPU ([0-9]+)/KVM$'
    boots="{{PIN_BOOTS}}"; [ -n "$boots" ] || boots=$(seq {{BOOTS}})
    for b in $boots; do
    for c in {{PIN_VCPUS}}; do
    for v in {{PIN_VARIANTS}}; do
        [ $v = noipi ] && img={{images}}/step6 want=1 || img={{images}}/step6_ipi want=0
        qc="{{QEMU_CPUS}}"
        cfg="$v,$c,{{VCPU_BASE}}-$(( {{VCPU_BASE}} + c - 1 )),${qc//,/ },{{CACHE_PCT}},{{EVICT_BATCH}},{{PREFETCH_BATCH}},$b"
        while pgrep -u "$(id -u)" qemu-system >/dev/null; do sleep 5; done
        echo "==> [$v, vcpus=$c on {{VCPU_BASE}}-$(( {{VCPU_BASE}} + c - 1 )), boot $b] q{{QUERY}}"
        timeout -k 10 {{PIN_TIMEOUT_S}} env -u LD_LIBRARY_PATH bash -c \
            "cd {{proot}}/osv && just run-duckdb ${q#0} {{PIN_REPEAT}} $mem $c $cache {{DDB_MEM}} {{EVICT_BATCH}} {{PREFETCH_BATCH}} '' '' '' $img {{QEMU_CPUS}} '-name osv,debug-threads=on'" \
            > "$log" 2>&1 </dev/null &
        pid=$!
        qpid=""; ready=""
        while kill -0 $pid 2>/dev/null; do
            [ -n "$qpid" ] || qpid=$(pgrep -u "$(id -u)" -n qemu-system)
            pinned=0
            for t in ${qpid:+/proc/$qpid/task/*}; do
                read -r comm 2>/dev/null < $t/comm || continue
                if [[ $comm =~ $vcpu_re ]]; then
                    cpu=$(( {{VCPU_BASE}} + BASH_REMATCH[1] ))
                else
                    cpu="{{QEMU_CPUS}}"
                fi
                allowed=$(sed -n 's/^Cpus_allowed_list:\s*//p' $t/status 2>/dev/null)
                [ "$allowed" = "$cpu" ] || taskset -pc "$cpu" ${t##*/} >/dev/null 2>&1 || continue
                [[ $comm =~ $vcpu_re ]] && pinned=$(( pinned + 1 ))
            done
            if [ -z "$ready" ] && [ $pinned -eq $c ]; then
                grep -q "=== RUN 1/" "$log" && ready=late || ready=yes
                echo "    $pinned vCPU threads pinned ($ready)"
            fi
            sleep 0.5
        done
        wait $pid; rc=$?
        [ $rc -eq 124 ] || [ $rc -eq 137 ] && echo "    !! timed out after {{PIN_TIMEOUT_S}} s (hung guest?)"
        # The app prints the switches it was built with; a mismatch means a stale image.
        if [ "$ready" != yes ] \
            || ! grep -q "\[ablation\] step=6 .*no_tlb_shootdown=$want " "$log" \
            || [ "$(gawk -f {{parse}}/osv_rows.awk "$log" | wc -l)" -ne {{PIN_REPEAT}} ]; then
            echo "$cfg,{{QUERY}},1,FAIL,na,na,na,na,na" >> "$RF"
            cp "$log" "{{results_dir}}/failed_ipipin_${v}_v${c}_b$b.log"
            echo "    !! failed (vCPUs pinned: ${ready:-no}), see failed_ipipin_${v}_v${c}_b$b.log"
            continue
        fi
        gawk -f {{parse}}/osv_rows.awk "$log" | sed "s/^/$cfg,/" >> "$RF"
    done
    done
    done

# Empty cpu_headroom keeps run-duckdb's default pinning and records no context switches.
_sweep file cpu_headroom:
    #!/usr/bin/env bash
    set -uo pipefail
    RF="{{results_dir}}/{{file}}"; mkdir -p "{{results_dir}}"
    h="{{cpu_headroom}}"
    [ -z "$h" ] && hdr="{{header}}" || hdr="{{taskset_header}}"
    just -f {{self}} _csv_header "$RF" "$hdr"
    cache=$(just -f {{self}} _cache_bytes) || exit 1
    mem=$(just -f {{self}} _vm_mem) || exit 1
    log="{{results_dir}}/ipi_run.log"
    q="{{QUERY}}"
    for b in $(seq {{BOOTS}}); do
    for c in {{VCPUS}}; do
    for v in noipi ipi; do
        [ $v = noipi ] && img={{images}}/step6 want=1 || img={{images}}/step6_ipi want=0
        cpuset=""; [ -n "$h" ] && cpuset="0-$((c + h - 1))"
        cfg="$v,$c${cpuset:+,$cpuset},{{CACHE_PCT}},{{EVICT_BATCH}},{{PREFETCH_BATCH}},$b"
        # A timed-out QEMU can outlive `timeout` while it pins the VM's memory, and holds the
        # SSD and the image until it exits.
        while pgrep -u "$(id -u)" qemu-system >/dev/null; do sleep 5; done
        echo "==> [$v, vcpus=$c${cpuset:+, cpuset=$cpuset}, boot $b] q{{QUERY}}"
        # LD_LIBRARY_PATH from the nix shell can load a mismatched alsa-lib into QEMU.
        # </dev/null: QEMU under `timeout` stops on SIGTTIN when it touches the tty.
        timeout {{TIMEOUT_S}} env -u LD_LIBRARY_PATH bash -c \
            "cd {{proot}}/osv && just run-duckdb ${q#0} {{REPEAT}} $mem $c $cache {{DDB_MEM}} {{EVICT_BATCH}} {{PREFETCH_BATCH}} '' '' '' $img $cpuset" \
            > "$log" 2>&1 </dev/null &
        pid=$!
        # QEMU's context switches over all its threads. A read racing the exit can miss
        # threads, so only a larger sum counts.
        ctxt="0,0"
        while [ -n "$h" ] && kill -0 $pid 2>/dev/null; do
            qpid=$(pgrep -u "$(id -u)" -n qemu-system) \
                && s=$(cat /proc/$qpid/task/*/status 2>/dev/null | gawk \
                    '/^voluntary_ctxt_switches:/ { v += $2 } /^nonvoluntary_ctxt_switches:/ { n += $2 }
                     END { if (v + n) print v "," n }') \
                && [ -n "$s" ] && [ "${s%,*}" -ge "${ctxt%,*}" ] && ctxt="$s"
            sleep 1
        done
        wait $pid
        [ "$ctxt" = "0,0" ] && ctxt="na,na"
        tail=""; [ -n "$h" ] && tail=",$ctxt"
        # The app prints the switches it was built with; a mismatch means a stale image.
        if ! grep -q "\[ablation\] step=6 .*no_tlb_shootdown=$want " "$log" \
            || ! gawk -f {{parse}}/osv_rows.awk "$log" | grep -q .; then
            echo "$cfg,{{QUERY}},1,FAIL,na,na,na,na,na$tail" >> "$RF"
            cp "$log" "{{results_dir}}/failed_ipi_${v}_v${c}_b$b.log"
            echo "    !! failed, see failed_ipi_${v}_v${c}_b$b.log"
            continue
        fi
        gawk -f {{parse}}/osv_rows.awk "$log" | sed "s/^/$cfg,/; s/\$/$tail/" >> "$RF"
    done
    done
    done

# results/ipi.pdf, and the same numbers on the console.
plot:
    cd {{proot}}/benchmarks/plots && python3 ipi.py

# results/ipi_taskset_tight.pdf.
plot-tight:
    cd {{proot}}/benchmarks/plots && python3 ipi.py ipi_taskset_tight

# results/<PIN_RESULT>.pdf, warm repetitions only.
plot-pinned:
    cd {{proot}}/benchmarks/plots && python3 ipi.py {{PIN_RESULT}} --warm

# results/ipi_taskset_headroom<CPU_HEADROOM>.pdf.
plot-headroom:
    cd {{proot}}/benchmarks/plots && python3 ipi.py ipi_taskset_headroom{{CPU_HEADROOM}}
