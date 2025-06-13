import matplotlib.pyplot as plt
import numpy as np
from scipy.interpolate import make_interp_spline
import matplotlib.pyplot as plt

# context size
x_axis = np.array([16, 32, 64, 128, 256, 512, 1024])

# ms/token
native = np.array([2, 3, 5, 7, 9, 12, 15])
virt = np.array([4, 5, 7, 9, 11, 14, 17])
trust = np.array([5, 6, 8, 10, 12, 15, 18])

# smooth out curves
x_axis_new = np.linspace(x_axis.min(), x_axis.max(), 500)

native_spline = make_interp_spline(x_axis, native)
native_smooth = native_spline(x_axis_new)

virt_spline = make_interp_spline(x_axis, virt)
virt_smooth = virt_spline(x_axis_new)

trust_spline = make_interp_spline(x_axis, trust)
trust_smooth = trust_spline(x_axis_new)

#plt.title("LLM-OS overhead")
plt.title("Lower is better ↓", fontsize=9, color="navy", weight="bold")
plt.xlabel("Context size (tokens)")
plt.ylabel("Inference latency (ms/token)")
plt.plot(x_axis_new, native_smooth, label = 'Native')
plt.plot(x_axis_new, virt_smooth, label = 'Virtualized')
plt.plot(x_axis_new, trust_smooth, label = 'LLM-OS')
plt.legend()
plt.grid()
x_ticks = np.linspace(0, 1024, 5, dtype=int)
plt.xticks(x_ticks, (str(i) for i in x_ticks))

plt.savefig('mock_overhead.png', format='png', dpi=1200)
plt.show()

