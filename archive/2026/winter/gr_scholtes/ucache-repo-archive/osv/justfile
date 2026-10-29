osv_dir := justfile_directory()
ssd_id := "c3:00.0"


build-kernel mode="release" app="benchmarks/example/example.o":
    #!/usr/bin/env bash
    set -euo pipefail
    cd {{osv_dir}}
    make -j$(nproc) mode={{mode}} APP_OBJECTS="{{app}}" bootfs_manifest=bootfs_empty.manifest.skel

# Boot OSv with the NVMe SSD (ssd_id) via PCI passthrough (VFIO).
# Uses PVH kernel mode (-k) to bypass BIOS INT 13h disk reads; without this
# SeaBIOS may assign the VFIO NVMe as drive 0x80 and boot16.S reads from the
# wrong disk (it hardcodes dl=0x80 instead of using the BIOS-provided drive number).
run:
    #!/usr/bin/env bash
    set -euo pipefail
    cd {{osv_dir}}
    ./scripts/run.py -k -i build/last/loader.img --pass-pci "0000:{{ssd_id}}"

# ── DuckDB TPC-H runner ───────────────────────────────────────────────────────

# Run the DuckDB TPC-H benchmark on OSv with the NVMe SSD (ssd_id).
# Parameters are forwarded as OSv environment variables (--env=KEY=VALUE).
#
# Examples:
#   just run-duckdb query=1                          # Q1, one pass
#   just run-duckdb query=6 repeat=3                 # Q6, three passes (warm cache)
#   just run-duckdb query=all                        # all 22 queries
#   just run-duckdb mem=16G cache=8G duckdb=6G query=1
#   just run-duckdb mem=32G cache=24G duckdb=20G query=all
#
# mem    — total physical memory given to the QEMU VM (default 8G)
# cache  — bytes for OSv uCache; K/M/G suffix accepted (default: 50% of mem)
# duckdb — DuckDB buffer pool limit; K/M/G suffix accepted (default: 80% of cache)
# image_dir — folder holding loader.img and loader-stripped.elf (default: the last build;
#             build/ablation/step<N> for an ablation step kept by build-duckdb)
# cpuset — host CPUs QEMU is pinned to (taskset -c list, default 0-63)
# qemu_args — extra arguments for QEMU itself, e.g. "-name osv,debug-threads=on"
run-duckdb query="1" repeat="1" mem="8G" vcpus="4" cache="" duckdb="" evict_batch="" prefetch_batch="" print="" sim_latency_us="" sim_bw_gbps="" image_dir="build/last" cpuset="0-63" qemu_args="":
    #!/usr/bin/env bash
    set -euo pipefail
    cd {{osv_dir}}

    # Ensure the SSD is bound to the vfio kernel driver.
    current_driver=$(sudo driverctl list-devices | grep "{{ssd_id}}" | awk '{print $2}')
    if [ "${current_driver}" != "vfio-pci" ]; then
        echo "==> Binding 0000:{{ssd_id}} to vfio-pci driver ..."
        sudo driverctl set-override "0000:{{ssd_id}}" vfio-pci
        sleep 1
    fi

    osv_env="--mount-nvme-ext --env=TPCH_QUERY={{query}} --env=TPCH_REPEAT={{repeat}}"
    if [ -n "{{cache}}" ]; then
        osv_env="${osv_env} --env=UCACHE_MEM={{cache}}"
    fi
    if [ -n "{{duckdb}}" ]; then
        osv_env="${osv_env} --env=DUCKDB_MEM={{duckdb}}"
    fi
    if [ -n "{{evict_batch}}" ]; then
        osv_env="${osv_env} --env=UCACHE_EVICT_BATCH={{evict_batch}}"
    fi
    if [ -n "{{prefetch_batch}}" ]; then
        osv_env="${osv_env} --env=UCACHE_PREFETCH_BATCH={{prefetch_batch}}"
    fi
    if [ -n "{{print}}" ]; then
        osv_env="${osv_env} --env=TPCH_PRINT=1"
    fi

    # Simulated object-store cost, matching cache_httpfs's knobs. Unset = off.
    if [ -n "{{sim_latency_us}}" ]; then
        osv_env="${osv_env} --env=UCACHE_SIM_LATENCY_US={{sim_latency_us}}"
    fi
    if [ -n "{{sim_bw_gbps}}" ]; then
        osv_env="${osv_env} --env=UCACHE_SIM_BW_GBPS={{sim_bw_gbps}}"
    fi
    # --pass-args=: with a space, argparse takes a value starting with "-" for an option.
    pass_args=()
    if [ -n "{{qemu_args}}" ]; then
        pass_args=("--pass-args={{qemu_args}}")
    fi

    # -k: PVH kernel mode — QEMU loads loader-stripped.elf directly, bypassing
    # BIOS INT 13h.  Without this, SeaBIOS may give the VFIO NVMe drive 0x80
    # and boot16.S (which hardcodes dl=0x80) reads from the wrong disk.
    # -i loader.img: still needed so run.py doesn't fall back to usr.img (which
    # we don't build); the file is presented as a virtio-blk but not used for boot.
    taskset -c "{{cpuset}}" ./scripts/run.py -k -i "{{image_dir}}/loader.img" \
        --kernel-path "{{image_dir}}/loader-stripped.elf" \
        -m "{{mem}}" -c "{{vcpus}}" \
        -e "${osv_env}" \
        --pass-pci "0000:{{ssd_id}}" "${pass_args[@]}"

# Sample one TPC-H repetition with OSv's sampler and pull the profile over the gdb stub.
# hz x seconds sampled must stay under ~7.7k samples per CPU (1 MiB trace ring); 100 Hz covers ~75 s.
profile-duckdb query="1" repeat="1" mem="8G" vcpus="4" cache="" duckdb="" evict_batch="" prefetch_batch="" hz="100" rep="" out="traces.bin" wait_s="1800" cpuset="0-63" image_dir="build/last":
    #!/usr/bin/env bash
    set -euo pipefail
    cd {{osv_dir}}
    # gdb auto-loads scripts/loader.py via the ELF's .debug_gdb_scripts section, but
    # doesn't add that script's own directory to sys.path the way `python3 script.py`
    # does, so its `from osv.trace import ...` fails unless scripts/ is already on
    # PYTHONPATH.
    export PYTHONPATH="{{osv_dir}}/scripts${PYTHONPATH:+:$PYTHONPATH}"

    current_driver=$(sudo driverctl list-devices | grep "{{ssd_id}}" | awk '{print $2}')
    if [ "${current_driver}" != "vfio-pci" ]; then
        echo "==> Binding 0000:{{ssd_id}} to vfio-pci driver ..."
        sudo driverctl set-override "0000:{{ssd_id}}" vfio-pci
        sleep 1
    fi

    osv_env="--noshutdown --mount-nvme-ext --env=TPCH_QUERY={{query}} --env=TPCH_REPEAT={{repeat}}"
    osv_env="${osv_env} --env=TPCH_SAMPLE_HZ={{hz}}"
    if [ -n "{{rep}}" ]; then
        osv_env="${osv_env} --env=TPCH_SAMPLE_REP={{rep}}"
    fi
    if [ -n "{{cache}}" ]; then
        osv_env="${osv_env} --env=UCACHE_MEM={{cache}}"
    fi
    if [ -n "{{duckdb}}" ]; then
        osv_env="${osv_env} --env=DUCKDB_MEM={{duckdb}}"
    fi
    if [ -n "{{evict_batch}}" ]; then
        osv_env="${osv_env} --env=UCACHE_EVICT_BATCH={{evict_batch}}"
    fi
    if [ -n "{{prefetch_batch}}" ]; then
        osv_env="${osv_env} --env=UCACHE_PREFETCH_BATCH={{prefetch_batch}}"
    fi

    out="{{osv_dir}}/../{{out}}"
    log="${out%.bin}.log"

    # </dev/null: a backgrounded QEMU that touches the tty stops dead on SIGTTIN/SIGTTOU.
    taskset -c "{{cpuset}}" ./scripts/run.py -k -i "{{image_dir}}/loader.img" \
        --kernel-path "{{image_dir}}/loader-stripped.elf" \
        -m "{{mem}}" -c "{{vcpus}}" \
        -H -e "${osv_env}" \
        --pass-pci "0000:{{ssd_id}}" > "${log}" 2>&1 < /dev/null &
    vm=$!
    # run.py does not forward signals to its QEMU child, so match on the image instead.
    trap 'kill ${vm} 2>/dev/null || true; pkill -f "qemu-system-x86_64.*loader.img" 2>/dev/null || true' EXIT

    echo "==> booting, sampling at {{hz}} Hz; waiting for the window to close (log: ${log})"
    for _ in $(seq {{wait_s}}); do
        grep -q "\[sampler\] window closed" "${log}" && break
        if ! kill -0 ${vm} 2>/dev/null; then break; fi
        sleep 1
    done
    if ! grep -q "\[sampler\] window closed" "${log}"; then
        echo "!! the sampled repetition never finished — see ${log}" >&2
        exit 1
    fi

    # The ring is frozen, so this is not a race; `monitor quit` then shuts the VM down.
    tail -n 3 "${log}"
    python3 scripts/trace.py extract -e "{{image_dir}}/loader.elf" -r localhost:1234 "${out}"
    python3 scripts/trace.py prof -e "{{image_dir}}/loader.elf" -S "${out}" > "${out%.bin}.prof"
    gdb "{{image_dir}}/loader.elf" -batch -ex 'target remote localhost:1234' -ex 'monitor quit' >/dev/null 2>&1 || true
    echo "==> ${out}  +  ${out%.bin}.prof"
    head -n 40 "${out%.bin}.prof"

# Build DuckDB as a static archive with LTO, then link it into the kernel.
#
# DuckDB is compiled with -flto -ffat-lto-objects so ld.bfd + the GCC LTO
# plugin can do a whole-program optimisation pass across DuckDB and the kernel.
#
# The app wrapper (duckdb_app.cc) is compiled by the OSv Makefile using the
# kernel's own CXXFLAGS, with the DuckDB include path appended via EXTRA_CXXFLAGS.
#
# Usage:
#   just build-duckdb              # release build with LTO
#   just build-duckdb release 0    # disable LTO (faster rebuild for debugging)
#   just build-duckdb release 1 1  # keep frame pointers (profilable DuckDB frames)
#   just build-duckdb release 1 0 2  # ablation step 2 (steps: benchmarks/duckdb/osv_ablation.hpp)
#   just build-duckdb release 1 0 6 1  # step 6 with the TLB shootdown IPIs back on
#   just build-duckdb release 1 0 6 0 5  # step 6 keeping the first 5 frames of each page resident
#
# Parameters are positional (mode lto fp step ipi hdr) - `lto=0` is read as mode, not as lto.
# The image of each build is also kept in build/ablation/step<N> (suffixed _ipi with ipi=1,
# _hdr<hdr> with hdr>0), for run-duckdb's image_dir.
#
build-duckdb mode="release" lto="1" fp="0" step="6" ipi="0" hdr="0":
    #!/usr/bin/env bash
    set -euo pipefail
    cd {{osv_dir}}

    duckdb_src="{{osv_dir}}/benchmarks/duckdb/duckdb"
    duckdb_build="{{osv_dir}}/build/{{mode}}.x64/duckdb"
    duckdb_lib="${duckdb_build}/src/libduckdb_static.a"

    lto_flags=""
    if [ "{{lto}}" = "1" ]; then
        lto_flags="-flto -ffat-lto-objects"
    fi
    if [ "{{fp}}" = "1" ]; then
        lto_flags="${lto_flags} -fno-omit-frame-pointer"
    fi

    # The ablation step is read from a generated header, by DuckDB's parquet extension and by
    # the app objects. Only rewritten on a change, so the same step again rebuilds nothing.
    variant_hdr="{{osv_dir}}/benchmarks/duckdb/osv_ablation_variant.hpp"
    variant_def="#define DUCKDB_ABLATION_STEP {{step}}"
    image_dir="build/ablation/step{{step}}"
    # ipi=1 overrides the step's default, to measure the shootdown alone.
    if [ "{{ipi}}" = "1" ]; then
        variant_def="${variant_def}"$'\n'"#define DUCKDB_NO_TLB_SHOOTDOWN 0"
        image_dir="${image_dir}_ipi"
    fi
    # hdr=N keeps the first N frames of each page resident, the first holding its header, see
    # osv_ablation.hpp.
    if [ "{{hdr}}" != "0" ]; then
        variant_def="${variant_def}"$'\n'"#define DUCKDB_KEEP_HEADER_FRAMES {{hdr}}"
        image_dir="${image_dir}_hdr{{hdr}}"
    fi
    if [ "$(cat "${variant_hdr}" 2>/dev/null)" != "${variant_def}" ]; then
        echo "${variant_def}" > "${variant_hdr}"
    fi

    # --- Step 1: build DuckDB static library ---
    osv_bench_inc="{{osv_dir}}/benchmarks/duckdb"
    cmake -S "${duckdb_src}" \
          -B "${duckdb_build}" \
          -DCMAKE_BUILD_TYPE=Release \
          -DCMAKE_C_COMPILER="${CC:-x86_64-linux-gnu-gcc}" \
          -DCMAKE_CXX_COMPILER="${CXX:-x86_64-linux-gnu-g++}" \
          -DCMAKE_C_FLAGS="-D__OSV__=1 -I${osv_bench_inc} -fno-pie -fno-stack-protector -mfma -ftls-model=local-exec ${lto_flags}" \
          -DCMAKE_CXX_FLAGS="-D__OSV__=1 -I${osv_bench_inc} -fno-pie -fno-stack-protector -mfma -ftls-model=local-exec ${lto_flags}" \
          -DBUILD_SHARED_LIBS=OFF \
          -DBUILD_SHELL=OFF \
          -DBUILD_UNITTESTS=OFF \
          -DDUCKDB_EXPLICIT_PLATFORM=linux_amd64 \
          -DENABLE_SANITIZER=OFF \
          -DENABLE_UBSAN=OFF \
          -DMUSL_ENABLED=1 \
          -DCMAKE_EXPORT_COMPILE_COMMANDS=ON

    cmake --build "${duckdb_build}" \
        --target duckdb_static core_functions_extension parquet_extension duckdb_generated_extension_loader \
        --parallel $(nproc)

    # --- Step 2: build ext filesystem modules (lwext4 + libext) ---
    # lwext4 clones upstream on first run; subsequent runs are fast (already built).
    make -C modules/lwext4
    make mode={{mode}} generated-headers
    make -C modules/libext module ARCH=x64 OSV_BUILD_PATH="{{osv_dir}}/build/release.x64"

    # --- Step 3: build kernel with DuckDB and OSv uCache file system baked in ---
    core_fn_lib="${duckdb_build}/extension/core_functions/libcore_functions_extension.a"
    parquet_lib="${duckdb_build}/extension/parquet/libparquet_extension.a"
    ext_loader_lib="${duckdb_build}/extension/libduckdb_generated_extension_loader.a"

    # Rebuild the objects that read the ablation step.
    rm -f build/{{mode}}.x64/benchmarks/duckdb/osv_ucache_file_system.o \
          build/{{mode}}.x64/benchmarks/duckdb/osv_page_resident_set.o

    make -j$(nproc) \
        mode={{mode}} \
        lto={{lto}} \
        fs=ext \
        APP_OBJECTS="benchmarks/duckdb/duckdb_app.o benchmarks/duckdb/osv_ucache_file_system.o benchmarks/duckdb/osv_page_resident_set.o" \
        APP_LIBS="${duckdb_lib} ${core_fn_lib} ${parquet_lib} ${ext_loader_lib}" \
        EXTRA_CXXFLAGS="-I${duckdb_src}/src/include -I${duckdb_src}/third_party/concurrentqueue -I${duckdb_src}/extension/parquet/include -I${duckdb_src}/third_party/thrift" \
        app_local_exec_tls_size=2048 \
        bootfs_manifest=bootfs_ext.manifest.skel

    # Keep this step's image, so several steps can be run without rebuilding.
    # loader.elf (unstripped) is kept too, for profile-duckdb's symbol resolution.
    mkdir -p "${image_dir}"
    cp build/{{mode}}.x64/loader.img build/{{mode}}.x64/loader-stripped.elf \
       build/{{mode}}.x64/loader.elf "${image_dir}/"

bind-ssd-vfio:
    #!/usr/bin/env bash
    current_driver=$(sudo driverctl list-devices | grep {{ssd_id}} | awk '{print $2 }')
    [ $current_driver = "vfio-pci" ] || sudo driverctl set-override 0000:{{ssd_id}} vfio-pci

bind-ssd-nvme:
    #!/usr/bin/env bash
    current_driver=$(sudo driverctl list-devices | grep {{ssd_id}} | awk '{print $2 }')
    if [ "$current_driver" != "nvme" ]
    then
        sudo driverctl set-override 0000:{{ssd_id}} nvme
    fi

# Run the DuckDB TPC-H benchmark directly on Linux (no VM).
# Ensures the SSD is bound to the nvme kernel driver and mounted at /nvme,
# then launches benchmarks/duckdb/duckdb_bench with the same env-var knobs
# as run-duckdb (OSv).
#
# Examples:
#   just run-duckdb-linux query=1
#   just run-duckdb-linux query=6 repeat=3 file_cache=4G threads=8
#   just run-duckdb-linux query=all duckdb=20G file_cache=8G threads=32
#
# threads    — total thread count including the main thread (default: nproc)
#              DuckDB worker threads = threads-1, main thread pinned to CPU threads-1
# duckdb     — DuckDB buffer pool limit; K/M/G suffix accepted (default: 40% of RAM)
# file_cache — DuckDB CachingFileSystem cap; K/M/G suffix (default: enabled, no cap)
#              pass 0 or "off" to disable
run-duckdb-linux query="1" repeat="1" threads="" file_cache="" duckdb="":
    #!/usr/bin/env bash
    set -euo pipefail

    bench_bin="{{osv_dir}}/benchmarks/duckdb/duckdb_bench"
    if [ ! -x "${bench_bin}" ]; then
        echo "Error: ${bench_bin} not found — run 'make linux' in benchmarks/duckdb/ first" >&2
        exit 1
    fi

    # Ensure the SSD is bound to the nvme kernel driver.
    current_driver=$(sudo driverctl list-devices | grep "{{ssd_id}}" | awk '{print $2}')
    if [ "${current_driver}" != "nvme" ]; then
        echo "==> Binding 0000:{{ssd_id}} to nvme driver ..."
        sudo driverctl set-override "0000:{{ssd_id}}" nvme
        sleep 1
    fi

    # Discover the block device exposed by the NVMe controller.
    nvme_dev="/dev/$(ls /sys/bus/pci/devices/0000:{{ssd_id}}/nvme)n1"
    echo "==> NVMe block device: ${nvme_dev}"

    # Mount at /nvme if not already mounted.
    if ! mountpoint -q /nvme; then
        sudo mkdir -p /nvme
        sudo mount -o ro "${nvme_dev}" /nvme
        echo "==> Mounted ${nvme_dev} at /nvme (read-only)"
    else
        echo "==> /nvme already mounted"
    fi

    # Build the env for duckdb_bench.
    export TPCH_QUERY="{{query}}"
    export TPCH_REPEAT="{{repeat}}"
    if [ -n "{{duckdb}}" ]; then
        export DUCKDB_MEM="{{duckdb}}"
    fi
    if [ -n "{{file_cache}}" ]; then
        export DUCKDB_FILE_CACHE="{{file_cache}}"
    fi

    nthreads="{{threads}}"
    if [ -z "${nthreads}" ]; then
        nthreads="$(nproc)"
    fi

    echo "==> Running DuckDB TPC-H (TPCH_QUERY={{query}} TPCH_REPEAT={{repeat}} threads=${nthreads})"
    taskset -c 0-63 "${bench_bin}" "${nthreads}"

# ── NVMe setup ────────────────────────────────────────────────────────────────

# Format the NVMe SSD (ssd_id) as ext4, copy TPC-H Parquet files, then bind
# it to vfio-pci ready for PCI passthrough. WARNING: destroys all data on the SSD.
# Requires: driverctl, sudo.
setup-nvme-ssd tpch="tpch10" confirm="yes":
    #!/usr/bin/env bash
    set -euo pipefail

    src="/scratch/ilya/{{tpch}}"

    if [[ ! -d "${src}" ]]; then
        echo "Error: source directory '${src}' not found" >&2; exit 1
    fi
    if [[ ! -d "/sys/bus/pci/devices/0000:{{ssd_id}}" ]]; then
        echo "Error: PCI device '0000:{{ssd_id}}' not found" >&2; exit 1
    fi

    if [[ "{{confirm}}" == "yes" ]]; then
        read -p "WARNING: ALL DATA ON 0000:{{ssd_id}} WILL BE DESTROYED. Continue? (y/N) " -n 1 -r
        echo
        [[ $REPLY =~ ^[Yy]$ ]] || { echo "Aborted."; exit 1; }
    fi

    # Ensure the device is bound to the nvme driver so we can reach the block device.
    current_driver=$(sudo driverctl list-devices | grep "{{ssd_id}}" | awk '{print $2}')
    if [[ "${current_driver}" != "nvme" ]]; then
        echo "==> Binding 0000:{{ssd_id}} to nvme driver ..."
        sudo driverctl set-override "0000:{{ssd_id}}" nvme
        sleep 1
    fi

    nvme_dev="/dev/$(ls /sys/bus/pci/devices/0000:{{ssd_id}}/nvme)n1"
    echo "==> Block device: ${nvme_dev}"

    echo "==> Formatting ${nvme_dev} as ext4 ..."
    sudo mke2fs -F -t ext4 -O ^metadata_csum "${nvme_dev}"

    mnt=$(mktemp -d)
    cleanup() { sudo umount "${mnt}" 2>/dev/null || true; rmdir "${mnt}" 2>/dev/null || true; }
    trap cleanup EXIT

    echo "==> Copying Parquet files from ${src} ..."
    sudo mount "${nvme_dev}" "${mnt}"
    sudo mkdir -p "${mnt}/tpch"
    sudo cp -v "${src}"/*.parquet "${mnt}/tpch/"
    sudo umount "${mnt}"
    rmdir "${mnt}"
    trap - EXIT

    echo "==> Binding 0000:{{ssd_id}} to vfio-pci ..."
    sudo driverctl set-override "0000:{{ssd_id}}" vfio-pci

    echo "Done. Run with: just run"