import matplotlib.pyplot as plt
import numpy as np
from scipy.interpolate import make_interp_spline
import matplotlib.pyplot as plt

# context size
x_axis = np.array([100, 300, 500, 1000, 1500, 2048, 4000])

# ms/token
#kv_reset = np.array([18.6,22.6,26.6,41,55, 73, 1460])
kv_reset = np.array([51.6,97.6,140.6,278,428.3, 599.5, 1304])
state_restore = np.array([252+195.3, 249.3+187.3, 280+207.3, 305+209.3, 341+228, 382+221, 521+260]) # avg save time + avg load time

# smooth out curves
#x_axis_new = np.linspace(x_axis.min(), x_axis.max(), 500)



#plt.title("LLM-OS overhead")
plt.title("Context switch duration by number of tokens\nLower is better ↓", fontsize=9, color="navy", weight="bold")
plt.xlabel("Number of tokens in each request")
plt.ylabel("Context switch duration (ms)")
plt.plot(x_axis, kv_reset, label = 'KV Recompute')
plt.plot(x_axis, state_restore, label = 'KV Save/Load (KV Cache)')
plt.legend()
plt.grid()
x_ticks = np.linspace(0, 4000, 5, dtype=int)
plt.xticks(x_ticks, (str(i) for i in x_ticks))

plt.savefig('context_switch_ctx_4096.png', format='png', dpi=1200)
#plt.show()

