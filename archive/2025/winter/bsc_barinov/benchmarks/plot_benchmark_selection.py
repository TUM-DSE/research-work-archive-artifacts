#!/usr/bin/env python3

import os

SCRIPT_DIR = os.path.dirname(os.path.abspath(__file__))
OUTPUT_TEX = os.path.join(SCRIPT_DIR, "benchmark_selection_table.tex")

# (polybench_category, benchmark, included, reason)
BENCHMARKS = [
    # Linear algebra — kernels
    ("Lin. Alg. Kernels", "2mm",      True,  ""),
    ("Lin. Alg. Kernels", "3mm",      True,  ""),
    ("Lin. Alg. Kernels", "atax",     True,  ""),
    ("Lin. Alg. Kernels", "bicg",     True,  ""),
    ("Lin. Alg. Kernels", "doitgen",  True,  ""),
    ("Lin. Alg. Kernels", "mvt",      True,  ""),
    # Linear algebra — BLAS
    ("Lin. Alg. BLAS",   "gemm",     True,  ""),
    ("Lin. Alg. BLAS",   "gemver",   True,  ""),
    ("Lin. Alg. BLAS",   "gesummv",  True,  ""),
    ("Lin. Alg. BLAS",   "symm",     True,  ""),
    ("Lin. Alg. BLAS",   "syr2k",    True,  ""),
    ("Lin. Alg. BLAS",   "syrk",     True,  ""),
    ("Lin. Alg. BLAS",   "trmm",     True,  ""),
    # Linear algebra — solvers
    ("Lin. Alg. Solvers", "cholesky",    False, "Requires FP"),
    ("Lin. Alg. Solvers", "durbin",      False, "Requires FP"),
    ("Lin. Alg. Solvers", "gramschmidt", False, "Requires FP"),
    ("Lin. Alg. Solvers", "lu",          False, "Requires FP"),
    ("Lin. Alg. Solvers", "ludcmp",      False, "Requires FP"),
    ("Lin. Alg. Solvers", "trisolv",     False, "Requires FP"),
    # Datamining
    ("Datamining",       "correlation", False, "Requires FP"),
    ("Datamining",       "covariance",  False, "Requires FP"),
    # Medley
    ("Medley",           "deriche",       False, "Requires FP"),
    ("Medley",           "floyd-warshall",True,  ""),
    ("Medley",           "nussinov",      True,  ""),
    # Stencils
    ("Stencils",         "adi",       False, "Requires FP"),
    ("Stencils",         "fdtd-2d",   False, "Requires FP"),
    ("Stencils",         "heat-3d",   False, "Requires FP"),
    ("Stencils",         "jacobi-1d", False, "Requires FP"),
    ("Stencils",         "jacobi-2d", False, "Requires FP"),
    ("Stencils",         "seidel-2d", False, "Requires FP"),
]

lines = []
lines.append(r"\begin{table}[h]")
lines.append(r"\centering")
lines.append(r"\small")
lines.append(r"\begin{tabularx}{\textwidth}{|X|X|>{\centering\arraybackslash}X|}")
lines.append(r"\hline")
lines.append(r"\textbf{Category} & \textbf{Benchmark} & \textbf{Included} \\")
lines.append(r"\hline")

prev_category = None
for i, (category, benchmark, included, reason) in enumerate(BENCHMARKS):
    cat_cell = category if category != prev_category else ""
    status = r"\textcolor{green}{$\checkmark$}" if included else r"\textcolor{red}{$\times$}"
    lines.append(f"  {cat_cell} & \\texttt{{{benchmark}}} & {status} \\\\")
    prev_category = category
    # Add hline between groups
    if i + 1 < len(BENCHMARKS) and BENCHMARKS[i + 1][0] != category:
        lines.append(r"\hline")

lines.append(r"\hline")
lines.append(r"\end{tabularx}")

included_count = sum(1 for _, _, inc, _ in BENCHMARKS if inc)
total_count = len(BENCHMARKS)
lines.append(f"\\caption{{PolyBenchC 4.2.1 benchmark selection. {included_count} of {total_count} benchmarks were included. Excluded benchmarks rely on floating-point arithmetic, which is not supported by the translator.}}")
lines.append(r"\label{tab:benchmark-selection}")
lines.append(r"\end{table}")

tex = "\n".join(lines) + "\n"

with open(OUTPUT_TEX, "w") as f:
    f.write(tex)

print(tex)
print(f"Table written to {OUTPUT_TEX}")
