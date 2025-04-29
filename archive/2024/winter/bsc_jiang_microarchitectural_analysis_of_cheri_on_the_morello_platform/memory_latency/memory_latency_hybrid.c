#include <cheriintrin.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <unistd.h>

#include "time.h"

// Taken from the Chips And Cheese MemoryLatency Benchmark:
// https://github.com/ChipsandCheese/Microbenchmarks/blob/master/MemoryLatency/MemoryLatency.c
int default_test_sizes[37] = {
    2,     4,     8,      12,     16,     24,     32,     48,    64,    96,
    128,   192,   256,    384,    512,    600,    768,    1024,  1536,  2048,
    3072,  4096,  5120,   6144,   8192,   10240,  12288,  16384, 24567, 32768,
    65536, 98304, 131072, 262144, 393216, 524288, 1048576};

int main(void) {
    long cache_line_size = 64;
    // printf("Page size: %ld B\nCache line size: %ld B\n", page_size,
    // cache_line_size);
    printf("size (kB),time (ns),\n");

    size_t pointer_size = sizeof(void *__capability);
    for (int i = 0; i < 37; ++i) {
        uint64_t array_size = default_test_sizes[i] * 1024;

        void *__capability ddc = cheri_ddc_get();
        uintptr_t virtual_pointer_array = (uintptr_t)malloc(array_size);
        if (!virtual_pointer_array) {
            fprintf(stderr,
                    "Allocation of pointer_array size: %ld kB failed.\n",
                    array_size / 1024);
            exit(EXIT_FAILURE);
        }
        void *__capability *__capability pointer_array =
            cheri_address_set(ddc, virtual_pointer_array);

        // Fill array with pointers pointing to own location
        uint32_t pointer_count = array_size / cache_line_size;
        uint32_t pointer_offset = cache_line_size / pointer_size;
        for (uint32_t i = 0; i < pointer_count; ++i) {
            size_t current_index = i * pointer_offset;
            pointer_array[current_index] = &pointer_array[current_index];
        }

        // Shuffling pointers to create circular list
        for (uint32_t i = pointer_count - 1; i > 0; --i) {
            uint32_t j = rand() % i;
            void *__capability tmp = pointer_array[j * pointer_offset];
            pointer_array[j * pointer_offset] =
                pointer_array[i * pointer_offset];
            pointer_array[i * pointer_offset] = tmp;
        }

        struct timespec start, end;
        clock_gettime(CLOCK_MONOTONIC, &start);

        // TODO: Implement heuristic for accurate benchmarks
        uint32_t iterations = 1000000;
        void *__capability current = pointer_array;
        for (uint32_t i = 0; i < iterations; ++i) {
            asm volatile("ldr %0, [%0]" : "+r"(current)::"memory");
        }

        clock_gettime(CLOCK_MONOTONIC, &end);

        double total_time = (end.tv_sec - start.tv_sec) * 1000000000ULL +
                            (end.tv_nsec - start.tv_nsec);
        double single_time = total_time / iterations;

        // printf("Array size: %ld B, time: %f nanoseconds\n", array_size,
        // single_time);

        printf("%ld,%f,\n", array_size / 1024, single_time);

        free((void *)virtual_pointer_array);
    }

    return 0;
}
