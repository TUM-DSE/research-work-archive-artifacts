#include <stdint.h>
#include <stddef.h>
#include "hook_ctx.h"

static void (*bpf_memcpy)(void *ctx, size_t ctx_len) = (void *)2;


static inline uint64_t MurmurHash64(uint64_t x) {
    x ^= x >> 32;
    x *= 0xd6e8feb86659fd93ULL;
    x ^= x >> 32;
    x *= 0xd6e8feb86659fd93ULL;
    x ^= x >> 32;
    return x;
}


uint64_t hash(void *ctx_in, size_t ctx_len) {
    struct hash_hook_ctx *ctx = (struct hash_hook_ctx*)ctx_in;
    const uint8_t *data = ctx->data;
    uint64_t len = ctx->len;

    if (len <= 12) {
        uint64_t h = 0xe17a1465U ^ (len * 0xc6a4a7935bd1e995U);
        const uint8_t not_empty = (len != 0);

        uint64_t block = 0;
       struct { void *dst, *src; uint64_t n; } args1 = {
            &block, (void *)data, 8
        };
        bpf_memcpy(&args1, sizeof(args1));

        h ^= block;
        h *= 0xd6e8feb86659fd93ULL * not_empty + (1 - not_empty);

         uint64_t hr = 0;
          struct { void *dst, *src; uint64_t n; } args_tail = {
              &hr, (void *)(data + 8), 4U
          };
          bpf_memcpy(&args_tail, sizeof(args_tail));
          const uint8_t not_a_nop = (len > 8);
          h ^= hr;
          h *= 0xd6e8feb86659fd93ULL * not_a_nop + (1 - not_a_nop);


        
        h = MurmurHash64(h);
        return h;
    }

    uint64_t h = 0xe17a1465U ^ (len * 0xc6a4a7935bd1e995U);


    uint64_t end = len & ~7ULL;
    for (uint64_t off = 0; off < end; off += 8) {
        uint64_t block;
        struct { void *dst, *src; uint64_t n; } args2 = {
           &block, (void *)(data + off), 8
        };
        bpf_memcpy(&args2, sizeof(args2));
   
        h ^= block;
        h *= 0xd6e8feb86659fd93ULL;
    }

    uint64_t hr = 0;
    struct { void *dst, *src; uint64_t n; } args3 = {
        &hr, (void *)(data + end), len & 7ULL
    };
    bpf_memcpy(&args3, sizeof(args3));
      const uint8_t not_nop = (len & 7ULL) != 0;
    h ^= hr;
    h *= 0xd6e8feb86659fd93ULL * not_nop + (1 - not_nop);


    h = MurmurHash64(h);
    return h;
}

