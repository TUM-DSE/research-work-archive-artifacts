import matplotlib.pyplot as plt
import numpy as np
from scipy.interpolate import make_interp_spline
import matplotlib.pyplot as plt

# context size
x_axis = np.array([10, 20, 30, 40, 50, 100, "Single\nClient"])

# ms/token
vm_gpu = np.array([52.4502, 63.1353, 70.7027, 73.7928, 76.4382, 79.096, 0])
vm_cpu = np.array([1.02924, 1.34828,1.64041, 1.8008,1.86078,2.48173, 0])

baseline_gpu_1_client = np.array([0,0,0,0,0,0, 81.8536])
baseline_cpu_1_client = np.array([0,0,0,0,0,0, 2.83991])

# smooth out curves
#x_axis_new = np.linspace(x_axis.min(), x_axis.max(), 500)

fig, (ax1, ax2) = plt.subplots(1, 2,figsize=(12, 5.5))
fig.suptitle("Scheduling Granulaties' Effect on Performance of 16 Concurrent Clients")

#plt.title("LLM-OS overhead")
fig.supxlabel("Number of tokens to generate between scheduler calls")
fig.supylabel("Throughput (tokens per second)")
a = ax1.bar(x_axis.astype('str'), vm_gpu, label = 'GPU Throughput', color=('#A0CA9C'))
ax1.bar_label(a, label_type='edge', fmt=lambda x: "{:.2f}".format(x) if x > 0 else '')
a = ax1.bar(x_axis.astype('str'), baseline_gpu_1_client, label = 'Baseline', color=('#747474'))
ax1.bar_label(a, label_type='edge', fmt=lambda x: "{:.2f}".format(x) if x > 0 else '')
ax1.set_title("GPU")
ax1.legend()

ax1.set_ylim([0,100])

a = ax2.bar(x_axis.astype('str'), vm_cpu, label = 'CPU Throughput', color=('#F3D683'))
ax2.bar_label(a, label_type='edge', fmt=lambda x: "{:.2f}".format(x) if x > 0 else '')
a = ax2.bar(x_axis.astype('str'), baseline_cpu_1_client, label = 'Baseline', color=('#747474'))
ax2.bar_label(a, label_type='edge', fmt=lambda x: "{:.2f}".format(x) if x > 0 else '')
ax2.set_title("CPU")
ax2.set_ylim([0,5])
ax2.legend()

plt.savefig('scheduler.png', format='png', dpi=1200)
#plt.show()

