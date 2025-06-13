import matplotlib.pyplot as plt
import numpy as np
from scipy.interpolate import make_interp_spline
import matplotlib.pyplot as plt

# context size
x_axis = np.array([16, 32, 64, 128, 256, 512, 1024])

# ms/token
#kv_reset = np.array([18.6,22.6,26.6,41,55, 73, 1460])
native_gpu = np.array([97.126297, 97.044359, 95.145198,95.544189,94.979616,93.985399, 92.903395])
vm_gpu = np.array([92.169766, 94.367953, 94.813442, 94.287733, 93.690567, 92.628699, 91.229307])

native_cpu = np.array([3.198267,3.228166, 3.229830, 3.217932, 3.209158, 3.180029, 3.093288])
vm_cpu = np.array([3.144735, 3.183336,3.168134, 3.145999,3.157139,3.118043,3.069257])

# smooth out curves
#x_axis_new = np.linspace(x_axis.min(), x_axis.max(), 500)

print(np.average(native_gpu) / np.average(vm_gpu) * 100)
print(np.average(native_cpu) / np.average(vm_cpu)  * 100)

fig, (ax1, ax2) = plt.subplots(1, 2,figsize=(9, 4))
fig.suptitle('Inference overhead of virtualization with llama-bench')

#plt.title("LLM-OS overhead")
fig.supxlabel("Number of tokens generated")
fig.supylabel("Throughput (tokens per second)")
a = ax1.bar(x_axis.astype('str'), native_gpu, label = 'Native', color=('#57978e'))
ax1.bar_label(a, label_type='edge', fmt="%.1f")
a = ax1.bar(x_axis.astype('str'), vm_gpu, label = 'VM', color=('#A0CA9C'))
ax1.bar_label(a, label_type='edge', fmt="%.1f",padding=-12)
ax1.set_title("GPU")

ax1.set_ylim([60,100])

a = ax2.bar(x_axis.astype('str'), native_cpu, label = 'Native', color=('#b9a050'))
ax2.bar_label(a, label_type='edge', fmt="%.2f")
a=ax2.bar(x_axis.astype('str'), vm_cpu, label = 'VM', color=('#F3D683'))
ax2.bar_label(a, label_type='edge', fmt="%.2f", padding=-12)
ax2.set_title("CPU")
ax2.set_ylim([1,5])
ax2.legend()

plt.savefig('llama-bench-overhead.png', format='png', dpi=1200)
#plt.show()

