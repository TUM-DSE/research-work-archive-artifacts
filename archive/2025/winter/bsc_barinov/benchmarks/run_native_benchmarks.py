#!/usr/bin/env python3

import glob
import json
import os
import subprocess
import time

SCRIPT_DIR = os.path.dirname(os.path.abspath(__file__))

with open(os.path.join(SCRIPT_DIR, "bench_config.json")) as f:
    config = json.load(f)

RUNS = config["runs"]
DATASET_SIZE = config["dataset_size"]

OUTPUT_FILE = os.path.join(SCRIPT_DIR, "native_execs.txt")
TESTS_DIR = os.path.join(SCRIPT_DIR, "polybench_tests")

gcc_base = [
    "gcc", "-O2", "-fno-stack-protector", "-w", "-mgeneral-regs-only",
    "-DSGT_CHANGES", "-DSGT_TIME", "-DDATA_TYPE_IS_INT", f"-D{DATASET_SIZE}",
    "-DPOLYBENCH_NO_FLUSH_CACHE",
]

polybench_src = os.path.join(SCRIPT_DIR, "./polybench.c")
polybench_inc = os.path.join(SCRIPT_DIR, ".")

with open(OUTPUT_FILE, "w") as out:
    for c_file in sorted(glob.glob(os.path.join(TESTS_DIR, "*.c"))):
        benchmark = os.path.splitext(os.path.basename(c_file))[0]
        binary = os.path.join(TESTS_DIR, f"{benchmark}.native")
        print(f"Processing {os.path.basename(c_file)}...")

        compile_cmd = gcc_base + [c_file, polybench_src, "-I", polybench_inc, "-o", binary]
        result = subprocess.run(compile_cmd, capture_output=True, text=True)
        if result.returncode != 0:
            line = f"{os.path.basename(c_file)}: COMPILATION FAILED\n{result.stderr}"
            print(line)
            out.write(line + "\n")
            continue

        times = []
        for i in range(RUNS):
            start = time.perf_counter()
            subprocess.run([binary], stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL)
            elapsed = time.perf_counter() - start
            times.append(elapsed)

        times_str = ",".join(f"{t:.6f}" for t in times)
        line = f"{os.path.basename(c_file)}: times={times_str}"
        print(f"{os.path.basename(c_file)}: avg={sum(times)/RUNS:.6f}s")
        out.write(line + "\n")

        # Cleanup
        try:
            os.remove(binary)
        except FileNotFoundError:
            pass

print(f"Done. Results saved to {OUTPUT_FILE}")
