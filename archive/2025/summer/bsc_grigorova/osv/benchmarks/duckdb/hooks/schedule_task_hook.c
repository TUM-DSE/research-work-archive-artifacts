#include <stdint.h>
#include <stddef.h>
#include "hook_ctx.h"
static void (*bpf_enqueue)(uint64_t scheduler_ptr, uint64_t token_ptr, uint64_t task_ptr) = (void *)4;



void schedule_task(void *ctx, size_t ctx_len){
    
    struct schedule_task_ctx *c = (struct schedule_task_ctx *)ctx;

    // Extract the real pointer values (u64 → pointer)
    unsigned long long sched = c->scheduler_ptr;
    unsigned long long tok   = c->token_ptr;
    unsigned long long tsk   = c->task_ptr;
    bpf_enqueue(sched,tok,tsk);
  //  bpf_enqueue((uint64_t )(ctx ),(uint64_t )(ctx + sizeof(uint64_t)), (uint64_t )(ctx + sizeof(uint64_t)*2));
}