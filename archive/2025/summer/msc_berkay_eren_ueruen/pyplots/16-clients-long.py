import matplotlib.pyplot as plt
import numpy as np
from scipy.interpolate import make_interp_spline
import matplotlib.pyplot as plt

# context size
x_axis = np.array([641, 960, 1211, 1391, 1591])

# ms/token
vm_cpu = np.array([1.02924, 0.79405, 0.663664, 0.573927, 0.525171])

# smooth out curves
#x_axis_new = np.linspace(x_axis.min(), x_axis.max(), 500)

plt.title("Average Throughput by the Total Number of Generated\nTokens with 16 Clients (Same Priority Requests)")

#plt.title("LLM-OS overhead")
plt.xlabel("Total Number of Tokens Generated for 16 Clients")
plt.ylabel("Throughput (tokens per second)")
a = plt.plot(x_axis, vm_cpu, label="CPU Inference", color=('#b9a050'))

plt.ylim([0,2])
plt.legend()

plt.savefig('16-clients-long.png', format='png', dpi=1200)
#plt.show()

