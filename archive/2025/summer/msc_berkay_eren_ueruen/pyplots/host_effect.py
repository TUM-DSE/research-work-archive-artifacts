import matplotlib.pyplot as plt
import numpy as np
from scipy.interpolate import make_interp_spline
import matplotlib.pyplot as plt

# context size
x_axis_gpu = np.array(["No LLM-OS\n Service", 1, 10, 20, 30, 40, "No Limit"])
#x_axis_cpu = np.array([4, 3, 2, 1])
x_axis_cpu = np.array(["No LLM-OS\n Service", 0.5, 1, 1.5, 2, "No Limit"])

# ms/token
#kv_reset = np.array([18.6,22.6,26.6,41,55, 73, 1460])
#data_gpu = np.array([10814699040 , 10251175665 , 10415925995 , 10437800165])
#data_gpu = np.array([9433416625 , 10475651855 , 10809967645 , 9971406935])
data_gpu = np.array([0, 10813820552, 10831253104, 10813706132, 10810291586, 10831513788, 0])
#data_cpu = np.array([8607574140 , 10037069885, 10246498645 , 9965576165])
#data_cpu = np.array([10179859600 , 10265992510, 9909305865 , 9729481400])
#data_cpu = np.array([10388914660, 10481455910,10455820540, 10708577720])
data_cpu = np.array([0,10660883102,10550895312,10418828036, 10321112206, 0])

#base = np.full((4,1), 11588837277 / 1024 / 1024)
#base = np.full((4,1), 10860600075 / 1024 / 1024)
base_gpu = [11553883234 / 1024 / 1024, 0, 0, 0, 0, 0, 0]
base_cpu = [11553883234 / 1024 / 1024, 0, 0, 0, 0, 0]
#no_limit_gpu = np.full((4,1), 9377212543 / 1024 / 1024)
#no_limit_gpu = np.full((4,1), 8968689250 / 1024 / 1024)
no_limit_gpu = np.array([0,0,0,0,0,0,10805066652 / 1024 / 1024])
#no_limit_cpu = np.full((4,1), 9108544705 / 1024 / 1024)
#no_limit_cpu = np.full((4,1), 10084521855 / 1024 / 1024)
no_limit_cpu = np.array([0, 0, 0, 0, 0, 9941705452 / 1024 / 1024])

data_gpu = data_gpu / 1024 /1024
data_cpu = data_cpu / 1024 /1024

fig, (ax1, ax2) = plt.subplots(1, 2,figsize=(12, 6))
fig.suptitle('Client side OpenSSL SHA256 performance by inference server throughput limit')

# smooth out curves
#x_axis_new = np.linspace(x_axis.min(), x_axis.max(), 500)


#plt.title("LLM-OS overhead")
fig.supxlabel("Throughput Limit (tokens per second)")
fig.supylabel("OpenSSL Performance (megabytes per second)")

ax1.set_ylim([7000,12000])
ax2.set_ylim([7000,12000])

a = ax1.bar(x_axis_gpu, data_gpu, label = 'Throughput Limit', color=('#A0CA9C'))
ax1.bar_label(a, label_type='edge', fmt=int)
a = ax1.bar(x_axis_gpu, base_gpu, label = 'No LLM-OS Service', linestyle='dashed', color=('#A897F5'))
ax1.bar_label(a, label_type='edge', fmt=int)
a = ax1.bar(x_axis_gpu, no_limit_gpu, label = 'No Throughput Limit', linestyle='dashed', color=('#747474'))
ax1.bar_label(a, label_type='edge', fmt=int)
ax1.set_title("GPU")


a = ax2.bar(x_axis_cpu, data_cpu, label = 'Throughput Limit', color=('#F3D683'))
ax2.bar_label(a, label_type='edge', fmt=int)
a = ax2.bar(x_axis_cpu, base_cpu, label = 'No LLM-OS Service', linestyle='dashed', color=('#A897F5'))
ax2.bar_label(a, label_type='edge', fmt=int)
a = ax2.bar(x_axis_cpu, no_limit_cpu, label = 'No Throughput Limit', linestyle='dashed', color=('#747474'))
ax2.bar_label(a, label_type='edge', fmt=int)
ax2.set_title("CPU")

ax1.legend(loc='lower right')
ax2.legend(loc='lower right')

plt.savefig('host-effect.png', format='png', dpi=1200)
#plt.show()

