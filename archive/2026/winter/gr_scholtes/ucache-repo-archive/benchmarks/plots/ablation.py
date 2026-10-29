#!/usr/bin/env python3
# -*- coding: utf-8 -*-
# Plot results/ablation.csv: one OSv build per ablation step, see benchmarks/ablation.just.
# ablation_cold.pdf, ablation_warm.pdf: one panel per (cache_pct, query), one bar per step.
# Cold is repetition 1, warm the later repetitions. A bar is the mean over the boots, the
# whisker their standard deviation.
# ablation_q<query>_<pct>pct.tex: the same means as a LaTeX table, cold and warm side by side.
from common import *

CSV = os.path.join(result_dir, "ablation.csv")

# TEMP: plot only tpch300, labeled in titles and filenames.
SCALE_FILTER = "tpch300"
SCALE_LABEL = "tpchsf300"

# What each step adds to the one before it (osv/benchmarks/duckdb/osv_ablation.hpp).
STEP_LABELS = {
    1: "base",
    2: "+prefetch",
    3: "+region evict",
    4: "+2 queues",
    5: "+no IPI",
    6: "+zero-copy",
}
# Runs that differ in any of these are different experiments, never averaged together.
CONFIG = ["scale", "threads", "evict_batch", "prefetch_batch",
          "sim_latency_us", "sim_bandwidth_gbps"]
POINT = ["cache_pct", "query", "step"]
# The first step's bar is cut off when it is this many times above the other steps.
CUT_RATIO = 2
KEY = CONFIG + POINT + ["boot", "repetition"]

NUMERIC = ["step", "boot", "repetition", "time", "metadata_time", "read_bytes",
           "prefetch_used", "peak_used", "min_huge_blocks", "cache_bytes", "cache_pct",
           "threads", "evict_batch", "prefetch_batch", "sim_latency_us",
           "sim_bandwidth_gbps"]


def load_data(path=CSV) -> pd.DataFrame:
    df = pd.read_csv(path, dtype={"query": str})
    # FAIL / TIMEOUT / ERROR / "na" -> NaN, so a dead run is a missing bar, not a 0.
    for col in NUMERIC:
        df[col] = pd.to_numeric(df[col], errors="coerce")
    df["query"] = df["query"].str.zfill(2)
    df = df[df["scale"] == SCALE_FILTER]  # TEMP: see SCALE_FILTER above.
    # Append-only file: keep the newest row per key so a re-run is not averaged
    # together with the rows it supersedes.
    dup = df.duplicated(KEY, keep="last")
    if dup.any():
        print(f"note: dropped {dup.sum()} superseded row(s) from re-runs")
        df = df[~dup]
    # Keep the configuration with the most rows, e.g. when the file also holds a run
    # with other THREADS or batch sizes.
    configs = df.groupby(CONFIG, dropna=False).size().sort_values(ascending=False)
    if len(configs) > 1:
        keep = configs.index[0]
        print(f"note: {len(configs)} configurations in the file; plotting "
              + ", ".join(f"{c}={v}" for c, v in zip(CONFIG, keep)) + " only")
        df = df[(df[CONFIG] == pd.Series(keep, index=CONFIG)).all(axis=1)]
    return df


def per_boot(df) -> pd.DataFrame:
    """One cold and one warm sample per boot: repetition 1, and the mean of the
    later repetitions of that boot (none if REPEAT was 1)."""
    cold = df[df["repetition"] == 1].assign(style="cold")
    warm = (df[df["repetition"] >= 2]
            .groupby(POINT + ["boot"], as_index=False)
            .agg({"time": "mean", "read_bytes": "mean"})
            .assign(style="warm"))
    cols = POINT + ["boot", "style", "time", "read_bytes"]
    return pd.concat([cold[cols], warm[cols]], ignore_index=True)


def draw_cut(ax, bar, ylim, text):
    """Marks bar as cut off at the top of the axis: a slanted break across it below the
    top, and its label inside the bar under the break."""
    x0, w = bar.get_x(), bar.get_width()
    y, gap, slant = 0.84 * ylim, 0.04 * ylim, 0.04 * ylim
    xs = [x0 - 0.1 * w, x0 + 1.1 * w]
    ax.fill_between(xs, [y, y + slant], [y + gap, y + gap + slant],
                    color="white", zorder=3, clip_on=False)
    for dy in (0, gap):
        ax.plot(xs, [y + dy, y + slant + dy], color="black", linewidth=0.6,
                zorder=4, clip_on=False)
    ax.text(x0 + w / 2, y - 0.03 * ylim, text, ha="center", va="top",
            fontsize=FONTSIZE - 2, zorder=4)


def plot_steps(samples, style):
    """One figure per run type, so the y axis fits that run type's bars."""
    samples = samples[samples["style"] == style]
    if not len(samples):
        print(f"note: no {style} runs; skipping ablation_{style}.pdf")
        return
    pcts = sorted(samples["cache_pct"].unique())
    queries = sorted(samples["query"].unique())
    steps = sorted(samples["step"].unique())
    labels = [STEP_LABELS.get(s, f"step {s:g}") for s in steps]

    fig, axes = plt.subplots(len(pcts), len(queries), sharex=True, squeeze=False,
                             figsize=(max(figwidth_half, figwidth_half * len(queries)),
                                      (fig_height + 0.5) * len(pcts)))
    for i, pct in enumerate(pcts):
        for j, query in enumerate(queries):
            ax = axes[i][j]
            sub = samples[(samples["cache_pct"] == pct) & (samples["query"] == query)]
            if not len(sub):
                ax.set_axis_off()
                continue
            sns.barplot(data=sub, x="step", y="time", order=steps, color=palette[1],
                        errorbar="sd", capsize=0.2, err_kws={"linewidth": 0.6},
                        edgecolor="black", linewidth=0.5, ax=ax)
            # Above each bar: its time, and the change against the previous step.
            mean = sub.groupby("step")["time"].mean().reindex(steps)
            texts = []
            for k, t in enumerate(mean):
                if pd.isna(t):
                    texts.append("")
                elif k == 0 or pd.isna(mean.iloc[k - 1]):
                    texts.append(f"{t:.2f}")
                else:
                    texts.append(f"{t:.2f}\n{100 * (t / mean.iloc[k - 1] - 1):+.0f}%")
            # A first step far above the others is cut off at the top of the axis, so the
            # later steps fill the axis and their differences show.
            bar_top = mean + sub.groupby("step")["time"].std().reindex(steps).fillna(0)
            top = bar_top.max()
            cut = len(steps) > 1 and mean.iloc[0] > CUT_RATIO * bar_top.iloc[1:].max()
            if cut:
                top = bar_top.iloc[1:].max()
                cut_text, texts[0] = texts[0], ""
            ax.bar_label(ax.containers[0], labels=texts, fontsize=FONTSIZE - 2, padding=2)
            ax.set_xticks(range(len(steps)))
            ax.set_xticklabels(labels, rotation=30, ha="right")
            ax.set_xlabel("")
            ax.set_ylabel("Time (s)" if j == 0 else "")
            # Room for the two-line bar labels.
            ylim = top * 1.4
            ax.set_ylim(0, ylim)
            if cut:
                draw_cut(ax, ax.containers[0][0], ylim, cut_text)
            ax.set_title(f"Q{query}, cache = {pct:g}% of {SCALE_LABEL}", fontsize=FONTSIZE)
            ax.tick_params(labelsize=FONTSIZE - 1)
            ax.set_axisbelow(True)
    fig.suptitle(f"uCache ablation, {SCALE_LABEL}, {style} runs, " + lower_better_str,
                 fontsize=FONTSIZE, color="navy")
    fig.tight_layout()
    out = os.path.join(result_dir, f"ablation_{style}_{SCALE_LABEL}.pdf")
    fig.savefig(out, format="pdf", bbox_inches="tight")
    print("wrote", out)


def print_summary(samples):
    """Mean, spread over the boots and change against the previous step."""
    stats = (samples.groupby(["cache_pct", "query", "style", "step"])["time"]
             .agg(["mean", "std", "count"]).reset_index())
    for (pct, query, style), grp in stats.groupby(["cache_pct", "query", "style"]):
        print(f"cache_pct={pct:g} Q{query} {style}:")
        prev = None
        for _, r in grp.sort_values("step").iterrows():
            change = "" if prev is None else f"  {100 * (r['mean'] / prev - 1):+6.1f}%"
            spread = "" if r["count"] < 2 else f" +-{r['std']:.3f}"
            label = STEP_LABELS.get(r["step"], "")
            print(f"  step {r['step']:g} {label:<12} {r['mean']:7.3f}s{spread}"
                  f"  (boots={r['count']:g}){change}")
            prev = r["mean"]


def write_tables(samples):
    """ablation_q<query>_<pct>pct.tex: one LaTeX table per (cache_pct, query), a row per
    step with its cold and warm mean and the change against the previous step."""
    mean = samples.groupby(["cache_pct", "query", "style", "step"])["time"].mean()
    for (pct, query), m in mean.groupby(level=["cache_pct", "query"]):
        m = m.droplevel(["cache_pct", "query"])
        lines = [r"\begin{tabular}{lrrrr}", r"\toprule",
                 r"Step & Cold (s) & $\Delta$ & Warm (s) & $\Delta$ \\", r"\midrule"]
        prev = {}
        for step in sorted(m.index.get_level_values("step").unique()):
            cells = [STEP_LABELS.get(step, f"step {step:g}")]
            for style in ("cold", "warm"):
                t = m.get((style, step))
                if t is None:
                    cells += ["", ""]
                    continue
                p = prev.get(style)
                cells += [f"{t:.2f}", "" if p is None else f"${100 * (t / p - 1):+.0f}\\%$"]
                prev[style] = t
            lines.append(" & ".join(cells) + r" \\")
        lines += [r"\bottomrule", r"\end{tabular}"]
        out = os.path.join(result_dir, f"ablation_q{query}_{pct:g}pct_{SCALE_LABEL}.tex")
        with open(out, "w") as f:
            f.write("\n".join(lines) + "\n")
        print("wrote", out)


def main():
    df = load_data().dropna(subset=["time"])
    if not len(df):
        print("note: no timings in", CSV)
        return
    samples = per_boot(df)
    plot_steps(samples, "cold")
    plot_steps(samples, "warm")
    write_tables(samples)
    print_summary(samples)


if __name__ == "__main__":
    main()
