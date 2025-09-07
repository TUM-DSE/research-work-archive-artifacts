#include <stdint.h>
#include <stddef.h>
#include "hook_ctx.h"
static uint64_t (*get_head_max_size) (uint64_t arena_allocator_ptr) = (void *)13;
static void (*arena_prepend_chunk) (uint64_t arena_allocator_ptr, uint64_t capacity) = (void *)14;


struct HookCtx {
    uint64_t arena_allocator_ptr;
	uint64_t min_size;
	uint64_t arena_max_capacity;
};

uint64_t allocate(void *ctx_in, size_t ctx_len) {
    struct arena_allocator_allocate_ctx *ctx = (struct arena_allocator_allocate_ctx*)ctx_in;
    uint64_t arena_allocator_ptr = ctx->arena_allocator_ptr;
    uint64_t min_size = ctx -> min_size;
    uint64_t ARENA_ALLOCATOR_MAX_CAPACITY = ctx-> arena_max_capacity;
    uint64_t capacity;
    uint64_t head_max_size = get_head_max_size(arena_allocator_ptr);
    if (head_max_size <0){
		capacity = 2048;
	} else {
		capacity = head_max_size;
	}
	// capacity of the previous block can be bigger than the max capacity if we allocate len > max capacity
	// for new blocks - try to set it back to the max capacity
	if (capacity > ARENA_ALLOCATOR_MAX_CAPACITY) {
		capacity = ARENA_ALLOCATOR_MAX_CAPACITY;
	}
	// if we are below the max capacity - double the size of the block
	if (capacity < ARENA_ALLOCATOR_MAX_CAPACITY) {
		capacity *= 2;
	}
	// we double the size until we can fit `len`
	// this is generally only relevant if len is very large
	while (capacity < min_size) {
		capacity *= 2;
	}

	arena_prepend_chunk(arena_allocator_ptr, capacity);

    return capacity;
}