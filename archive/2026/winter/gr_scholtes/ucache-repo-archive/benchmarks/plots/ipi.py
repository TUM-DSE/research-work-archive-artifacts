#!/usr/bin/env python3
# -*- coding: utf-8 -*-
# Plot results/ipi.csv: ablation step 6 with and without TLB shootdown IPIs, see
# benchmarks/ipi.just. One bar pair per vCPU count: a bar is the mean over the boots of each
# boot's mean time, the whisker their standard deviation. The label is the IPI penalty.
# An optional argument plots results/<name>.csv instead; --warm drops each boot's first
# repetition.
import sys
from common import *

args = [a for a in sys.argv[1:] if a != "--warm"]
warm = "--warm" in sys.argv[1:]
name = args[0] if args else "ipi"
df = pd.read_csv(os.path.join(result_dir, f"{name}.csv"), dtype={"query": str})
# FAIL -> NaN, so a dead run is a missing bar, not a 0.
df["time"] = pd.to_numeric(df["time"], errors="coerce")
df = df.dropna(subset=["time"])
if warm:
    df = df[df["repetition"] > 1]
c = df.iloc[0]
config = (f"TPC-H Q{int(c['query'])}, cache {c['cache_pct']:g}%, evict/prefetch batch "
          f"{c['evict_batch']}/{c['prefetch_batch']}")
if warm:
    config += ", warm runs"
if "cpuset" in df:
    # Spare host cores beyond the vCPUs, from cpusets of the form 0-<last>.
    spare = set(df["cpuset"].str.split("-").str[1].astype(int) + 1 - df["vcpus"])
    config += "\nQEMU pinned to " + ", ".join(
        f"vCPUs + {s} host cores" if s else "one host core per vCPU" for s in sorted(spare))
df = df.groupby(["vcpus", "variant", "boot"], as_index=False)["time"].mean()

mean = df.groupby(["vcpus", "variant"])["time"].mean().unstack()
# Time saved by avoiding the IPIs, relative to sending them.
saved = 100 * (1 - mean["noipi"] / mean["ipi"])
print(pd.concat([mean, saved.rename("saved %")], axis=1).round(3))

fig, ax = plt.subplots(figsize=(figwidth_half, fig_height + 0.5))
sns.barplot(data=df, x="vcpus", y="time", hue="variant", hue_order=["ipi", "noipi"],
            palette=[palette[1], palette[0]], errorbar="sd", capsize=0.2,
            err_kws={"linewidth": 0.6}, edgecolor="black", linewidth=0.5, ax=ax)
ax.bar_label(ax.containers[1], labels=[f"{-s:+.1f}%" for s in saved],
             fontsize=FONTSIZE - 2, padding=2)
ax.set_ylim(0, ax.get_ylim()[1] * 1.1)
ax.set_xlabel("Cores")
ax.set_ylabel("Time (s)")
handles, labels = ax.get_legend_handles_labels()
names = {"noipi": "noipi (Ours)"}
ax.legend(handles, [names.get(l, l) for l in labels], title="", fontsize=FONTSIZE - 1)
ax.set_title(f"TLB Shootdown Strategy\n{config}", fontsize=FONTSIZE - 1, color="navy")
fig.tight_layout()
fig.savefig(os.path.join(result_dir, f"{name}.pdf"), format="pdf", bbox_inches="tight",
            pad_inches=0.02)
