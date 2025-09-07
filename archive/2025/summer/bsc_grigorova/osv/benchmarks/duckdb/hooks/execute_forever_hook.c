#include <stdint.h>
#include <stddef.h>
#include "hook_ctx.h"

static void (*bpf_thread_flush)  (uint64_t, uint64_t, uint64_t) = (void*) 18;
static uint64_t (*bpf_queue_wait)         (uint64_t) = (void*) 19;
static uint64_t (*bpf_queue_wait_timeout) (uint64_t, uint64_t) = (void*) 20;
static uint64_t (*bpf_allocator_decay_delay)()= (void*) 21;
static uint64_t (*bpf_queue_dequeue)      (uint64_t, uint64_t)= (void*) 22;
static uint64_t (*bpf_task_execute)       (uint64_t)= (void*) 23;
static void (*bpf_task_deschedule)    (uint64_t)= (void*) 24;
static void (*bpf_task_reset)         (uint64_t)= (void*) 25;
static void (*bpf_allocator_thread_idle)() = (void*) 26;

enum TaskExecutionResult_u64 {
    TASK_FINISHED       = 0,
    TASK_NOT_FINISHED   = 1,
    TASK_ERROR          = 2,
    TASK_BLOCKED        = 3
};

// Do ONE iteration of the original loop body
uint64_t execute_forever(void *ctx_in, uint64_t ctx_len) {
    struct execute_forever_ctx *ctx = (struct execute_forever_ctx *)ctx_in;
    uint64_t sched = ctx->scheduler_ptr;
    uint64_t initial_flush_wait = 500000;
    uint64_t task = ctx->out_task_slot;

    if (ctx->supports_flush == 0) {
        bpf_queue_wait(sched);
    } else if (!bpf_queue_wait_timeout(sched, initial_flush_wait)) {
            bpf_thread_flush(sched, ctx->flush_threshold, ctx->requested_threads);

            uint64_t decay_delay = bpf_allocator_decay_delay(); 
            if (decay_delay == 0) {
                bpf_queue_wait(sched);
            } else {
                int64_t decay_us = (int64_t)decay_delay * 1000000ull - initial_flush_wait;
                    if (!bpf_queue_wait_timeout(sched, (uint64_t)decay_us)) {
                        bpf_allocator_thread_idle();
                        bpf_queue_wait(sched);
                    }
            }
        }

    if (bpf_queue_dequeue(sched, task)) {
        uint64_t res = bpf_task_execute(task);
        if (res == TASK_FINISHED || res == TASK_ERROR) {
            (void)bpf_task_reset(task);
        } else if (res == TASK_BLOCKED) {
            (void)bpf_task_deschedule(task);
            (void)bpf_task_reset(task);
        }
    }
    return 0;
}
