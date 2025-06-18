#!/usr/bin/env bash
set -euo pipefail

# Directory containing the .cwasm files, relative to this script
TEST_DIR="$(dirname "$0")/../tests/bin"
OUT_CSV="$(dirname "$0")/benchmark_sightglass_objdump.csv"

# Files to skip
declare -A skip=()

# Header
echo "filename,lifting_time_ns" > "$OUT_CSV"

# Iterate all .cwasm in TEST_DIR
for fullpath in "$TEST_DIR"/*.cwasm; do
  fname=$(basename "$fullpath")
  # Skip excluded files
  if [[ -n "${skip[$fname]:-}" ]]; then
    continue
  fi

  # Time llvm-objdump -d (drop output)
  start=$(date +%s%N)
  llvm-objdump -d "$fullpath" > /dev/null
  end=$(date +%s%N)

  duration=$((end - start))
  echo "$fname,$duration" >> "$OUT_CSV"
done

echo "Results written to $OUT_CSV"
