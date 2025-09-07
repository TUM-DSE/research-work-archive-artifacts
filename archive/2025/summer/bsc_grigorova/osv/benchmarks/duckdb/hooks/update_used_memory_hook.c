#include <stdint.h>
#include <stddef.h>
#include "hook_ctx.h"

static uint64_t (*bpf_get_estimated_cpu_id)() = (void*)15;   
static uint64_t (*bpf_fetch_add)(uint64_t addr, uint64_t size) = (void*)16;   
static uint64_t (*bpf_exchange)(uint64_t addr, uint64_t new_val) = (void*)17;   

static inline uint64_t abs_u64(int64_t v) {
    return (v < 0) ? (uint64_t)(-v) : (uint64_t)v;
}

static inline uint64_t addr_elem_u64(uint64_t base, uint64_t index) {
    // 8-byte elements
    return base + (index * 8u);
}

uint64_t update_used_memory(void* ctx_in, size_t ctx_len){
    struct update_used_memory_ctx *ctx = (struct update_used_memory_ctx *)ctx_in;

    uint64_t  tag_idx     = ctx->tag_idx;
    int64_t  size        = ctx->size;
    uint64_t threshold   = ctx->threshold;
    uint64_t  cache_count = ctx->cache_count;
    uint64_t  tags_len    = ctx->tags_len;
    uint64_t  total_idx   = ctx->total_index;

    uint64_t caches_base = ctx->cache_ptr; 
    uint64_t usage_base  = ctx->memory_usage_ptr;  

    if (abs_u64(size) < threshold) {
        uint64_t cache_idx = (bpf_get_estimated_cpu_id() % (uint64_t)cache_count);

        uint64_t tag_offset = cache_idx * (uint64_t)tags_len + (uint64_t)tag_idx;
        uint64_t tag_addr   = addr_elem_u64(caches_base, tag_offset);

        int64_t new_tag     = (int64_t)bpf_fetch_add(tag_addr, (uint64_t)size) + size;

        if (abs_u64(new_tag) >= threshold) {
            int64_t tag_size = (int64_t)bpf_exchange(tag_addr, (uint64_t)0);
            uint64_t usage_tag_addr = addr_elem_u64(usage_base, (uint64_t)tag_idx);
            (void)bpf_fetch_add(usage_tag_addr, (uint64_t)tag_size);
        }

        uint64_t total_offset = cache_idx * (uint64_t)tags_len + (uint64_t)total_idx;
        uint64_t total_addr   = addr_elem_u64(caches_base, total_offset);

        int64_t old_tot   = (int64_t)bpf_fetch_add(total_addr, (uint64_t)size);
        int64_t new_tot   = old_tot + size;

        if (abs_u64(new_tot) >= threshold) {
            int64_t cached_total = (int64_t)bpf_exchange(total_addr, (uint64_t)0);
            uint64_t usage_total_addr = addr_elem_u64(usage_base, (uint64_t)total_idx);
            (void)bpf_fetch_add(usage_total_addr, (uint64_t)cached_total);
        }
    } else {

        uint64_t usage_tag_addr   = addr_elem_u64(usage_base,  (uint64_t)tag_idx);
        uint64_t usage_total_addr = addr_elem_u64(usage_base,  (uint64_t)total_idx);
        (void)bpf_fetch_add(usage_tag_addr,   (uint64_t)size);
        (void)bpf_fetch_add(usage_total_addr, (uint64_t)size);
    }
    return 0;
}