import matplotlib.pyplot as plt
import numpy as np

# Your data
categories = ["bs", "hsto", "rscd", "sc", "tq"]
group_a    = [0.091152, 0.597514, 0.209900, 0.144360, 0.093536]
group_b    = [0.082684, 0.479597, 0.188587, 0.082300, 0.085883]
group_c    = [0.056165, 0.535819, 0.105279, 0.062606, 0.069294]

x = np.arange(len(categories))
width = 0.26

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
plt.savefig("baseline_cxl_cxl_l4.png", bbox_inches="tight", dpi=300)
