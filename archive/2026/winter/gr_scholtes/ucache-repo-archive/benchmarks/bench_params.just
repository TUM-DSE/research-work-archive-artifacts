# Parameters and helpers shared by the benchmark recipe files that must run under the same
# conditions (cachefs_compare.just, ablation.just). Imported, not run by itself; the importing
# file also imports ../justfile for proot.
# Knobs are NAME=value assignments set before the recipe name, or environment variables.
# Exported so an override reaches the nested `just` calls.

results_dir := proot + "/benchmarks/results"
parse := proot + "/benchmarks/parse"

export THREADS := env("THREADS", "16")
export REPEAT := env("REPEAT", "3")
export CACHE_PCT := env("CACHE_PCT", "10")
# 0 = off. 1400:0.7 = MinIO over 100 GbE (measured); 20000-40000 us ~= real S3.
export LATENCY_US := env("LATENCY_US", "0")
export BANDWIDTH := env("BANDWIDTH", "0")

export DDB_MEM := env("DDB_MEM", "64G")
# uCache only, in 4 KiB pages. EVICT_BATCH must not be below PREFETCH_BATCH: a prefetch
# burst gets one eviction round, so a smaller round lets the cache grow past its size.
export EVICT_BATCH := env("EVICT_BATCH", "2048")
export PREFETCH_BATCH := env("PREFETCH_BATCH", "2048")
# Both systems size their VM as cache + this, so the memory outside the cache is
# the same on each and constant as CACHE_PCT moves.
export VM_HEADROOM := env("VM_HEADROOM", "96G")
# CACHE_PCT's denominator. DATA_BYTES short-circuits DATA_DIR; SCALE labels the CSV
# rows so SF10 and SF100 never blend, and defaults to DATA_DIR's basename.
export DATA_DIR := env("DATA_DIR", "/scratch/ilya/tpch100")
export DATA_BYTES := env("DATA_BYTES", "")
export SCALE := env("SCALE", "")
# Per-query budget, covering all REPEAT repetitions of that one query.
export TIMEOUT_S := env("TIMEOUT_S", "1800")
# Skip a (system, scale, cache_pct, query) that already has a good row in the CSV.
export RESUME := env("RESUME", "true")
# Cache size as a % of the dataset, for `sweep`.
export CACHE_PCTS := env("CACHE_PCTS", "1 10 20 40 100")
# Host cores QEMU is taskset to (run-duckdb's cpuset). 80-95 = physical cores, no HT.
export CPUSET := env("CPUSET", "80-95")

# Total apparent bytes of the dataset, for turning CACHE_PCT into a byte budget.
# du -b matches the NVMe copy without unbinding the disk from vfio-pci.
_data_bytes:
    #!/usr/bin/env bash
    set -euo pipefail
    if [ -n "{{DATA_BYTES}}" ]; then echo "{{DATA_BYTES}}"; exit 0; fi
    if [ -z "{{DATA_DIR}}" ]; then
        echo "CACHE_PCT needs a dataset size: set DATA_DIR to the host folder holding" >&2
        echo "the TPC-H parquet, or set DATA_BYTES directly." >&2
        exit 1
    fi
    du -bc {{DATA_DIR}}/*.parquet | tail -1 | cut -f1

# CACHE_PCT of the dataset, in exact bytes: a sub-GiB cache truncated to "0G" reads
# back as unset, and the OSv app then falls back to half of RAM.
_cache_bytes:
    @echo $(( $(just -f {{justfile()}} _data_bytes) * {{CACHE_PCT}} / 100 ))

# VM size for either system: the cache, rounded up to whole GiB, plus VM_HEADROOM.
_vm_mem:
    #!/usr/bin/env bash
    set -euo pipefail
    cache=$(just -f {{justfile()}} _cache_bytes)
    cache_g=$(( (cache + 1073741823) / 1073741824 ))
    headroom="{{VM_HEADROOM}}"
    echo "$(( cache_g + ${headroom%[Gg]} ))G"

# Start a CSV, or move one written by an older harness schema aside: the header is only
# written when the file is absent, so appending to a stale one silently mixes schemas.
_csv_header file header:
    #!/usr/bin/env bash
    set -euo pipefail
    if [ -f "{{file}}" ] && [ "$(head -1 "{{file}}")" != "{{header}}" ]; then
        mv "{{file}}" "{{file}}.legacy-$(date +%Y%m%d%H%M%S)"
        echo "    note: CSV schema changed; previous rows kept as {{file}}.legacy-*"
    fi
    [ -f "{{file}}" ] || echo "{{header}}" > "{{file}}"
