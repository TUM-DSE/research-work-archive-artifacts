import matplotlib.pyplot as plt
import numpy as np

categories = ["bs", "cedt", "hsto", "rscd", "sc", "sssp", "tq"]
group_a = [0.082684, 0.086579, 0.479597, 0.188587, 0.082300, 2.894897/3, 0.085883]
group_b = [0.065386, 0.070366, 0.541974, 0.175279, 0.062606, 2.861072/3, 0.073498]
group_c = [0.056165, 0.070366, 0.535819, 0.105279, 0.062606, 2.861064/3, 0.069294]

x = np.arange(len(categories))
width = 0.25  # narrower, since three bars must fit in the same space

fig, ax = plt.subplots(figsize=(7, 4))

ax.bar(x - width, group_a, width, label="CXL_GPU", color="#55A868",
       edgecolor="black", linewidth=0.6)
ax.bar(x,         group_b, width, label="CXL_GPU+L4 Cache disabled",  color="#4C72B0",
       edgecolor="black", linewidth=0.6)
ax.bar(x + width, group_c, width, label="CXL_GPU+L4 Cache enabled", color="#DD8452",
       edgecolor="black", linewidth=0.6)

ax.set_ylabel("Latency (ticks × 10⁻¹²)")
ax.set_xlabel("Benchmark name")
ax.set_xticks(x)
ax.set_xticklabels(categories)
ax.legend(frameon=False)

plt.tight_layout()
plt.savefig("plain-cache-no-cache.png", bbox_inches="tight", dpi=300)
