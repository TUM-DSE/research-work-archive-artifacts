#!/usr/bin/env bash

set -o errexit  # when a command fails, exist
set -o nounset  # fail when accessing an unset variable
set -o pipefail # fail pipeline if any command errors

CORE=5
ARRAY_SIZES=(64 128 256 512 1024 2048 2560  3072 4096 6144 8192 10240  12288  16384 24576 32768 49152  65536 98304 131072 196608 262144 524288 786432  1048576 1572864 2097152 4194304 8388608 16777216 33554432 50331648 67108864)

run_experiment() {
    local executable=$1
    local output_file=$2

    rm -f "$output_file"
    touch "$output_file"
    echo "len;duration" >> "$output_file"
    for size in "${ARRAY_SIZES[@]}"; do
        taskset -c "$CORE" "$executable" 10 "$size" 1337 | tee -a "$output_file"
        sleep 5s
    done
}

rm -rf results
mkdir results

run_experiment "./load" "results/load.csv"
run_experiment "./load_mte" "results/load_mte.csv"

