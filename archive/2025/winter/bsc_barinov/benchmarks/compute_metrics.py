#!/usr/bin/env python3
"""
Compute overhead metrics for native vs. translated benchmark runs.

Metrics reported
----------------
Per benchmark:
  - Native / SGTranslator / QEMU mean ± stddev (s)
  - 95 % confidence interval for each mean  (t-distribution)
  - Coefficient of Variation (CV = stddev/mean)  – measures run-to-run stability
  - Slowdown vs native for SGTranslator and QEMU
  - QEMU / SGTranslator ratio  (>1 means QEMU is slower)

Summary:
  - Geometric mean slowdown  (standard for performance comparisons)
  - Arithmetic mean slowdown
  - Median slowdown
  - Min / Max slowdown  (best / worst benchmark)
  - Geometric mean slowdown per category
  - SGTranslator vs QEMU comparison
"""

import json
import math
import os
import re
import statistics

SCRIPT_DIR = os.path.dirname(os.path.abspath(__file__))

with open(os.path.join(SCRIPT_DIR, "bench_config.json")) as f:
    config = json.load(f)

NATIVE_FILE     = os.path.join(SCRIPT_DIR, "native_execs.txt")
TRANSLATED_FILE = os.path.join(SCRIPT_DIR, "translated_execs.txt")
QEMU_FILE       = os.path.join(SCRIPT_DIR, "qemu_execs.txt")
TIMES_RE        = re.compile(r"times=([0-9.,]+)")

CATEGORIES = {
    "BLAS-2 (matrix-vector)": ["atax", "bicg", "gemver", "gesummv", "mvt"],
    "BLAS-3 (matrix-matrix)": ["2mm", "3mm", "gemm", "symm", "syr2k", "syrk", "trmm"],
    "Other":                   ["doitgen", "floyd-warshall", "nussinov"],
}

# t-distribution critical values for 95 % CI (two-tailed) by degrees of freedom.
# Covers up to dof=100; beyond that 1.960 is used.
_T95 = {
    1: 12.706, 2: 4.303, 3: 3.182, 4: 2.776, 5: 2.571,
    6: 2.447,  7: 2.365, 8: 2.306, 9: 2.262, 10: 2.228,
    11: 2.201, 12: 2.179, 13: 2.160, 14: 2.145, 15: 2.131,
    16: 2.120, 17: 2.110, 18: 2.101, 19: 2.093, 20: 2.086,
    25: 2.060, 30: 2.042, 40: 2.021, 60: 2.000, 80: 1.990,
    100: 1.984,
}


def t95(n):
    """Two-tailed t critical value for 95 % CI with n-1 degrees of freedom."""
    dof = n - 1
    for threshold, tval in sorted(_T95.items()):
        if dof <= threshold:
            return tval
    return 1.960


def parse_results(path):
    """Returns dict: name -> list[float]."""
    results = {}
    with open(path) as f:
        for line in f:
            line = line.strip()
            if not line:
                continue
            name, _, rest = line.partition(": ")
            name = re.sub(r"\.c$", "", name)
            m = TIMES_RE.search(rest)
            if m:
                results[name] = [float(t) for t in m.group(1).split(",")]
    return results


def stats(times):
    n    = len(times)
    mean = sum(times) / n
    sd   = math.sqrt(sum((t - mean) ** 2 for t in times) / (n - 1))  # sample stddev
    ci   = t95(n) * sd / math.sqrt(n)
    cv   = sd / mean * 100  # coefficient of variation in percent
    return mean, sd, ci, cv


def geomean(values):
    return math.exp(sum(math.log(v) for v in values) / len(values))


# ── Load data ──────────────────────────────────────────────────────────────────

native_raw     = parse_results(NATIVE_FILE)
translated_raw = parse_results(TRANSLATED_FILE)
qemu_raw       = parse_results(QEMU_FILE) if os.path.exists(QEMU_FILE) else {}
has_qemu       = bool(qemu_raw)

common = sorted(
    [b for b in native_raw if b in translated_raw],
    key=lambda b: native_raw[b][0],   # sort by native mean for readability
)

# ── Per-benchmark table ────────────────────────────────────────────────────────

COL = 14

if has_qemu:
    header = (
        f"{'Benchmark':<20} "
        f"{'Native mean':>{COL}}  {'CV%':>5}  "
        f"{'SGTrans. mean':>{COL}}  {'CV%':>5}  {'Slowdown':>8}  "
        f"{'QEMU mean':>{COL}}  {'CV%':>5}  {'Slowdown':>8}  "
        f"{'QEMU/SGT':>8}"
    )
else:
    header = (
        f"{'Benchmark':<20} "
        f"{'Native mean':>{COL}}  {'±sd':>8}  {'95%CI':>8}  {'CV%':>5}  "
        f"{'Trans. mean':>{COL}}  {'±sd':>8}  {'95%CI':>8}  {'CV%':>5}  "
        f"{'Slowdown':>8}"
    )
sep = "-" * len(header)

print("\n" + sep)
print(header)
print(sep)

slowdowns = []
qemu_slowdowns = []

for b in common:
    n_mean, n_sd, n_ci, n_cv = stats(native_raw[b])
    t_mean, t_sd, t_ci, t_cv = stats(translated_raw[b])
    slow = t_mean / n_mean
    slowdowns.append((b, slow))
    if has_qemu and b in qemu_raw:
        q_mean, q_sd, q_ci, q_cv = stats(qemu_raw[b])
        q_slow = q_mean / n_mean
        qemu_slowdowns.append((b, q_slow))
        ratio = q_mean / t_mean
        print(
            f"{b:<20} "
            f"{n_mean:>{COL}.4f}  {n_cv:>5.2f}  "
            f"{t_mean:>{COL}.4f}  {t_cv:>5.2f}  {slow:>8.3f}x  "
            f"{q_mean:>{COL}.4f}  {q_cv:>5.2f}  {q_slow:>8.3f}x  "
            f"{ratio:>8.3f}x"
        )
    else:
        print(
            f"{b:<20} "
            f"{n_mean:>{COL}.4f}  {n_sd:>8.4f}  {n_ci:>8.4f}  {n_cv:>5.2f}  "
            f"{t_mean:>{COL}.4f}  {t_sd:>8.4f}  {t_ci:>8.4f}  {t_cv:>5.2f}  "
            f"{slow:>8.3f}x"
        )

print(sep)

# ── Overall summary ────────────────────────────────────────────────────────────

all_slowdowns = [s for _, s in slowdowns]
gm    = geomean(all_slowdowns)
am    = sum(all_slowdowns) / len(all_slowdowns)
med   = statistics.median(all_slowdowns)
best  = min(slowdowns, key=lambda x: x[1])
worst = max(slowdowns, key=lambda x: x[1])

print(f"\n{'── SGTranslator overall summary ':─<60}")
print(f"  Benchmarks measured   : {len(slowdowns)}")
print(f"  Geometric mean        : {gm:.3f}x")
print(f"  Arithmetic mean       : {am:.3f}x")
print(f"  Median                : {med:.3f}x")
print(f"  Best  slowdown        : {best[0]}  ({best[1]:.3f}x)")
print(f"  Worst slowdown        : {worst[0]}  ({worst[1]:.3f}x)")

# ── Per-category geometric mean ────────────────────────────────────────────────

print(f"\n{'── Geometric mean by category (SGTranslator) ':─<60}")
for cat, members in CATEGORIES.items():
    cat_slow = [s for b, s in slowdowns if b in members]
    if cat_slow:
        print(f"  {cat:<30}  {geomean(cat_slow):.3f}x  ({len(cat_slow)} benchmarks)")

# ── Stability summary (highest CV = most noisy benchmarks) ────────────────────

print(f"\n{'── Top 5 most variable benchmarks (native CV%) ':─<60}")
cvs = []
for b in common:
    n_mean, n_sd, _, n_cv = stats(native_raw[b])
    t_mean, t_sd, _, t_cv = stats(translated_raw[b])
    cvs.append((b, n_cv, t_cv))

for b, n_cv, t_cv in sorted(cvs, key=lambda x: max(x[1], x[2]), reverse=True)[:5]:
    print(f"  {b:<20}  native CV: {n_cv:.2f}%   translated CV: {t_cv:.2f}%")

# ── QEMU comparison ────────────────────────────────────────────────────────────

if has_qemu and qemu_slowdowns:
    q_all = [s for _, s in qemu_slowdowns]
    q_gm  = geomean(q_all)
    q_am  = sum(q_all) / len(q_all)
    q_med = statistics.median(q_all)
    q_best  = min(qemu_slowdowns, key=lambda x: x[1])
    q_worst = max(qemu_slowdowns, key=lambda x: x[1])

    print(f"\n{'── QEMU overall summary ':─<60}")
    print(f"  Benchmarks measured   : {len(qemu_slowdowns)}")
    print(f"  Geometric mean        : {q_gm:.3f}x")
    print(f"  Arithmetic mean       : {q_am:.3f}x")
    print(f"  Median                : {q_med:.3f}x")
    print(f"  Best  slowdown        : {q_best[0]}  ({q_best[1]:.3f}x)")
    print(f"  Worst slowdown        : {q_worst[0]}  ({q_worst[1]:.3f}x)")

    print(f"\n{'── Geometric mean by category (QEMU) ':─<60}")
    for cat, members in CATEGORIES.items():
        cat_slow = [s for b, s in qemu_slowdowns if b in members]
        if cat_slow:
            print(f"  {cat:<30}  {geomean(cat_slow):.3f}x  ({len(cat_slow)} benchmarks)")

    # SGT vs QEMU: ratio > 1 means SGTranslator is faster than QEMU
    common_q = [b for b, _ in qemu_slowdowns if b in dict(slowdowns)]
    sgt_dict  = dict(slowdowns)
    qemu_dict = dict(qemu_slowdowns)
    ratios = [(b, qemu_dict[b] / sgt_dict[b]) for b in common_q]
    ratio_gm = geomean([r for _, r in ratios])

    print(f"\n{'── SGTranslator vs QEMU (QEMU slowdown / SGT slowdown) ':─<60}")
    print(f"  Geometric mean ratio  : {ratio_gm:.3f}x  ", end="")
    if ratio_gm > 1:
        print(f"(SGTranslator is {ratio_gm:.3f}x faster than QEMU on average)")
    else:
        print(f"(QEMU is {1/ratio_gm:.3f}x faster than SGTranslator on average)")
    print()
    for b, r in sorted(ratios, key=lambda x: x[1], reverse=True):
        direction = f"SGT {r:.2f}x faster" if r > 1 else f"QEMU {1/r:.2f}x faster"
        print(f"  {b:<20}  {direction}")

print()
