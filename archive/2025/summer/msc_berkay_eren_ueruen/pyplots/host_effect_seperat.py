import matplotlib.pyplot as plt
import matplotlib
import numpy as np
from scipy.interpolate import make_interp_spline

# context size
x_axis_gpu = np.array(["No LLM-OS\n Service", 1, 10, 20, 30, 40, "No Limit"])
x_axis_cpu = np.array(["No LLM-OS\n Service", 0.5, 1, 1.5, 2, "No Limit"])

# Data for GPU plot
data_gpu = np.array([0, 10813820552, 10831253104, 10813706132, 10810291586, 10831513788, 0])
base_gpu = [11553883234 / 1024 / 1024, 0, 0, 0, 0, 0, 0]
no_limit_gpu = np.array([0, 0, 0, 0, 0, 0, 10805066652 / 1024 / 1024])

# Data for CPU plot
data_cpu = np.array([0, 10660883102, 10550895312, 10418828036, 10321112206, 0])
base_cpu = [11553883234 / 1024 / 1024, 0, 0, 0, 0, 0]
no_limit_cpu = np.array([0, 0, 0, 0, 0, 9941705452 / 1024 / 1024])

# Convert to MB/s
data_gpu = data_gpu / 1024 / 1024
data_cpu = data_cpu / 1024 / 1024

# Create and save GPU plot
fig_gpu = plt.figure(figsize=(7, 6))
ax_gpu = fig_gpu.add_subplot(111)
ax_gpu.set_title("GPU")
ax_gpu.set_xlabel("Throughput Limit (tokens per second)")
ax_gpu.set_ylabel("OpenSSL Performance (megabytes per second)")
ax_gpu.set_ylim([7000, 12000])

a = ax_gpu.bar(x_axis_gpu, data_gpu, label='Throughput Limit', color='#A0CA9C')
ax_gpu.bar_label(a, label_type='edge', fmt=int)
a = ax_gpu.bar(x_axis_gpu, base_gpu, label='No LLM-OS Service', linestyle='dashed', color='#A897F5')
ax_gpu.bar_label(a, label_type='edge', fmt=int)
a = ax_gpu.bar(x_axis_gpu, no_limit_gpu, label='No Throughput Limit', linestyle='dashed', color='#747474')
ax_gpu.bar_label(a, label_type='edge', fmt=int)
ax_gpu.legend(loc='lower right')
plt.tight_layout()
plt.savefig('host-effect-gpu.png', format='png', dpi=1200)
plt.close(fig_gpu)

# Create and save CPU plot
fig_cpu = plt.figure(figsize=(7, 6))
ax_cpu = fig_cpu.add_subplot(111)
ax_cpu.set_title("CPU")
ax_cpu.set_xlabel("Throughput Limit (tokens per second)")
ax_cpu.set_ylabel("OpenSSL Performance (megabytes per second)")
ax_cpu.set_ylim([7000, 12000])

a = ax_cpu.bar(x_axis_cpu, data_cpu, label='Throughput Limit', color='#F3D683')
ax_cpu.bar_label(a, label_type='edge', fmt=int)
a = ax_cpu.bar(x_axis_cpu, base_cpu, label='No LLM-OS Service', linestyle='dashed', color='#A897F5')
ax_cpu.bar_label(a, label_type='edge', fmt=int)
a = ax_cpu.bar(x_axis_cpu, no_limit_cpu, label='No Throughput Limit', linestyle='dashed', color='#747474')
ax_cpu.bar_label(a, label_type='edge', fmt=int)
ax_cpu.legend(loc='lower right')
plt.tight_layout()
plt.savefig('host-effect-cpu.png', format='png', dpi=1200)
plt.close(fig_cpu)

# If you still want the combined figure as well
fig_combined, (ax1, ax2) = plt.subplots(1, 2, figsize=(12, 6))
fig_combined.suptitle('Client side OpenSSL SHA256 performance by inference server throughput limit')
fig_combined.supxlabel("Throughput Limit (tokens per second)")
fig_combined.supylabel("OpenSSL Performance (megabytes per second)")

ax1.set_ylim([7000, 12000])
ax2.set_ylim([7000, 12000])

# GPU plot (left)
a = ax1.bar(x_axis_gpu, data_gpu, label='Throughput Limit', color='#A0CA9C')
ax1.bar_label(a, label_type='edge', fmt=int)
a = ax1.bar(x_axis_gpu, base_gpu, label='No LLM-OS Service', linestyle='dashed', color='#A897F5')
ax1.bar_label(a, label_type='edge', fmt=int)
a = ax1.bar(x_axis_gpu, no_limit_gpu, label='No Throughput Limit', linestyle='dashed', color='#747474')
ax1.bar_label(a, label_type='edge', fmt=int)
ax1.set_title("GPU")

# CPU plot (right)
a = ax2.bar(x_axis_cpu, data_cpu, label='Throughput Limit', color='#F3D683')
ax2.bar_label(a, label_type='edge', fmt=int)
a = ax2.bar(x_axis_cpu, base_cpu, label='No LLM-OS Service', linestyle='dashed', color='#A897F5')
ax2.bar_label(a, label_type='edge', fmt=int)
a = ax2.bar(x_axis_cpu, no_limit_cpu, label='No Throughput Limit', linestyle='dashed', color='#747474')
ax2.bar_label(a, label_type='edge', fmt=int)
ax2.set_title("CPU")

ax1.legend(loc='lower right')
ax2.legend(loc='lower right')

plt.tight_layout()
plt.savefig('host-effect-combined.png', format='png', dpi=1200)
plt.close(fig_combined)