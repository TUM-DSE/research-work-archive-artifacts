#!/usr/bin/env python3
# -*- coding: utf-8 -*-
# Plot results/hdr.csv: ablation step 6 against step 6 keeping the first N frames of each page
# resident (step6_hdr<N>), see benchmarks/hdr.just. Warm runs only. One bar group per cache
# size and simulated object-store setting: a bar is the mean over the warm repetitions of all
# boots, the whisker their standard deviation. The label is the time a step6_hdr<N> saves over
# step6 in the same group.
# An optional argument plots results/<name>.csv instead.
import re
import sys
from common import *

BASE = "step6"

name = sys.argv[1] if len(sys.argv) > 1 else "hdr"
df = pd.read_csv(os.path.join(result_dir, f"{name}.csv"), dtype={"query": str})
# Rows from before the frame count was a parameter kept one frame.
df["variant"] = df["variant"].replace({"step6_hdr": "step6_hdr1"})
# FAIL/HANG/TIMEOUT -> NaN, so a dead run is a missing bar, not a 0.
df["time"] = pd.to_numeric(df["time"], errors="coerce")
for _, r in df[df["time"].isna()].iterrows():
    print(f"note: no timing for {r['variant']} Q{r['query']} at cache {r['cache_pct']:g}%, "
          f"{r['sim_latency_us']:g} us, {r['sim_bandwidth_gbps']:g} GB/s, boot {r['boot']}")
df = df.dropna(subset=["time"])
df = df[df["repetition"] > 1]
if not len(df):
    sys.exit(f"note: no warm timings in {name}.csv")


def frames(v):
    m = re.fullmatch(r"step6_hdr(\d+)", v)
    return int(m.group(1)) if m else 0


def label(v):
    if v == BASE:
        return "Ours"
    return f"Ours, first {frames(v)} frame{'s' if frames(v) > 1 else ''} kept"


order = sorted(df["variant"].unique(), key=frames)
names = {v: label(v) for v in order}

df["setting"] = [
    "No sim. latency" if lat == 0 and bw == 0 else f"{lat:g} µs, {bw:g} GB/s"
    for lat, bw in zip(df["sim_latency_us"], df["sim_bandwidth_gbps"])]
if df["cache_pct"].nunique() > 1:
    df["setting"] = [f"{p:g}% cache\n{s}" for p, s in zip(df["cache_pct"], df["setting"])]
if df["query"].nunique() > 1:
    df["setting"] = "Q" + df["query"] + "\n" + df["setting"]
settings = list(df.sort_values(["query", "cache_pct", "sim_latency_us", "sim_bandwidth_gbps"])
                ["setting"].unique())
c = df.iloc[0]
queries = ", ".join(f"Q{int(q)}" for q in sorted(df["query"].unique()))
config = f"TPC-H {queries}, {c['scale']}, "
if df["cache_pct"].nunique() == 1:
    config += f"cache {c['cache_pct']:g}% ({c['cache_bytes'] / 2**20:.0f} MiB), "
config += f"evict/prefetch batch {c['evict_batch']}/{c['prefetch_batch']}, warm runs"

stats = df.groupby(["setting", "variant"])["time"].agg(["mean", "std", "count"])
mean = stats["mean"].unstack().reindex(index=settings, columns=order)
std = stats["std"].unstack().reindex(index=settings, columns=order).fillna(0)
# Time saved by keeping frames, relative to evicting them, within the same group.
saved = {v: 100 * (1 - mean[v] / mean[BASE]) if BASE in mean else mean[v] * np.nan
         for v in order if v != BASE}
peak = df.groupby(["setting", "variant"])["peak_used"].mean().unstack() / 2**30
read = df.groupby(["setting", "variant"])["read_bytes"].mean().unstack() / 2**30
print(pd.concat([mean, stats["count"].unstack().reindex(index=settings, columns=order)
                 .add_suffix(" n"),
                 pd.DataFrame(saved).add_suffix(" saved %"),
                 read.reindex(index=settings, columns=order).add_suffix(" read GiB"),
                 peak.reindex(index=settings, columns=order).add_suffix(" peak GiB")],
                axis=1).round(3).to_string())

fig, ax = plt.subplots(figsize=(figwidth_half, fig_height + 0.5))
x = np.arange(len(settings))
width = 0.8 / len(order)
for i, v in enumerate(order):
    # Only the groups this variant has a timing for.
    have = mean[v].notna().to_numpy()
    xs = x[have] + (i - (len(order) - 1) / 2) * width
    bars = ax.bar(xs, mean[v][have], width, yerr=std[v][have], capsize=2,
                  color=palette[i], hatch=hatch_def[i], edgecolor="black", linewidth=0.5,
                  error_kw={"linewidth": 0.6}, label=names[v])
    if v != BASE:
        ax.bar_label(bars, labels=[f"{-s:+.1f}%" if pd.notna(s) else ""
                                   for s in saved[v][have]],
                     fontsize=FONTSIZE - 2, padding=2)
ax.set_xticks(x)
ax.set_xticklabels(settings)
ax.set_ylim(0, ax.get_ylim()[1] * 1.15)
ax.set_ylabel("Time (s)")
ax.legend(title="", fontsize=FONTSIZE - 1)
ax.set_title(f"Keeping Leading Page Frames Resident\n{config}", fontsize=FONTSIZE - 1,
             color="navy")
fig.tight_layout()
out = os.path.join(result_dir, f"{name}.pdf")
fig.savefig(out, format="pdf", bbox_inches="tight", pad_inches=0.02)
print("wrote", out)
