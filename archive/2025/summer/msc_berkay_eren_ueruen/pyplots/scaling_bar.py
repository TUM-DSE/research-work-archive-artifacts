import matplotlib.pyplot as plt
import numpy as np

# context size
x_axis = np.array([2, 4, 8, 16])

# Data
same_prio_gpu = np.array([53.8424, 52.2921, 52.612, 52.4502])
diff_prio_gpu = np.array([80.6864, 70.6828, 72.5182, 73.5048])

same_prio_cpu = np.array([0.445242,0.611119, 0.8198, 1.02924])
diff_prio_cpu = np.array([3.01894, 3.03376, 3.01022, 2.52995])

base_gpu = np.full((4,1), 83.4922)
base_cpu = np.full((4,1), 3.07812)

# Bar width and x-axis offset
bar_width = 0.35
x_indices = np.arange(len(x_axis))

# Create plots
fig, (ax1, ax2) = plt.subplots(1, 2, figsize=(10, 5))
fig.suptitle('Throughput change with number of clients')
fig.supxlabel("Number of active clients")
fig.supylabel("Throughput (tokens per second)")

# GPU bar chart
ax1.bar(x_indices - bar_width/2, same_prio_gpu, width=bar_width, label='Same Priority Requests', color=('#57978e'))
ax1.bar(x_indices + bar_width/2, diff_prio_gpu, width=bar_width, label='Different Priority Requests', color=('#A0CA9C'))
ax1.hlines(y=83.4922, xmin=-0.5, xmax=3.5, colors='#747474', linestyles='--', label="Single Request")
ax1.set_xticks(x_indices)
ax1.set_xticklabels(x_axis)
ax1.set_title("GPU")
ax1.legend(loc='lower left')

# CPU bar chart
ax2.bar(x_indices - bar_width/2, same_prio_cpu, width=bar_width, label='Same Priority Requests', color=('#b9a050'))
ax2.bar(x_indices + bar_width/2, diff_prio_cpu, width=bar_width, label='Different Priority requests', color=('#F3D683'))
ax2.hlines(y=3.01894, xmin=-0.5, xmax=3.5, colors='#747474', linestyles='--', label="Single Request")
ax2.set_xticks(x_indices)
ax2.set_xticklabels(x_axis)
ax2.set_title("CPU")
ax2.legend(loc='lower right')


plt.tight_layout()
plt.savefig('server-client-scaling.png', format='png', dpi=1200)
#plt.show()
