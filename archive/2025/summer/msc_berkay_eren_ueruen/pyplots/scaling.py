import matplotlib.pyplot as plt
import numpy as np
from scipy.interpolate import make_interp_spline
import matplotlib.pyplot as plt

# context size
x_axis = np.array([1, 2, 4, 8, 16])

# ms/token
#kv_reset = np.array([18.6,22.6,26.6,41,55, 73, 1460])
same_prio_gpu = np.array([81.8536, 53.9743, 52.2258, 52.411, 50.3522])
diff_prio_gpu = np.array([81.8536, 80.4614, 72.5223, 72.9985, 71.122])

same_prio_cpu = np.array([2.83991, 0.589248, 0.820965, 1.08444, 1.30965])
diff_prio_cpu = np.array([2.83991, 3.00996, 3.01702, 3.01491, 2.45732])

# smooth out curves
#x_axis_new = np.linspace(x_axis.min(), x_axis.max(), 500)

fig, (ax1, ax2) = plt.subplots(1, 2,figsize=(10, 5))
fig.suptitle('Throughput change with number of clients')

#plt.title("LLM-OS overhead")
#ax1.title("Througput change with number of same priority client")
fig.supxlabel("Number of active clients")
fig.supylabel("Throughput (tokens per second)")
ax1.plot(x_axis, same_prio_gpu, label = 'Same priotiy requests')
ax1.plot(x_axis, diff_prio_gpu, label = 'Different priotiy requests')
ax1.set_title("GPU")

ax2.plot(x_axis, same_prio_cpu, label = 'Same priotiy requests')
ax2.plot(x_axis, diff_prio_cpu, label = 'Different priotiy requests')
ax2.set_title("CPU")
#plt.plot(x_axis, same_prio_gpu, label = 'Same priotiy requests on CPU')
#plt.plot(x_axis, same_prio_gpu, label = 'Different priotiy requests on CPU')
ax2.legend()
plt.savefig('server-client-scaling.png', format='png', dpi=1200)
#plt.show()

