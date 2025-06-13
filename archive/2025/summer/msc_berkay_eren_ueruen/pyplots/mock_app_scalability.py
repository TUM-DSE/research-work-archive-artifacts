import matplotlib.pyplot as plt
import numpy as np
from scipy.interpolate import make_interp_spline
import matplotlib.pyplot as plt

def add_labels(x,y, x_offset, y_offset):
    for i in range(len(x)):
        plt.text(i + x_offset, y[i] + y_offset, y[i], ha = 'center')

x = ['256', '512', '1024', '2048']
x_axis = np.arange(len(x))

y_axis = [0, 5, 10, 15, 20]


no_opt  = [16, 8, 4, 2]
optim = [20, 12, 6, 3]

plt.xticks(x_axis, x)
plt.yticks(y_axis, (str(i) for i in y_axis))
plt.title("Higher is better ↑", fontsize=9, color="navy", weight="bold")
plt.xlabel("Context size (tokens)")
plt.ylabel("# apps active")
plt.bar(x_axis - 0.2, no_opt, 0.4, label = 'No mem. optimizations', edgecolor = "black")
plt.bar(x_axis + 0.2, optim, 0.4, label = 'Mem. optimizations', edgecolor = "black")
add_labels(x, no_opt, -0.2, 0.2)
add_labels(x, optim, 0.2, 0.2)
plt.legend()
#plt.grid()

plt.savefig('mock_app_scalability.png', format='png', dpi=1200)
plt.show()

