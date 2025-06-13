
import numpy as np
from scipy.interpolate import make_interp_spline
import matplotlib.pyplot as plt

# context size
x_axis = np.array([128, 512, 1024, 2048, 3000, 3500, 4096])

# ms/token
#kv_reset = np.array([6,19.3,34.6,51.6,97,1170, 1460])
kv_reset = np.array([33,127,268,569,880,1062, 1460])
state_restore = np.array([228+161.6, 261+180.6,300+199.3, 600, 675, 474+250, 521+260]) # avg save time + avg load time

# smooth out curves
#x_axis_new = np.linspace(x_axis.min(), x_axis.max(), 500)



#plt.title("LLM-OS overhead")
plt.title("Context switch duration with fully utilized KV cache\nLower is better ↓")
plt.xlabel("KV cache capacity (tokens)")
plt.ylabel("Context switch duration (ms)")
plt.plot(x_axis, kv_reset, label = 'KV recompute', color=('#57978e'), linewidth=2)
plt.plot(x_axis, state_restore, label = 'State save/load', color=('#b9a050'), linewidth=2)
plt.legend()
plt.grid()
x_ticks = np.linspace(0, 4000, 5, dtype=int)
plt.xticks(x_ticks, (str(i) for i in x_ticks))

plt.savefig('context_switch_by_ctx.png', format='png', dpi=1200)
#plt.show()

