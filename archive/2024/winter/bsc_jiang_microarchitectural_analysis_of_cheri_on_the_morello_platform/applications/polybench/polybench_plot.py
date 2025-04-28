import pandas as pd
import matplotlib.pyplot as plt
import numpy as np
import sys

# Read the CSV data
if len(sys.argv) > 1:
    df = pd.read_csv(sys.argv[1])
else:
    df = pd.read_csv(sys.stdin)

# Group by benchmark and capability, then calculate mean and std for each group
grouped_df = df.groupby(['benchmark', 'capability'])['time'].agg(['mean', 'std']).reset_index()

# Create pivot tables with mean and std values
pivot_mean = grouped_df.pivot(index='benchmark', columns='capability', values='mean')
pivot_std = grouped_df.pivot(index='benchmark', columns='capability', values='std')

# Calculate normalized values and errors
# For NOCAP: value is 1.0, error is std/mean
# For PURECAP: value is PURECAP/NOCAP ratio, error is propagated
normalized_data = pd.DataFrame(index=pivot_mean.index)
normalized_data['NOCAP_val'] = 1.0
normalized_data['NOCAP_err'] = pivot_std['NOCAP'] / pivot_mean['NOCAP']
normalized_data['PURECAP_val'] = pivot_mean['PURECAP'] / pivot_mean['NOCAP']
normalized_data['PURECAP_err'] = normalized_data['PURECAP_val'] * np.sqrt(
    (pivot_std['PURECAP'] / pivot_mean['PURECAP'])**2 +
    (pivot_std['NOCAP'] / pivot_mean['NOCAP'])**2
)

# Sort benchmarks based on PURECAP/NOCAP ratio
sorted_data = normalized_data.sort_values('PURECAP_val')

# Create the figure and axis
fig, ax = plt.subplots(figsize=(14, 10))

# Set width of bars
index = np.arange(len(sorted_data.index))
bar_width = 0.35

# Plot NOCAP values (always 1.0) with error bars
nocap_bars = ax.bar(index - bar_width/2, sorted_data['NOCAP_val'],
                   width=bar_width, color='lightblue', edgecolor='black',
                   label='NOCAP (baseline)', alpha=0.7)

# Add error bars for NOCAP
ax.errorbar(index - bar_width/2, sorted_data['NOCAP_val'],
           yerr=sorted_data['NOCAP_err'], fmt='none', color='black', capsize=3)

# Plot PURECAP values (normalized to NOCAP) with error bars
purecap_bars = ax.bar(index + bar_width/2, sorted_data['PURECAP_val'],
                     width=bar_width, edgecolor='black',
                     label='PURECAP (relative to NOCAP)', alpha=0.7)

# Add error bars for PURECAP
ax.errorbar(index + bar_width/2, sorted_data['PURECAP_val'],
           yerr=sorted_data['PURECAP_err'], fmt='none', color='black', capsize=3)

# # Add a horizontal line at y=1.0 to indicate the baseline
# ax.axhline(y=1.0, color='black', linestyle='--', alpha=0.5)

# # Add percentage labels for PURECAP bars
# for i, ratio in enumerate(sorted_data['PURECAP_val']):
#     pct_diff = (ratio - 1) * 100
#     if abs(pct_diff) > 1:  # Only label if difference is more than 1%
#         label = f"{pct_diff:.1f}%"
#         y_pos = ratio + 0.05 if ratio > 1 else ratio - 0.1
#         ax.text(i + bar_width/2, y_pos, label, ha='center', va='center',
#                fontsize=8, color='black', fontweight='bold')

# Customize the plot
ax.set_xlabel('Benchmarks')
ax.set_ylabel('Normalized Performance (NOCAP = 1.0)')
ax.set_title('Polybench Performance: PURECAP vs NOCAP (Normalized)')
ax.set_xticks(index)
ax.set_xticklabels(sorted_data.index, rotation=45, ha='right')

# Add legend
ax.legend()

# Set y-axis limits with some padding
y_min = min(0.8, sorted_data['PURECAP_val'].min() * 0.9)
y_max = max(1.2, sorted_data['PURECAP_val'].max() * 1.1)
ax.set_ylim(y_min, y_max)

# Set grid for better readability
ax.grid(True, linestyle='--', alpha=0.3)

# Tight layout to ensure labels fit
plt.tight_layout()

plt.savefig('./output/polybench_plot.svg', dpi=300, bbox_inches='tight')
