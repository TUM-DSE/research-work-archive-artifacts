import pandas as pd
import matplotlib as mpl
import matplotlib.pyplot as plt

# plt.rcParams['text.usetex'] = True
plt.rcParams['text.latex.preamble'] = r'\renewcommand{\mathdefault}[1]{}'
mpl.rcParams.update({
    "font.family": "serif",
    "font.size": 10,
})

instrs_integer = [
    "adc", "adcs", "add", "adds", "adr", "adrp", "and", "ands", "asrv", "bfm",
    "bic", "bl", "ccmn", "ccmp", "csel", "csinc", "csinv", "csneg", "clz",
    "cls", "eon", "eor", "extr", "lslv", "lsrv", "madd", "movk", "movn", "movz",
    "msub", "orn", "orr", "rbit", "rev", "rev16", "rev32", "rorv", "sbc",
    "sbcs", "sbfm", "sdiv", "smaddl", "smulh", "smsubl", "sub", "subs", "ubfm",
    "udiv", "umaddl", "umulh", "umsubl"
]

instrs_branch = [
    "b", "bcc", "blr", "br", "cbnz", "cbz", "ret", "tbnz", "tbz"
]

instrs_memory = [
    "ldar", "ldarb", "ldarh", "ldp", "ldpsw", "ldr", "ldrb", "ldrh",
    "ldrsb", "ldrsh", "ldrsw", "ldur", "stp", "str", "strb", "strh"
]

group_labels = ["Integer", "Branch", "Memory"]
group_instrs = [instrs_integer, instrs_branch, instrs_memory]
metrics = ["lifting_time", "instruction_count", "block_count"]

def title_case(snake_str):
    return snake_str.replace("_", " ").title()

auto = pd.read_csv("benchmark_instructions.csv")
manual = pd.read_csv("benchmark_instructions_manual.csv")

auto.set_index("opcode", inplace=True)
manual.set_index("opcode", inplace=True)

ratios = {metric: [[] for _ in group_labels] for metric in metrics}

for i, instr_group in enumerate(group_instrs):
    for opcode in instr_group:
        if opcode in auto.index and opcode in manual.index:
            for metric in metrics:
                air = auto.at[opcode, metric]
                man = manual.at[opcode, metric]
                if man != 0:
                    ratios[metric][i].append(air / man)

all_values = [
    val for metric_data in ratios.values() for group in metric_data for val in group if val > 0
]
ymin = min(all_values)
ymax = max(all_values)

for metric in metrics:
    plt.figure(figsize=(3.1, 5))
    plt.boxplot(ratios[metric], vert=True)
    plt.title(f"{title_case(metric)}")
#     plt.xlabel("Ratio (log scale)")
    plt.yscale("log")
    plt.ylim(ymin, ymax)
    plt.xticks(ticks=[1, 2, 3], labels=group_labels)
    plt.grid(True, axis='y')
    plt.tight_layout()
    plt.subplots_adjust(bottom=0.1)
    plt.savefig(f"plot_{metric}.pdf")
