import matplotlib.pyplot as plt
import numpy as np
from scipy.interpolate import make_interp_spline
import matplotlib.pyplot as plt

# context size
x = ['Linux Kernel', 'Godot', 'gcc', 'gdb']
x_axis = np.arange(len(x))


idle  = [15 * 60, 10 * 60, 12 * 60, 11 * 60]
infer = [17 * 60, 11 * 60, 14 * 60, 12 * 60]

plt.xticks(x_axis, x)
plt.title("Lower is better ↓", fontsize=9, color="navy", weight="bold")
plt.xlabel("Compilation task")
plt.ylabel("Time [s]")
plt.bar(x_axis - 0.2, idle, 0.4, label = 'LLM-OS idle', edgecolor = "black")
plt.bar(x_axis + 0.2, infer, 0.4, label = 'LLM-OS inference', edgecolor = "black")
plt.legend()
#plt.grid()

plt.savefig('mock_impact.png', format='png', dpi=1200)
plt.show()

