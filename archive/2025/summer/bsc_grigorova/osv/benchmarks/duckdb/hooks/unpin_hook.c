#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "hook_ctx.h"

static uint64_t (*handle_buffer)(uint64_t handle, uint64_t lock) = (void*)5;   
static uint64_t (*handle_decrement_readers)(uint64_t handle)= (void*)6;     
static void    (*verify_zero_readers)(uint64_t bm, uint64_t handle, uint64_t lock)= (void*)7;  
static bool    (*add_to_eviction_queue)(uint64_t bm, uint64_t handle, uint64_t lock) = (void*)8;  
static void    (*bpf_purge_block)(uint64_t bm, uint64_t handle) = (void*)9;  



uint64_t unpin(void *ctx_in, size_t ctx_len){
    struct unpin_hook_ctx *ctx = (struct unpin_hook_ctx*)ctx_in;
    uint64_t bm     = ctx->bm;
    uint64_t handle = ctx->handle;
    uint64_t lock = ctx->lock;
    bool purge = false;
    if (handle_buffer(handle, lock) !=0 ) {
        return 0;
    }
    uint64_t new_readers = handle_decrement_readers(handle);
    if (new_readers == 0) {
        verify_zero_readers(bm, handle, lock);
        purge = add_to_eviction_queue(bm, handle, lock);
    }

    if (purge) {
        return 0;
       // bpf_purge_block(bm, handle);
    } 
    return -1;
}