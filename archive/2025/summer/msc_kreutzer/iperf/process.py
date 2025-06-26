import math
import json
import sys

REPS = 5

def percentile(datapoints, p):
    i = p * (len(datapoints) - 1)
    i1 = math.floor(i)
    i2 = math.ceil(i)
    return 0.5 * (datapoints[i1] + datapoints[i2])

def upper_whisker(datapoints, up_quart, iqr):
    limit = up_quart + 1.5 * iqr
    for n in reversed(datapoints):
        if n < limit:
            return n

def lower_whisker(datapoints, low_quart, iqr):
    limit = low_quart - 1.5 * iqr
    for n in datapoints:
        if n > limit:
            return n

def process(mode, direction):
    datapoints = []
    for rep in range(REPS):
        path = f"outputs_iperf2/iperf3VMs_iperf3_1vms_{mode}_{direction}_tcp_-1B_rep{rep}/vm1.json"
        with open(path, 'r') as f:
            data = json.load(f)
        for timepoint in data["intervals"]:
            bps = timepoint["sum"]["bits_per_second"]
            if "sum_bidir_reverse" in data:
                bps += timepoint["sum_bidir_reverse"]["bits_per_second"]
            bps /= 1000000000
            datapoints.append(bps)
    datapoints.sort()
    median = percentile(datapoints, 0.5)
    low_quart = percentile(datapoints, 0.25)
    up_quart = percentile(datapoints, 0.75)
    iqr = up_quart - low_quart
    low_whisk = lower_whisker(datapoints, low_quart, iqr)
    up_whisk = upper_whisker(datapoints, up_quart, iqr)
    outliers = []
    for n in datapoints:
        if n < low_whisk or n > up_whisk:
            outliers.append(str(n))
    outliers_str = "\\\\\n".join(outliers)
    outliers_str += "\\\\"
    print(f"""\\addplot+ [boxplot prepared={{
    lower whisker={low_whisk},
    lower quartile={low_quart},
    median={median},
    upper quartile={up_quart},
    upper whisker={up_whisk},
}}] table [row sep=\\\\,y index=0,header=false] {{
{outliers_str}
}};""")

interfaces = [
    "vfio",
    "bridge",
    "bridge-vhost",
    "vmux-emu",
    "vmux-dpdk-e810",
    "vmux-med",
    "vmux-vdpdk"
]
interface_names = [
    "Qemu-pt",
    "Qemu-VirtIO",
    "Qemu-vhost",
    "vMux-emu-e1000",
    "vMux-emu-e810",
    "vMux-med-e810",
    "vMux-vDPDK"
]

print(r"""
\begin{tikzpicture}
\begin{axis}[
	boxplot/draw direction=y,
	width=.7\textwidth,
	ylabel={TCP throughput (Gbit/s)},
	xtick=\empty,
	name=axis,
	cycle list={deep1,deep2,deep3,deep4,deep5,deep6,deep7,deep8,deep9},
	legend style={draw=none,at={(1.03,0.5)},matrix anchor=west,cells={anchor=west},column sep=1ex},
	legend image code/.code={
\draw[fill] (0cm,-0.1cm) rectangle (0.6cm,0.1cm);
},
]
""")
for intf, name in zip(interfaces, interface_names):
    process(intf, sys.argv[1])
    print(f"\\addlegendentry{{{name}}}")

print(r"""
\end{axis}
\end{tikzpicture}
""")
