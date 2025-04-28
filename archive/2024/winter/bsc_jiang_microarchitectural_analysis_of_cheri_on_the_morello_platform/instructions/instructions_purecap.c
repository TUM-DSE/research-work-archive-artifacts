#include <cheriintrin.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>

// External declarations for instruction measurement functions
// Throughput measurement functions
extern void ldr_throughput_loop(long iterations, long* tmp);
extern void str_throughput_loop(long iterations, long* tmp);
extern void ldar_throughput_loop(long iterations, long* tmp);
extern void stlr_throughput_loop(long iterations, long* tmp);
extern void ldr_cap_throughput_loop(long iterations, uintptr_t* tmp_cap);
extern void str_cap_throughput_loop(long iterations, uintptr_t* tmp_cap);
extern void ldp_throughput_loop(long iterations, uintptr_t* tmp_cap);
extern void ldp_cap_throughput_loop(long iterations, uintptr_t* tmp_cap);
extern void stp_throughput_loop(long iterations, uintptr_t* tmp_cap);
extern void stp_cap_throughput_loop(long iterations, uintptr_t* tmp_cap,
                                    uintptr_t* tmp1_cap);
extern void cvtd_tocap_throughput_loop(long iterations, long tmp);
extern void cfhi_throughput_loop(long iterations, uintptr_t* tmp);
extern void cthi_throughput_loop(long iterations, uintptr_t* tmp);
extern void cvtd_toptr_throughput_loop(long iterations, long tmp);
extern void cvtp_tocap_throughput_loop(long iterations, long tmp);
extern void cvtp_toptr_throughput_loop(long iterations, long tmp);
extern void cvt_tocap_throughput_loop(long iterations, long tmp);
extern void cvt_toptr_throughput_loop(long iterations, long tmp);
extern void str_inplace_throughput_loop(long iterations, long* tmp);
extern void str_cap_inplace_throughput_loop(long iterations, uintptr_t* tmp_cap);
extern void stp_inplace_throughput_loop(long iterations, uintptr_t* tmp_cap);
extern void stp_cap_inplace_throughput_loop(long iterations, uintptr_t* tmp_cap,
                                          uintptr_t* tmp1_cap);

// Latency measurement functions
extern void ldr_latency_loop(long iterations, long* tmp);
extern void ldr_cap_latency_loop(long iterations, intptr_t* tmp);
extern void ldp_cap_latency_loop(long iterations, intptr_t* tmp);
extern void cvtd_tocap_latency_loop(long iterations, long tmp);
extern void cfhi_latency_loop(long iterations, uintptr_t* tmp);
extern void cthi_latency_loop(long iterations, uintptr_t* tmp);
extern void cvtd_toptr_latency_loop(long iterations, long tmp);
extern void cvtp_tocap_latency_loop(long iterations, long tmp);
extern void cvtp_toptr_latency_loop(long iterations, long tmp);
extern void cvt_tocap_latency_loop(long iterations, long tmp);
extern void cvt_toptr_latency_loop(long iterations, long tmp);

// Instruction types to measure
enum Instruction {
    LDR,
    LDR_CAP,
    STR,
    STR_CAP,
    LDAR,
    STLR,
    LDP,
    LDP_CAP,
    STP,
    STP_CAP,
    CVTD_TOCAP,
    CFHI,
    CTHI,
    CVTD_TOPTR,
    CVTP_TOCAP,
    CVTP_TOPTR,
    CVT_TOCAP,
    CVT_TOPTR,
    STR_INPLACE,
    STR_CAP_INPLACE,
    STP_INPLACE,
    STP_CAP_INPLACE
};
enum Metric { LATENCY, THROUGHPUT };

// Arg 1: Instruction: Which instruction should be executed?
// Arg 2: Metric: Is the Latency or Throughput being measured?
// Arg 2: Iterations / 16: How many times (* 16) should the instruction should
// be executed?
int main(int argc, char* argv[]) {
    if (argc != 4) {
        goto cli_failure;
    }
    enum Instruction instruction = atoi(argv[1]);
    enum Metric metric = atoi(argv[2]);
    long iterations = atoi(argv[3]);
    // Temporary arrays used by measurement functions
    long tmp[] = {0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0};
    uintptr_t tmp_cap[] = {0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0};

    // Execute appropriate measurement function based on instruction type and metric
    if (instruction == LDR && metric == LATENCY) {
        ldr_latency_loop(iterations, (long*)tmp);
    } else if (instruction == LDR && metric == THROUGHPUT) {
        ldr_throughput_loop(iterations, (long*)tmp);
    } else if (instruction == STR && metric == THROUGHPUT) {
        str_throughput_loop(iterations, (long*)tmp);
    } else if (instruction == STR_CAP && metric == THROUGHPUT) {
        str_cap_throughput_loop(iterations, (uintptr_t*)tmp_cap);
    } else if (instruction == LDAR && metric == THROUGHPUT) {
        ldar_throughput_loop(iterations, (long*)tmp);
    } else if (instruction == STLR && metric == THROUGHPUT) {
        stlr_throughput_loop(iterations, (long*)tmp);
    } else if (instruction == LDR_CAP && metric == LATENCY) {
        intptr_t address;
        address = (intptr_t)&address;
        ldr_cap_latency_loop(iterations, &address);
    } else if (instruction == LDR_CAP && metric == THROUGHPUT) {
        ldr_cap_throughput_loop(iterations, &tmp_cap[0]);
    } else if (instruction == LDP && metric == THROUGHPUT) {
        ldp_throughput_loop(iterations, &tmp_cap[0]);
    } else if (instruction == LDP_CAP && metric == LATENCY) {
        intptr_t tmp[] = {0, 0};
        tmp[0] = (intptr_t)&tmp[0];
        ldp_cap_latency_loop(iterations, &tmp[0]);
    } else if (instruction == LDP_CAP && metric == THROUGHPUT) {
        ldp_cap_throughput_loop(iterations, &tmp_cap[0]);
    } else if (instruction == STP && metric == THROUGHPUT) {
        stp_throughput_loop(iterations, (uintptr_t*)tmp_cap);
    } else if (instruction == STP_CAP && metric == THROUGHPUT) {
        stp_cap_throughput_loop(iterations, &tmp_cap[0], &tmp_cap[1]);
    } else if (instruction == CVTD_TOCAP && metric == LATENCY) {
        cvtd_tocap_latency_loop(iterations, tmp[0]);
    } else if (instruction == CVTD_TOCAP && metric == THROUGHPUT) {
        cvtd_tocap_throughput_loop(iterations, tmp[0]);
    } else if (instruction == CFHI && metric == LATENCY) {
        cfhi_latency_loop(iterations, &tmp_cap[0]);
    } else if (instruction == CFHI && metric == THROUGHPUT) {
        cfhi_throughput_loop(iterations, &tmp_cap[0]);
    } else if (instruction == CTHI && metric == LATENCY) {
        cthi_latency_loop(iterations, &tmp_cap[0]);
    } else if (instruction == CTHI && metric == THROUGHPUT) {
        cthi_throughput_loop(iterations, &tmp_cap[0]);
    } else if (instruction == CVTD_TOPTR && metric == LATENCY) {
        cvtd_toptr_latency_loop(iterations, tmp[0]);
    } else if (instruction == CVTD_TOPTR && metric == THROUGHPUT) {
        cvtd_toptr_throughput_loop(iterations, tmp[0]);
    } else if (instruction == CVTP_TOCAP && metric == LATENCY) {
        cvtp_tocap_latency_loop(iterations, tmp[0]);
    } else if (instruction == CVTP_TOCAP && metric == THROUGHPUT) {
        cvtp_tocap_throughput_loop(iterations, tmp[0]);
    } else if (instruction == CVTP_TOPTR && metric == LATENCY) {
        cvtp_tocap_latency_loop(iterations, tmp[0]);
    } else if (instruction == CVTP_TOPTR && metric == THROUGHPUT) {
        cvtp_tocap_throughput_loop(iterations, tmp[0]);
    } else if (instruction == CVTD_TOPTR && metric == THROUGHPUT) {
        cvtd_toptr_throughput_loop(iterations, tmp[0]);
    } else if (instruction == CVT_TOCAP && metric == LATENCY) {
        cvt_tocap_latency_loop(iterations, tmp[0]);
    } else if (instruction == CVT_TOCAP && metric == THROUGHPUT) {
        cvt_tocap_throughput_loop(iterations, tmp[0]);
    } else if (instruction == CVT_TOPTR && metric == LATENCY) {
        cvt_tocap_latency_loop(iterations, tmp[0]);
    } else if (instruction == CVT_TOPTR && metric == THROUGHPUT) {
        cvt_tocap_throughput_loop(iterations, tmp[0]);
    } else if (instruction == STR_INPLACE && metric == THROUGHPUT) {
        str_inplace_throughput_loop(iterations, (long*)tmp);
    } else if (instruction == STR_CAP_INPLACE && metric == THROUGHPUT) {
        str_cap_inplace_throughput_loop(iterations, (uintptr_t*)tmp_cap);
    } else if (instruction == STP_INPLACE && metric == THROUGHPUT) {
        stp_inplace_throughput_loop(iterations, (uintptr_t*)tmp_cap);
    } else if (instruction == STP_CAP_INPLACE && metric == THROUGHPUT) {
        stp_cap_inplace_throughput_loop(iterations, &tmp_cap[0], &tmp_cap[1]);
    }
    else {
        goto cli_failure;
    }
    return 0;

cli_failure:
    fprintf(stderr, "Invalid/Unimplemented command line arguments\n");
    exit(EXIT_FAILURE);
}
