#pragma once

#include <unordered_map>
#include "llama.h"
#include "llama_bench_args.h"

#define CTX_SIZE 2048  // Also KV cache size
#define BATCH_SIZE 2048

#define MIN(a, b) (((a) < (b)) ? (a) : (b))
#define MAX(a, b) (((a) > (b)) ? (a) : (b))

#define KV_CACHE_LIMIT 2000
#define KV_CACHE_POLICY_LIMIT 2000

// just a really low priority for now, set this to a really high number
#define NO_PRIORITY 1000
enum Policy { save_load_full, save_load_kv, recompute };
extern Policy general_policy;
extern std::unordered_map<int, llama_lora_adapter *> lora_adapters;
extern bool use_llama_bench_args;
extern llama_model *model;
extern cmd_params_instance llama_bench_args;
