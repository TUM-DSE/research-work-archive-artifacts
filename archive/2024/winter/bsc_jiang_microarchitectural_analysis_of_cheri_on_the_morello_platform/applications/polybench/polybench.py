import subprocess
import os
import csv
import sys
from enum import Enum
import math


class Benchmark(Enum):
    TWO_MM = "2mm"
    THREE_MM = "3mm"
    ADI = "adi"
    ATAX = "atax"
    BICG = "bicg"
    CHOLESKY = "cholesky"
    CORRELATION = "correlation"
    COVARIANCE = "covariance"
    DOITGEN = "doitgen"
    DURBIN = "durbin"
    DYNPROG = "dynprog"
    FDTD_2D = "fdtd-2d"
    FDTD_APML = "fdtd-apml"
    FLOYD_WARSHALL = "floyd-warshall"
    GEMM = "gemm"
    GEMVER = "gemver"
    GESUMMV = "gesummv"
    GRAMSCHMIDT = "gramschmidt"
    JACOBI_1D_IMPER = "jacobi-1d-imper"
    JACOBI_2D_IMPER = "jacobi-2d-imper"
    LU = "lu"
    LUDCMP = "ludcmp"
    MVT = "mvt"
    REG_DETECT = "reg_detect"
    SEIDEL_2D = "seidel-2d"
    SYMM = "symm"
    SYR2K = "syr2k"
    SYRK = "syrk"
    TRISOLV = "trisolv"
    TRMM = "trmm"

class Capability(Enum):
    NOCAP = "_nocap"
    PURECAP = "_purecap"

WARUMUP_ITERATIONS = 10
BASE_ITERATIONS = 5
MIN_TEST_TIME = 2

os.chdir("./output")

writer = csv.writer(sys.stdout)
writer.writerow(["benchmark", "capability", "iteration", "time"])

for bench in Benchmark:
    for type in Capability:
        warmup_execution_time = 1
        for i in range(WARUMUP_ITERATIONS):
            warmup_execution_time = float(
                subprocess.run([f"./{bench.value}{type.value}"], capture_output=True, text=True).stdout
            )

        iterations = max(BASE_ITERATIONS, math.ceil(MIN_TEST_TIME / warmup_execution_time))

        for i in range(iterations):
            execution_time = float(
                subprocess.run([f"./{bench.value}{type.value}"], capture_output=True, text=True).stdout
            )
            writer.writerow([bench.value, type.name, i, execution_time])
