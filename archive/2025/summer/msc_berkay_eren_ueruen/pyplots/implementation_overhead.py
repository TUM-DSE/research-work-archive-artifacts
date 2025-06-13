import matplotlib.pyplot as plt
import numpy as np
from scipy.interpolate import make_interp_spline
import matplotlib.pyplot as plt

# context size
x_axis = np.array(['llama-bench\n(Native)', 'llama-bench\n(VM)', 'LLM-OS Kernel'])

# ms/token
#kv_reset = np.array([18.6,22.6,26.6,41,55, 73, 1460])
data_gpu = np.array([93.985399, 92.628699, 83.4922])
data_cpu = np.array([3.180029, 3.118043, 3.07812])

# smooth out curves
#x_axis_new = np.linspace(x_axis.min(), x_axis.max(), 500)

print(data_gpu[0] / data_gpu[2] * 100)
print(data_cpu[0] / data_cpu[2] * 100)

fig, (ax1, ax2) = plt.subplots(1, 2,figsize=(10, 5))
fig.suptitle('Inference overhead of our implementation with a single client generating 512 tokens')

#plt.title("LLM-OS overhead")
fig.supylabel("Throughput (tokens per second)")
a = ax1.bar(x_axis, data_gpu, label = 'GPU Inference', color=('#A0CA9C'))
a[0].set_color('#A0CA9C')
a[1].set_color('#77b196')
a[2].set_color('#57978e')
ax1.bar_label(a, label_type='edge', fmt="%.2f")
ax1.set_title("GPU")
ax1.set_ylim([0,100])
a = ax2.bar(x_axis, data_cpu, label = 'CPU Inference', color=('#F3D683'))
a[0].set_color('#F3D683')
a[1].set_color('#ffd088')
a[2].set_color('#ffc996')
ax2.bar_label(a, label_type='edge', fmt="%.2f")
ax2.set_title("CPU")
ax2.set_ylim([0,5])
plt.savefig('implementation-overhead.png', format='png', dpi=1200)
#plt.show()

