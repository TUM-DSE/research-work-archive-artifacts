#!/usr/bin/env python3

import json
import math
import os
import re

import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt
import numpy as np

SCRIPT_DIR = os.path.dirname(os.path.abspath(__file__))

with open(os.path.join(SCRIPT_DIR, "bench_config.json")) as f:
    config = json.load(f)

NATIVE_FILE     = os.path.join(SCRIPT_DIR, "native_execs.txt")
TRANSLATED_FILE = os.path.join(SCRIPT_DIR, "translated_execs.txt")
QEMU_FILE       = os.path.join(SCRIPT_DIR, "qemu_execs.txt")
OUTPUT_FAST     = os.path.join(SCRIPT_DIR, "benchmark_fast.pdf")
OUTPUT_SLOW     = os.path.join(SCRIPT_DIR, "benchmark_slow.pdf")
OUTPUT_TINY     = os.path.join(SCRIPT_DIR, "benchmark_tiny.pdf")
OUTPUT_FAST_SVG = os.path.join(SCRIPT_DIR, "benchmark_fast.svg")
OUTPUT_SLOW_SVG = os.path.join(SCRIPT_DIR, "benchmark_slow.svg")
OUTPUT_TINY_SVG = os.path.join(SCRIPT_DIR, "benchmark_tiny.svg")

TIMES_RE = re.compile(r"times=([0-9.,]+)")

COLOR_NATIVE     = "#4C72B0"
COLOR_TRANSLATED = "#DD8452"
COLOR_QEMU       = "#55A868"
BAR_WIDTH        = 0.25


def _stddev(times):
    mean = sum(times) / len(times)
    return math.sqrt(sum((t - mean) ** 2 for t in times) / len(times))


def parse_results(path):
    """Returns dict: name -> (mean, stddev)."""
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
                times = [float(t) for t in m.group(1).split(",")]
                results[name] = (sum(times) / len(times), _stddev(times))
    return results


native     = parse_results(NATIVE_FILE)
translated = parse_results(TRANSLATED_FILE)
qemu       = parse_results(QEMU_FILE) if os.path.exists(QEMU_FILE) else {}

# Only keep benchmarks present in both; sort by native mean
benchmarks = sorted(
    [b for b in native if b in translated],
    key=lambda b: native[b][0],
)

# Split: fast = native mean < 1 s; slow = the rest
fast = [b for b in benchmarks if native[b][0] < 1.0]
slow = [b for b in benchmarks if native[b][0] >= 1.0]


def _add_errorbars(ax, x_positions, means, sds):
    """Overlay symmetric error-bar arrows (↑↓ stddev) on bars."""
    ax.errorbar(
        x_positions, means, yerr=sds,
        fmt="none",
        ecolor="black",
        capsize=5,
        capthick=1.5,
        elinewidth=1.5,
        zorder=5,
    )


def _add_overhead_labels(ax, x_positions, n_means, t_means, t_sds, y_top=None):
    """Print slowdown ratio above each translated bar, clear of the error bar cap."""
    for x, nm, tm, tsd in zip(x_positions, n_means, t_means, t_sds):
        ratio = tm / nm
        # place label just above the top cap of the error bar
        y = (y_top if y_top is not None else (tm + tsd)) + ax.get_ylim()[1] * 0.025
        ax.text(
            x, tm + tsd + ax.get_ylim()[1] * 0.03,
            f"{ratio:.1f}×",
            ha="center", va="bottom",
            fontsize=7.5, color="black", zorder=6,
        )


def _style_ax(ax):
    ax.yaxis.grid(True, linestyle="--", alpha=0.6, zorder=0)
    ax.set_axisbelow(True)
    ax.spines["top"].set_visible(False)
    ax.spines["right"].set_visible(False)


def plot_fast(benchmarks_list, outpath):
    x       = np.arange(len(benchmarks_list))
    n_means = np.array([native[b][0]     for b in benchmarks_list])
    n_sds   = np.array([native[b][1]     for b in benchmarks_list])
    t_means = np.array([translated[b][0] for b in benchmarks_list])
    t_sds   = np.array([translated[b][1] for b in benchmarks_list])
    has_qemu = all(b in qemu for b in benchmarks_list)
    if has_qemu:
        q_means = np.array([qemu[b][0] for b in benchmarks_list])
        q_sds   = np.array([qemu[b][1] for b in benchmarks_list])

    fig, ax = plt.subplots(figsize=(max(6, len(benchmarks_list) * 1.1), 5))

    ax.bar(x - BAR_WIDTH, n_means, BAR_WIDTH,
           color=COLOR_NATIVE, label="Native", zorder=3)
    ax.bar(x,              t_means, BAR_WIDTH,
           color=COLOR_TRANSLATED, label="Translated (SGTranslator)", zorder=3)
    if has_qemu:
        ax.bar(x + BAR_WIDTH, q_means, BAR_WIDTH,
               color=COLOR_QEMU, label="Translated (QEMU)", zorder=3)

    _add_errorbars(ax, x - BAR_WIDTH, n_means, n_sds)
    _add_errorbars(ax, x,             t_means, t_sds)
    if has_qemu:
        _add_errorbars(ax, x + BAR_WIDTH, q_means, q_sds)

    ax.set_xticks(x)
    ax.set_xticklabels(benchmarks_list, rotation=30, ha="right", fontsize=10)
    ax.set_ylabel("Execution time (s)")
    ax.set_ylim(bottom=0)
    ax.legend(framealpha=0.9)
    _style_ax(ax)

    # Overhead labels — added after ylim is set
    _add_overhead_labels(ax, x,             n_means, t_means, t_sds)
    if has_qemu:
        _add_overhead_labels(ax, x + BAR_WIDTH, n_means, q_means, q_sds)

    fig.tight_layout()
    fig.savefig(outpath, bbox_inches="tight")
    svg_path = outpath.replace('.pdf', '.svg')
    fig.savefig(svg_path, bbox_inches="tight", format='svg')
    plt.close(fig)
    print(f"Saved {outpath}")
    print(f"Saved {svg_path}")


def plot_slow(benchmarks_list, outpath, cut_low=20.0, cut_high=90.0):
    """Bar chart with a broken y-axis: [0, cut_low] // [cut_high, max]."""
    x       = np.arange(len(benchmarks_list))
    n_means = np.array([native[b][0]     for b in benchmarks_list])
    n_sds   = np.array([native[b][1]     for b in benchmarks_list])
    t_means = np.array([translated[b][0] for b in benchmarks_list])
    t_sds   = np.array([translated[b][1] for b in benchmarks_list])
    has_qemu = all(b in qemu for b in benchmarks_list)
    if has_qemu:
        q_means = np.array([qemu[b][0] for b in benchmarks_list])
        q_sds   = np.array([qemu[b][1] for b in benchmarks_list])

    all_tops = [np.max(n_means + n_sds), np.max(t_means + t_sds)]
    if has_qemu:
        all_tops.append(np.max(q_means + q_sds))
    y_max = max(all_tops) * 1.08

    # Height ratio: scale panels proportionally to their y-range
    bot_range = cut_low
    top_range = y_max - cut_high

    fig, (ax_top, ax_bot) = plt.subplots(
        2, 1,
        sharex=True,
        figsize=(max(6, len(benchmarks_list) * 1.1), 6),
        gridspec_kw={"height_ratios": [top_range / bot_range, 1], "hspace": 0.06},
    )

    for ax in (ax_top, ax_bot):
        ax.bar(x - BAR_WIDTH, n_means, BAR_WIDTH,
               color=COLOR_NATIVE, label="Native", zorder=3)
        ax.bar(x,              t_means, BAR_WIDTH,
               color=COLOR_TRANSLATED, label="Translated (SGTranslator)", zorder=3)
        if has_qemu:
            ax.bar(x + BAR_WIDTH, q_means, BAR_WIDTH,
                   color=COLOR_QEMU, label="Translated (QEMU)", zorder=3)
        _add_errorbars(ax, x - BAR_WIDTH, n_means, n_sds)
        _add_errorbars(ax, x,             t_means, t_sds)
        if has_qemu:
            _add_errorbars(ax, x + BAR_WIDTH, q_means, q_sds)
        _style_ax(ax)

    ax_bot.set_ylim(0, cut_low)
    ax_top.set_ylim(cut_high, y_max)

    # Remove the spines between the two panels
    ax_top.spines["bottom"].set_visible(False)
    ax_bot.spines["top"].set_visible(False)
    ax_top.tick_params(bottom=False)

    # Diagonal break marks on both panel edges
    d = 0.012
    kw_top = dict(transform=ax_top.transAxes, color="k", clip_on=False, linewidth=1.2)
    ax_top.plot((-d, +d), (-d, +d), **kw_top)
    ax_top.plot((1 - d, 1 + d), (-d, +d), **kw_top)

    kw_bot = dict(transform=ax_bot.transAxes, color="k", clip_on=False, linewidth=1.2)
    ax_bot.plot((-d, +d), (1 - d, 1 + d), **kw_bot)
    ax_bot.plot((1 - d, 1 + d), (1 - d, 1 + d), **kw_bot)

    ax_bot.set_xticks(x)
    ax_bot.set_xticklabels(benchmarks_list, rotation=30, ha="right", fontsize=10)

    # Single shared y-axis label
    fig.text(-0.02, 0.5, "Execution time (s)", va="center", rotation="vertical", fontsize=11)

    ax_top.legend(framealpha=0.9)

    # Overhead labels — route each benchmark to the panel that shows its bar
    def _slow_label(means, sds, x_offset):
        for xi, nm, tm, tsd in zip(x + x_offset, n_means, means, sds):
            ratio = tm / nm
            label = f"{ratio:.1f}\u00d7"
            if tm >= cut_high:
                gap = (ax_top.get_ylim()[1] - ax_top.get_ylim()[0]) * 0.03
                ax_top.text(xi, tm + tsd + gap, label,
                            ha="center", va="bottom", fontsize=7.5, color="black", zorder=6)
            else:
                gap = (ax_bot.get_ylim()[1] - ax_bot.get_ylim()[0]) * 0.03
                ax_bot.text(xi, tm + tsd + gap, label,
                            ha="center", va="bottom", fontsize=7.5, color="black", zorder=6)

    _slow_label(t_means, t_sds, 0)
    if has_qemu:
        _slow_label(q_means, q_sds, BAR_WIDTH)

    fig.subplots_adjust(left=0.1, right=0.97, top=0.93, bottom=0.18)
    fig.savefig(outpath, bbox_inches="tight")
    svg_path = outpath.replace('.pdf', '.svg')
    fig.savefig(svg_path, bbox_inches="tight", format='svg')
    plt.close(fig)
    print(f"Saved {outpath}")
    print(f"Saved {svg_path}")


TINY_BENCHMARKS = ["gesummv", "atax", "bicg", "mvt", "gemver"]

plot_fast([b for b in fast if b not in TINY_BENCHMARKS], OUTPUT_FAST)
plot_fast(
    sorted([b for b in TINY_BENCHMARKS if b in native and b in translated],
           key=lambda b: native[b][0]),
    OUTPUT_TINY,
)
plot_slow(slow, OUTPUT_SLOW)
