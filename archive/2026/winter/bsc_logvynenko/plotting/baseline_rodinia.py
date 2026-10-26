import matplotlib.pyplot as plt
import numpy as np

categories = ["b+tree", "bfs", "dwt2d", "gaussian", "hotspot", "lavaMD", "nn", "nw"]
group_a = [0.626419, 0.077877, 0.078216, 0.081937, 0.061091, 0.276027, 0.220161, 0.355406]
group_b = [0.611008, 0.065598, 0.065883, 0.068448, 0.060668, 0.227210, 0.208765, 0.321700]
group_c = [0.604993, 0.060426, 0.060715, 0.067079, 0.057010, 0.262293, 0.205602, 0.303391]

x = np.arange(len(categories))
width = 0.25  # narrower, since three bars must fit in the same space

fig, ax = plt.subplots(figsize=(7, 4))

ax.bar(x - width, group_a, width, label="Baseline", color="#8172B3",
       edgecolor="black", linewidth=0.6)
ax.bar(x,         group_b, width, label="CXL_GPU",  color="#4C72B0",
       edgecolor="black", linewidth=0.6)
ax.bar(x + width, group_c, width, label="CXL_GPU+L4", color="#DD8452",
       edgecolor="black", linewidth=0.6)

ax.set_ylabel("Latency (ticks × 10⁻¹²)")
ax.set_xlabel("Benchmark name")
ax.set_xticks(x)
ax.set_xticklabels(categories)
ax.legend(frameon=False)

plt.tight_layout()
plt.savefig("baseline.png", bbox_inches="tight", dpi=300)
