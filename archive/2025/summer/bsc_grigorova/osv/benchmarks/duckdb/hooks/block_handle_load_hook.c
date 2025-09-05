#include <stdint.h>
#include <stddef.h>

#include "hook_ctx.h"
static uint64_t (*block_is_loaded) (uint64_t block_ptr) = (void *)10;
static uint64_t (*id_less_than_max) (uint64_t block_ptr, uint64_t reusable_buffer_ptr, uint64_t buffer_ptr) = (void *)11;
static uint64_t (*write_temprorary_file) (uint64_t block_ptr, uint64_t reusable_buffer_ptr, uint64_t buffer_ptr) = (void *)12;


uint64_t load(void *ctx_in, size_t ctx_len) {
    struct block_handle_load_ctx *ctx = (struct block_handle_load_ctx*)ctx_in;
    uint64_t block_ptr = ctx->block_ptr;
    uint64_t buffer_ptr = ctx->buffer_ptr;
    uint64_t reusable_buffer_ptr = ctx->reusable_buffer_ptr;
    if(block_is_loaded(block_ptr) == 0){
        return 0;
    }
    if(id_less_than_max(block_ptr, reusable_buffer_ptr, buffer_ptr) == 0){
        return 1;
    } else {
        return write_temprorary_file(block_ptr, reusable_buffer_ptr, buffer_ptr);
    }
}