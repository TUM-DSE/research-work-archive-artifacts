import matplotlib.pyplot as plt
import numpy as np

# Your data
categories = ["bs", "cedt", "hsto", "rscd", "sc", "sssp", "tq"]
group_a    = [0.082684, 0.086579, 0.479597, 0.188587, 0.082300, 2.894897/3, 0.085883]
group_b    = [0.056165, 0.070366, 0.535819, 0.105279, 0.062606, 2.861064/3, 0.069294]

x = np.arange(len(categories))  # label positions
width = 0.35                    # bar width

fig, ax = plt.subplots(figsize=(6, 4))

ax.bar(x - width/2, group_a, width, label="CXL-GPU", color="#4C72B0",
       edgecolor="black", linewidth=0.6)
ax.bar(x + width/2, group_b, width, label="CXL-GPU+L4",  color="#DD8452",
       edgecolor="black", linewidth=0.6)

ax.set_ylabel("Latency (ticks * 10^-12)")
ax.set_xlabel("Benchmark name")
ax.set_xticks(x)
ax.set_xticklabels(categories)
ax.legend(frameon=False)

plt.tight_layout()
plt.savefig("cxl-gpu_cxl-gpu-l4.png", bbox_inches="tight")
