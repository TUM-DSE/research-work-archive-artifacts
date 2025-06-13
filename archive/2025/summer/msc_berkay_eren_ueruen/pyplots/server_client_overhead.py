import matplotlib.pyplot as plt
import numpy as np
from scipy.interpolate import make_interp_spline
import matplotlib.pyplot as plt

# context size
x_axis = np.array(['Native', 'VM'])

# ms/token
#kv_reset = np.array([18.6,22.6,26.6,41,55, 73, 1460])
data_gpu = np.array([86.4729, 81.8536])
data_cpu = np.array([1.61976, 2.83991])

# smooth out curves
#x_axis_new = np.linspace(x_axis.min(), x_axis.max(), 500)



#plt.title("LLM-OS overhead")
plt.title("Inference overhead of virtualization", fontsize=9, color="navy", weight="bold")
plt.xlabel("Execution environment")
plt.ylabel("Throughput (tokens per second)")
plt.bar(x_axis, data_gpu, label = 'GPU Inference')
plt.bar(x_axis, data_cpu, label = 'CPU Inference')
plt.legend()
plt.savefig('server-client-overhead.png', format='png', dpi=1200)
#plt.show()

