#!/usr/bin/env python3

import glob
import json
import math
import os
import subprocess

SCRIPT_DIR = os.path.dirname(os.path.abspath(__file__))

with open(os.path.join(SCRIPT_DIR, "bench_config.json")) as f:
    config = json.load(f)

DATASET_SIZE = config["dataset_size"]

TESTS_DIR = os.path.join(SCRIPT_DIR, "polybench_tests")
OUTPUT_TEX = os.path.join(SCRIPT_DIR, "binary_sizes_table.tex")

gcc_base = [
    "aarch64-unknown-linux-gnu-gcc", "-O2", "-fno-stack-protector", "-w", "-mgeneral-regs-only",
    "-DSGT_CHANGES", "-DSGT_TIME", "-DDATA_TYPE_IS_INT", f"-D{DATASET_SIZE}",
    "-DPOLYBENCH_NO_FLUSH_CACHE",
]

polybench_src = os.path.join(SCRIPT_DIR, "./polybench.c")
polybench_inc = os.path.join(SCRIPT_DIR, ".")

SGTRANSLATOR = os.path.join(SCRIPT_DIR, "../build/bin/sgtranslator")
CONFIG = os.path.join(SCRIPT_DIR, "../config.json")

# PolyBench categories
GROUPS = {
    "BLAS-2 (matrix-vector)": ["atax", "bicg", "gemver", "gesummv", "mvt"],
    "BLAS-3 (matrix-matrix)": ["2mm", "3mm", "gemm", "symm", "syr2k", "syrk", "trmm"],
    "Other": ["doitgen", "floyd-warshall", "nussinov"],
}

# --- Compile and translate, collect sizes ---
sizes = {}  # benchmark -> (aarch64_bytes, x86_bytes)

for c_file in sorted(glob.glob(os.path.join(TESTS_DIR, "*.c"))):
    benchmark = os.path.splitext(os.path.basename(c_file))[0]
    arm_binary = os.path.join(TESTS_DIR, f"{benchmark}.arm")
    x86_binary = os.path.join(TESTS_DIR, f"{benchmark}.x86")
    print(f"Processing {benchmark}...")

    compile_cmd = gcc_base + [c_file, polybench_src, "-I", polybench_inc, "-o", arm_binary]
    result = subprocess.run(compile_cmd, capture_output=True, text=True)
    if result.returncode != 0:
        print(f"  COMPILATION FAILED: {result.stderr}")
        continue

    translate_cmd = [SGTRANSLATOR, arm_binary, "--config", CONFIG]
    result = subprocess.run(translate_cmd, capture_output=True, text=True)
    if result.returncode != 0:
        print(f"  TRANSLATION FAILED: {result.stderr}")
        try:
            os.remove(arm_binary)
        except FileNotFoundError:
            pass
        continue

    arm_size = os.path.getsize(arm_binary)
    x86_size = os.path.getsize(x86_binary)
    sizes[benchmark] = (arm_size, x86_size)
    print(f"  AArch64: {arm_size} B  x86: {x86_size} B  ratio: {x86_size/arm_size:.2f}x")

    for binary in [arm_binary, x86_binary]:
        try:
            os.remove(binary)
        except FileNotFoundError:
            pass

# --- Generate LaTeX table ---
lines = []
lines.append(r"\begin{table}[h]")
lines.append(r"\centering")
lines.append(r"\resizebox{\textwidth}{!}{%")
lines.append(r"\begin{tabular}{|c|c|c|c|c|}")
lines.append(r"\hline")
lines.append(r"\textbf{Group} & \textbf{Benchmark} & \textbf{AArch64 (KiB)} & \textbf{x86 (KiB)} & \textbf{Size ratio} \\")
lines.append(r"\hline")

all_ratios = []

for i, (group, benchmarks) in enumerate(GROUPS.items()):
    present = [b for b in benchmarks if b in sizes]
    for j, b in enumerate(present):
        arm_b, x86_b = sizes[b]
        arm_kib = arm_b / 1024
        x86_kib = x86_b / 1024
        ratio = x86_b / arm_b
        all_ratios.append(ratio)
        group_cell = r"\multirow{" + str(len(present)) + r"}{*}{" + group + r"}" if j == 0 else ""
        lines.append(f"  {group_cell} & \\texttt{{{b}}} & {arm_kib:.1f} & {x86_kib:.1f} & {ratio:.2f}$\\times$ \\\\")
    if i < len(GROUPS) - 1:
        lines.append(r"\hline")

geomean = math.exp(sum(math.log(r) for r in all_ratios) / len(all_ratios))
lines.append(r"\hline")
lines.append(f"  \\multicolumn{{5}}{{|c|}}{{Geometric mean size ratio: {geomean:.2f}$\\times$}} \\\\")
lines.append(r"\hline")
lines.append(r"\end{tabular}}")
lines.append(f"\\caption{{AArch64 vs.\\ translated x86 binary sizes for PolyBenchC benchmarks compiled with \\texttt{{-O2}}.}}")
lines.append(r"\label{tab:binary-sizes}")
lines.append(r"\end{table}")

tex = "\n".join(lines) + "\n"

with open(OUTPUT_TEX, "w") as f:
    f.write(tex)

print(tex)
print(f"Table written to {OUTPUT_TEX}")
