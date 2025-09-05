#ifndef HOOK_CTX_H
#define HOOK_CTX_H

struct unpin_hook_ctx {
    uint64_t bm; 
    uint64_t handle; 
    uint64_t lock; 
};

struct hash_hook_ctx{
    const uint8_t *data;
    uint64_t      len;
};

struct arena_allocator_allocate_ctx {
    uint64_t arena_allocator_ptr;
	uint64_t min_size;
	uint64_t arena_max_capacity;
};

struct block_handle_load_ctx {
    uint64_t block_ptr; 
    uint64_t buffer_ptr; 
    uint64_t reusable_buffer_ptr; 
};

struct schedule_task_ctx {
    unsigned long long scheduler_ptr;
    unsigned long long token_ptr;
    unsigned long long task_ptr;
};

struct update_used_memory_ctx {
    uint64_t  tag_idx;           
    int64_t  size;              
    uint64_t threshold;         
    uint64_t  cache_count;       
    uint64_t  tags_len;          
    uint64_t  total_index;       

    uint64_t cache_ptr;       
    uint64_t memory_usage_ptr;       
};

struct execute_forever_ctx {
    uint64_t scheduler_ptr;       // TaskScheduler*
    uint64_t supports_flush;      // 1/0
    uint64_t flush_threshold;     // allocator_flush_threshold (or -1 to use default)
    uint64_t requested_threads;   // requested_thread_count.load()
    uint64_t out_task_slot;       // &task (shared_ptr<Task>*)
};

#endif // HOOK_CTX_H