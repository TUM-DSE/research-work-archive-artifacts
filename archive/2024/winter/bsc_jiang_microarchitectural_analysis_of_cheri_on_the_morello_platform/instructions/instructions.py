import subprocess
import statistics
import pandas as pd
import sys
from enum import Enum

# Defines all testable instructions.
# Values correspond to CLI arguments passed to the test binaries.
class Instruction(Enum):
    LDR = 0
    LDR_CAP = 1
    STR = 2
    STR_CAP = 3
    LDAR = 4
    STLR = 5
    LDP = 6
    LDP_CAP = 7
    STP = 8
    STP_CAP = 9
    CVTD_TOCAP = 10
    CFHI = 11
    CTHI = 12
    CVTD_TOPTR = 13
    CVTP_TOCAP = 14
    CVTP_TOPTR = 15
    CVT_TOCAP = 16
    CVT_TOPTR = 17
    STR_INPLACE = 18
    STR_CAP_INPLACE = 19
    STP_INPLACE = 20
    STP_CAP_INPLACE = 21


class BinaryType(Enum):
    PURECAP = "instructions_purecap.out"
    NOCAP = "instructions_nocap.out"

class Metric(Enum):
    LATENCY = 0
    THROUGHPUT = 1

WARMUP_RUNS = 100
TEST_RUNS = 10
TEST_ITERATIONS = 1000000

# Defines all permutations of binary type, instruction and metric that will be benchmarked.
config = {
    BinaryType.PURECAP: [
        (Instruction.LDR, Metric.LATENCY),
        (Instruction.LDR_CAP, Metric.LATENCY),
        (Instruction.LDR, Metric.THROUGHPUT),
        (Instruction.LDR_CAP, Metric.THROUGHPUT),
        (Instruction.STR, Metric.THROUGHPUT),
        (Instruction.STR_CAP, Metric.THROUGHPUT),
        (Instruction.LDAR, Metric.THROUGHPUT),
        (Instruction.STLR, Metric.THROUGHPUT),
        (Instruction.LDP_CAP, Metric.LATENCY),
        (Instruction.LDP, Metric.THROUGHPUT),
        (Instruction.LDP_CAP, Metric.THROUGHPUT),
        (Instruction.STP, Metric.THROUGHPUT),
        (Instruction.STP_CAP, Metric.THROUGHPUT),
        (Instruction.CVTD_TOCAP, Metric.LATENCY),
        (Instruction.CVTD_TOCAP, Metric.THROUGHPUT),
        (Instruction.CFHI, Metric.LATENCY),
        (Instruction.CFHI, Metric.THROUGHPUT),
        (Instruction.CTHI, Metric.LATENCY),
        (Instruction.CTHI, Metric.THROUGHPUT),
        (Instruction.CVTD_TOPTR, Metric.LATENCY),
        (Instruction.CVTD_TOPTR, Metric.THROUGHPUT),
        (Instruction.CVTP_TOCAP, Metric.LATENCY),
        (Instruction.CVTP_TOCAP, Metric.THROUGHPUT),
        (Instruction.CVTP_TOPTR, Metric.LATENCY),
        (Instruction.CVTP_TOPTR, Metric.THROUGHPUT),
        (Instruction.CVT_TOCAP, Metric.LATENCY),
        (Instruction.CVT_TOCAP, Metric.THROUGHPUT),
        (Instruction.CVT_TOPTR, Metric.LATENCY),
        (Instruction.CVT_TOPTR, Metric.THROUGHPUT),
        (Instruction.STR_INPLACE, Metric.THROUGHPUT),
        (Instruction.STR_CAP_INPLACE, Metric.THROUGHPUT),
        (Instruction.STP_INPLACE, Metric.THROUGHPUT),
        (Instruction.STP_CAP_INPLACE, Metric.THROUGHPUT),
    ],
    BinaryType.NOCAP: [
        (Instruction.LDR, Metric.LATENCY),
        (Instruction.LDR, Metric.THROUGHPUT),
        (Instruction.STR, Metric.THROUGHPUT),
        (Instruction.LDAR, Metric.THROUGHPUT),
        (Instruction.STLR, Metric.THROUGHPUT),
        (Instruction.LDP, Metric.LATENCY),
        (Instruction.LDP, Metric.THROUGHPUT),
        (Instruction.STP, Metric.THROUGHPUT),
        (Instruction.STR_INPLACE, Metric.THROUGHPUT),
        (Instruction.STP_INPLACE, Metric.THROUGHPUT),
    ]
}

def check_program_runs(binary_path, instruction, metric):
    # Run with a single iteration to check if program works without errors
    cmd = [binary_path, str(instruction.value), str(metric.value), "10"]
    try:
        result = subprocess.run(cmd, capture_output=True, timeout=1)  # Short timeout
        return result.returncode == 0
    except:
        return False

def perf_stat_executable(binary_path, instruction, metric, iterations):
    command = ["perf", "stat", "-x,", "-e", "cycles",
               binary_path, str(instruction.value), str(metric.value), str(iterations)]
    return subprocess.run(command, capture_output=True, text=True)

def parse_cycles(output):
    for line in output.splitlines():
        if "cycles" in line:
            # With -x, format, the first field is the counter value
            fields = line.split(',')
            if len(fields) > 0:
                return int(fields[0])
    raise ValueError("cycles not found in perf output!")


def collect_cycles(binary_path, instruction, metric, iterations, runs):
    cycles_list = []

    for _ in range(runs):
        result = perf_stat_executable(binary_path, instruction, metric, iterations)
        cycles = parse_cycles(result.stderr)
        if result.stdout:
            print(result.stdout)
        cycles_list.append(cycles)

    return cycles_list

def main():
    # Create a list to store results
    results = []

    for binary_type, tests in config.items():
        binary_path = f"./{binary_type.value}"
        for instruction, metric in tests:
            # First check if the program runs without error for this configuration
            if not check_program_runs(binary_path, instruction, metric):
                results.append({
                    "Instruction": instruction.name,
                    "Mode": binary_type.name,
                    "Metric": metric.name,
                    "Value": "error"
                })
                continue

            # Collect data for this test
            warmup_cycles = collect_cycles(binary_path, instruction, metric, 0, WARMUP_RUNS)

            test_cycles = collect_cycles(binary_path, instruction, metric, TEST_ITERATIONS, TEST_RUNS)

            avg_warmup_cycles = statistics.mean(warmup_cycles)
            avg_test_cycles = statistics.mean(test_cycles)

            cycles_per_instruction = (avg_test_cycles - avg_warmup_cycles) / (TEST_ITERATIONS * 16)

            if metric == Metric.THROUGHPUT:
                # For throughput tests, we report instructions per cycle
                value = round(1 / cycles_per_instruction, 3)
            else:  # LATENCY
                # For latency tests, we report cycles per instruction
                value = round(cycles_per_instruction, 3)

            results.append({
                "Instruction": instruction.name,
                "Mode": binary_type.name,
                "Metric": metric.name,
                "Value": value
            })

    # Convert results to DataFrame
    df = pd.DataFrame(results)

    # Pivot to get the final format
    pivot_df = df.pivot_table(
        index=["Instruction", "Mode"],
        columns="Metric",
        values="Value",
        aggfunc='first'  # Take the first value in case of duplicates
    ).reset_index()

    # Reorder
    pivot_df = pivot_df[["Instruction", "Mode", "THROUGHPUT", "LATENCY"]]

    # Rename columns to match requirements
    pivot_df = pivot_df.rename(columns={"THROUGHPUT": "Throughput", "LATENCY": "Latency"})

    pivot_df = pivot_df.fillna("-")

    instructions_purecap_file = open("./instructions_purecap.tex", "w")
    instructions_nocap_file = open("./instructions_nocap.tex", "w")

    pivot_df[pivot_df['Mode'] == 'PURECAP'].drop(columns=['Mode']).to_latex(
        instructions_purecap_file,
        index=False,
        formatters={"Instruction": lambda x: f"\\verb|{x}|"},
        float_format="{:.3f}".format
    )

    pivot_df[pivot_df['Mode'] == 'NOCAP'].drop(columns=['Mode']).to_latex(
        instructions_nocap_file,
        index=False,
        formatters={"Instruction": lambda x: f"\\verb|{x}|"},
        float_format="{:.3f}".format
    )

if __name__ == "__main__":
    main()
