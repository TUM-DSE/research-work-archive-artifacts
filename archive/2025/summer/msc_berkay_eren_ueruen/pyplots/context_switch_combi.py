import matplotlib.pyplot as plt
import numpy as np
from scipy.interpolate import make_interp_spline
import matplotlib.pyplot as plt

# context size
x_axis = np.array([128, 512, 1024, 2048, 3000, 3500, 4096])

# ms/token
#kv_reset = np.array([6,19.3,34.6,51.6,97,1170, 1460])
kv_reset = np.array([33,127,268,569,880,1062, 1460])
state_restore = np.array([228+161.6, 261+180.6,300+199.3, 600, 675, 474+250, 521+260]) # avg save time + avg load time

kv_reset_4096 = np.array([51.6,97.6,140.6,278,428.3, 599.5, 1304])
state_restore_4096 = np.array([252+195.3, 249.3+187.3, 280+207.3, 305+209.3, 341+228, 400+297, 521+260]) # avg save time + avg load time

# smooth out curves
#x_axis_new = np.linspace(x_axis.min(), x_axis.max(), 500)



#plt.title("LLM-OS overhead")
plt.title("Context switch duration with fully utilized KV cache\nLower is better ↓", fontsize=9, color="navy", weight="bold")
plt.xlabel("KV cache capacity")
plt.ylabel("Context switch duration (ms)")
plt.plot(x_axis, kv_reset, label = 'KV recompute')
plt.plot(x_axis, state_restore, label = 'State save/load')
plt.plot(x_axis, kv_reset_4096, label = 'KV recompute 4096')
plt.plot(x_axis, state_restore_4096, label = 'State save/load 4096')
plt.legend()
plt.grid()
x_ticks = np.linspace(0, 4000, 5, dtype=int)
plt.xticks(x_ticks, (str(i) for i in x_ticks))

plt.savefig('context_switch_combi.png', format='png', dpi=1200)
#plt.show()

