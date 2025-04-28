#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>

// External declarations for instruction measurement functions
extern void ldr_throughput_loop(long iterations, long* tmp);
extern void str_throughput_loop(long iterations, long* tmp);
extern void ldar_throughput_loop(long iterations, long* tmp);
extern void stlr_throughput_loop(long iterations, long* tmp);
extern void ldp_throughput_loop(long iterations, long* tmp);
extern void stp_throughput_loop(long iterations, long* tmp);
extern void str_inplace_throughput_loop(long iterations, long* tmp);
extern void stp_inplace_throughput_loop(long iterations, long* tmp);

extern void ldr_latency_loop(long iterations, long* tmp);
extern void ldp_latency_loop(long iterations, long* tmp);

// Instruction types to measure, with gaps (_N) to match Python enum values
enum Instruction {
    LDR,
    _1,
    STR,
    _3,
    LDAR,
    STLR,
    LDP,
    _7,
    STP,
    _9,
    _10,
    _11,
    _12,
    _13,
    _14,
    _15,
    _16,
    _17,
    STR_INPLACE,
    _19,
    STP_INPLACE
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
    // Temporary array used by measurement functions
    long tmp[] = {0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0};

    // Execute appropriate measurement function based on instruction type and
    // metric
    if (instruction == LDR && metric == LATENCY) {
        ldr_latency_loop(iterations, (long*)tmp);
    } else if (instruction == LDR && metric == THROUGHPUT) {
        ldr_throughput_loop(iterations, (long*)tmp);
    } else if (instruction == STR && metric == THROUGHPUT) {
        str_throughput_loop(iterations, (long*)tmp);
    } else if (instruction == LDAR && metric == THROUGHPUT) {
        ldar_throughput_loop(iterations, (long*)tmp);
    } else if (instruction == STLR && metric == THROUGHPUT) {
        stlr_throughput_loop(iterations, (long*)tmp);
    } else if (instruction == LDP && metric == LATENCY) {
        long ldp_tmp[] = {0, 0};
        ldp_tmp[0] = (long)&ldp_tmp[0];
        ldp_latency_loop(iterations, (long*)ldp_tmp);
    } else if (instruction == LDP && metric == THROUGHPUT) {
        ldp_throughput_loop(iterations, (long*)tmp);
    } else if (instruction == STP && metric == THROUGHPUT) {
        stp_throughput_loop(iterations, (long*)tmp);
    } else if (instruction == STR_INPLACE && metric == THROUGHPUT) {
        str_inplace_throughput_loop(iterations, (long*)tmp);
    } else if (instruction == STP_INPLACE && metric == THROUGHPUT) {
        stp_inplace_throughput_loop(iterations, (long*)tmp);
    } else {
        goto cli_failure;
    }
    return 0;

cli_failure:
    fprintf(stderr, "Invalid/Unimplemented command line arguments\n");
    exit(EXIT_FAILURE);
}
