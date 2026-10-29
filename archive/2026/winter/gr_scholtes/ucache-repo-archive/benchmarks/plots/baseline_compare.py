#!/usr/bin/env python3
# -*- coding: utf-8 -*-
# Plot results/baseline_compare.csv: original uCache vs this tree's ablation step 1 and
# step 6, see benchmarks/baseline_compare.just.
# baseline_compare.pdf: per-query runtime, each system's cold bar next to its warm bar.
# baseline_compare_5pct.pdf: the same at 5% cache, the original uCache vs step 6 only.
# baseline_sweep.pdf: total runtime vs cache_pct, and its ratio to the original uCache.
# The CSV has the columns of cachefs_compare.csv, so loading and the per-query plot are
# the ones of cachefs_compare.py.
# An optional argument, e.g. tpch300, plots that scale only, into files named
# <name>_<scale>.pdf.
import sys
import cachefs_compare as cc
from common import *

CSV = os.path.join(result_dir, "baseline_compare.csv")

SYSTEM_ORDER = ["ucache_upstream", "ucache_base", "ucache"]
# The system the ratios are taken against.
REFERENCE = "ucache_upstream"

cc.SYSTEM_ORDER = SYSTEM_ORDER
cc.COLORS = {"ucache_upstream": palette[0], "ucache_base": palette[2], "ucache": palette[1]}
cc.HATCH = {"ucache_upstream": hatch_def[0], "ucache_base": hatch_def[2],
            "ucache": hatch_def[1]}

# The headline plot: the original uCache against the full design, at one cache size.
HEADLINE_SYSTEMS = ["ucache_upstream", "ucache"]
HEADLINE_PCT = 5
HEADLINE_NAMES = {"ucache_upstream": "Unmodified uCache", "ucache": "Ours"}


def totals(df, systems):
    """Total time per (cache_pct, system), over the queries every system finished at
    every cache_pct: summing what is present would reward a system for its failures."""
    t = df.pivot_table(index=["cache_pct", "query"], columns="system", values="time")
    t = t.reindex(columns=systems).dropna()
    per_query = t.groupby(level="query").size()
    common = per_query[per_query == t.index.get_level_values("cache_pct").nunique()].index
    t = t[t.index.get_level_values("query").isin(common)]
    return t.groupby(level="cache_pct").sum(), len(common)


def plot_sweep(cold, warm, systems, suffix=""):
    """Total runtime against cache size, and below it each system against REFERENCE."""
    fig, (a1, a2) = plt.subplots(2, 1, sharex=True,
                                 figsize=(figwidth_half, (fig_height + 0.4) * 2))
    drawn = False
    for style, df, ls in (("cold", cold, "-"), ("warm", warm, "--")):
        if not len(df):
            continue
        tot, n = totals(df, systems)
        if len(tot) < 2:
            continue
        drawn = True
        for i, s in enumerate(systems):
            a1.plot(tot.index, tot[s], ls, marker=marker_def[i], color=cc.COLORS[s],
                    markersize=3, label=f"{s} ({style}, n={n})")
            if REFERENCE in systems and s != REFERENCE:
                a2.plot(tot.index, tot[s] / tot[REFERENCE], ls, marker=marker_def[i],
                        color=cc.COLORS[s], markersize=3, label=f"{s} ({style})")
    if not drawn:
        print("note: need 2+ cache_pcts for the sweep; skipping baseline_sweep.pdf")
        plt.close(fig)
        return
    # Log x, but ticked at the points actually measured, not at the decades.
    a1.set_xscale("log")
    a1.set_xticks(sorted(set(cold["cache_pct"]) | set(warm["cache_pct"])))
    a1.xaxis.set_major_formatter(ticker.ScalarFormatter())
    a1.xaxis.set_minor_locator(ticker.NullLocator())
    a1.set_ylabel("Total time (s)")
    a1.set_title("Query-set runtime vs cache size, " + lower_better_str,
                 fontsize=FONTSIZE)
    a1.legend(fontsize=FONTSIZE - 1)
    a2.axhline(1.0, color="black", linewidth=0.8)
    a2.set_ylabel(f"Time / {REFERENCE}")
    a2.set_xlabel("Cache size (% of dataset)")
    a2.set_title("Relative to the original uCache, 1.0 = parity", fontsize=FONTSIZE)
    if a2.get_legend_handles_labels()[0]:
        a2.legend(fontsize=FONTSIZE - 1)
    for ax in (a1, a2):
        ax.tick_params(labelsize=FONTSIZE - 1)
        ax.set_axisbelow(True)
    fig.tight_layout()
    out = os.path.join(result_dir, f"baseline_sweep{suffix}.pdf")
    fig.savefig(out, format="pdf", bbox_inches="tight")
    print("wrote", out)


def print_summary(cold, warm, systems):
    """Totals per cache_pct over the queries every system finished at that cache_pct,
    and each system against the one before it in SYSTEM_ORDER and against REFERENCE."""
    for style, d in (("cold", cold), ("warm", warm)):
        for pct, grp in d.groupby("cache_pct"):
            t = (grp.pivot_table(index="query", columns="system", values="time")
                    .reindex(columns=systems).dropna())
            if not len(t):
                continue
            tot = t.sum()
            print(f"{style} cache_pct={pct:g} (n={len(t)}):")
            for i, s in enumerate(systems):
                line = f"  {s:<16} {tot[s]:8.1f}s"
                if REFERENCE in systems and s != REFERENCE:
                    line += f"  {tot[s] / tot[REFERENCE]:.2f}x {REFERENCE}"
                if i > 0 and systems[i - 1] != REFERENCE:
                    line += f"  {tot[s] / tot[systems[i - 1]]:.2f}x {systems[i - 1]}"
                print(line)


def main():
    scale = sys.argv[1] if len(sys.argv) > 1 else None
    suffix = f"_{scale}" if scale else ""
    at = f", {scale}" if scale else ""
    df = cc.load_data(CSV, scale)
    failed = df[df["time"].isna()]
    for _, r in failed.iterrows():
        print(f"note: no timing for {r['system']} Q{r['query']} at cache_pct={r['cache_pct']:g}")
    df = df.dropna(subset=["time"])
    systems = [s for s in SYSTEM_ORDER if s in set(df["system"])]
    missing = [s for s in SYSTEM_ORDER if s not in systems]
    if missing:
        print("WARNING: no rows for " + ", ".join(missing))
    if not systems:
        print("note: no timings in", CSV)
        return
    cold, warm = cc.split_reps(df)
    cc.plot_runtime(cold, warm, name=f"baseline_compare{suffix}.pdf",
                    title="Original uCache vs this tree" + at)
    headline = lambda d: d[(d["cache_pct"] == HEADLINE_PCT)
                           & d["system"].isin(HEADLINE_SYSTEMS)]
    cc.plot_runtime(headline(cold), headline(warm), name=f"baseline_compare_5pct{suffix}.pdf",
                    title="Unmodified uCache vs Ours" + at, names=HEADLINE_NAMES)
    plot_sweep(cold, warm, systems, suffix)
    print_summary(cold, warm, systems)


if __name__ == "__main__":
    main()
