#include <stdio.h>
#include <stdint.h>
#include <string.h>


#include "ubpf_runtime.h"
#include "duckdb/parallel/task_scheduler.hpp"
#include "duckdb/storage/standard_buffer_manager.hpp"
#include "duckdb/parallel/task.hpp"
#include "duckdb/parallel/concurrentqueue.hpp"
#include "duckdb/common/allocator.hpp"
#include "duckdb/storage/buffer/buffer_pool.hpp"
#include "duckdb/storage/buffer/buffer_handle.hpp"

#include "duckdb/storage/buffer/block_handle.hpp"
#include "duckdb/storage/arena_allocator.hpp"


using namespace duckdb;


// Define the bpf_print function
uint64_t bpf_print(char* a) {
    const char* buf = (const char*)a;
    if (!buf) return -1;
    printf("[eBPF] %s\n", buf);
    return 0;
}


uint64_t bpf_memcpy(void *ctx, size_t ctx_len) {
    struct memcpy_args {
        void *dst;
        void *src;
        uint64_t n;
    };
    // Cast ctx into your named struct
    struct memcpy_args *args = (struct memcpy_args *)ctx;
    memcpy(args->dst, args->src, args->n);
    return 0;  
}

uint64_t bpf_load(const void *src) {
    uint64_t x = 0;
    memcpy(&x, src, 8);     // host memcpy, always safe
    return x;
}


namespace duckdb{
using BlockLock = std::unique_lock<std::mutex>;

uint64_t bpf_task_enqueue(uint64_t sched_ptr, uint64_t token_ptr, uint64_t task_ptr) {
    auto token   = reinterpret_cast<ProducerToken*>((uintptr_t)token_ptr );
    auto scheduler   = reinterpret_cast<TaskScheduler*>( (uintptr_t)sched_ptr );
    auto task  = reinterpret_cast<shared_ptr<Task>*>( (uintptr_t)task_ptr );
    scheduler->EnqueueHelper(*token, std::move(*task));
    return 0;
}

uint64_t handle_buffer(uint64_t handle_ptr, uint64_t lock_ptr){    
    auto handle_ptr1 = reinterpret_cast<duckdb::shared_ptr<duckdb::BlockHandle>*>((uintptr_t)handle_ptr);
    auto &handle = *handle_ptr1;
    auto lock = reinterpret_cast<BlockLock*>((uintptr_t)lock_ptr);
    if (!handle->GetBuffer(*lock) || handle->GetBufferType() == FileBufferType::TINY_BUFFER) {
		return -1;
	}
    return 0;

}

uint64_t handle_decrement_readers(uint64_t handle_ptr){
    auto handle_ptr1 = reinterpret_cast<duckdb::shared_ptr<duckdb::BlockHandle>*>((uintptr_t)handle_ptr);
    auto &handle = *handle_ptr1;
    auto new_readers = handle->DecrementReaders(); 
    return new_readers;
}

void verify_zero_readers(uint64_t bm_ptr, uint64_t handle_ptr, uint64_t lock_ptr){
    auto handle_ptr1 = reinterpret_cast<duckdb::shared_ptr<duckdb::BlockHandle>*>((uintptr_t)handle_ptr);
    auto &handle = *handle_ptr1;
    auto bm = reinterpret_cast<duckdb::StandardBufferManager*>((uintptr_t)bm_ptr);
    auto lock = reinterpret_cast<BlockLock*>(static_cast<uintptr_t>(lock_ptr));
    bm->VerifyZeroReaders(*lock, handle);
}


bool add_to_eviction_queue(uint64_t bm_ptr, uint64_t handle_ptr, uint64_t lock_ptr) {
    auto handle_ptr1 = reinterpret_cast<duckdb::shared_ptr<duckdb::BlockHandle>*>((uintptr_t)handle_ptr);
    auto &handle = *handle_ptr1;
    auto bm = reinterpret_cast<duckdb::StandardBufferManager*>((uintptr_t)bm_ptr);
    bool purge = false;
    auto lock = reinterpret_cast<BlockLock*>(static_cast<uintptr_t>(lock_ptr));
    if(handle->MustAddToEvictionQueue()){
        purge = bm -> buffer_pool.AddToEvictionQueue(handle);
    } else{
        handle->Unload(*lock);
    }
    return purge;
}

void bpf_purge_block(uint64_t bm_ptr, uint64_t handle_ptr){
    auto handle_ptr1 = reinterpret_cast<duckdb::shared_ptr<duckdb::BlockHandle>*>((uintptr_t)handle_ptr);
    auto &handle = *handle_ptr1;
    auto bm = reinterpret_cast<duckdb::StandardBufferManager*>((uintptr_t)bm_ptr);
    bm -> PurgeQueue(*handle);
}

uint64_t block_is_loaded(uint64_t block_ptr){
    auto block = reinterpret_cast<duckdb::BlockHandle*>((uintptr_t)block_ptr);
    if (block->GetState() == duckdb::BlockState::BLOCK_LOADED){
    block->IncrementReaders();
    return 0;
    }
    return -1;
}

uint64_t id_less_than_max (uint64_t block_ptr, uint64_t reusable_buffer_ptr, uint64_t buffer_ptr){
    auto block = reinterpret_cast<duckdb::BlockHandle*>((uintptr_t)block_ptr);
    auto reusable_buffer = reinterpret_cast<duckdb::unique_ptr<duckdb::FileBuffer>*>((uintptr_t) reusable_buffer_ptr);
    auto buffer   = reinterpret_cast<std::unique_ptr<FileBuffer>*>((uintptr_t)buffer_ptr);
    if(block->BlockId() < MAXIMUM_BLOCK) {
        auto new_block = AllocateBlock(block->block_manager, std::move(*reusable_buffer), block->BlockId());
        block->block_manager.Read(*new_block);
        *buffer = std::move(new_block);
        return 0;
    }
    return -1;
}

uint64_t write_temprorary_file(uint64_t block_ptr, uint64_t reusable_buffer_ptr, uint64_t buffer_ptr){
    auto block = reinterpret_cast<duckdb::BlockHandle*>((uintptr_t)block_ptr);
    auto reusable_buffer = reinterpret_cast<duckdb::unique_ptr<duckdb::FileBuffer>*>((uintptr_t) reusable_buffer_ptr);
    if(block->MustWriteToTemporaryFile()){
        block-> ReadTemporaryBufferHelper(std::move(*reusable_buffer));
        return 1;
    } else {
        return -1; 
    } 
}

uint64_t get_arena_head(uint64_t arena_allocator_ptr){
    auto arena_allocator = reinterpret_cast<duckdb::ArenaAllocator*>((uintptr_t)arena_allocator_ptr);
    auto head = arena_allocator->GetHead();
    if(!head){
        return -1;
    }
    return head->maximum_size;
}

void arena_prepend_chunk(uint64_t arena_allocator_ptr, uint64_t capacity){
    auto arena_allocator = reinterpret_cast<duckdb::ArenaAllocator*>((uintptr_t)arena_allocator_ptr);
    arena_allocator->PrependChunk(capacity);
    
}

uint64_t bpf_get_estimated_cpu_id() {
    return (uint64_t)duckdb::TaskScheduler::GetEstimatedCPUId();
}

uint64_t bpf_fetch_add(uint64_t addr, uint64_t size){
    auto *p = reinterpret_cast<std::atomic<int64_t>*>(addr);
    return (uint64_t)p->fetch_add(size, std::memory_order_relaxed);
}

uint64_t bpf_exchange(uint64_t addr, uint64_t new_val){
    auto *p = reinterpret_cast<std::atomic<int64_t>*>(addr);
    return (uint64_t)p->exchange(new_val, std::memory_order_relaxed);
}

}

void bpf_thread_flush(uint64_t sched_ptr, uint64_t threshold, uint64_t requested_thread_count) {
    auto scheduler = reinterpret_cast<duckdb::TaskScheduler*>(sched_ptr);
    scheduler->FlushHelper(threshold, requested_thread_count);
}

void bpf_queue_wait(uint64_t sched_ptr) {
    auto scheduler = reinterpret_cast<TaskScheduler*>((uintptr_t)sched_ptr);
    if (!scheduler) return;
    scheduler->QueueWaitHelper();
}

uint64_t bpf_queue_wait_timeout(uint64_t sched_ptr, uint64_t timeout_us) {
    auto sched = reinterpret_cast<TaskScheduler*>((uintptr_t)sched_ptr);
    if (!sched) return 0;
    return sched->QueueWaitTimeoutHelper((int64_t)timeout_us) ? 1 : 0;
}

uint64_t bpf_allocator_decay_delay() {
    auto d = Allocator::DecayDelay();
    if (!d.IsValid()) {
        return 0;
    }
    return (uint64_t)(d.GetIndex());
}

uint64_t bpf_queue_dequeue(uint64_t sched_ptr, uint64_t out_task_ptr) {
    auto sched    = reinterpret_cast<TaskScheduler*>((uintptr_t)sched_ptr);
    auto out_slot = reinterpret_cast<shared_ptr<Task>*>((uintptr_t)out_task_ptr);
    if (!sched || !out_slot) {
        fprintf(stderr, "[eBPF] dequeue: null arg\n");
        return 0;
    }
    bool ok = sched->TryDequeueHelper(*out_slot);
    return ok ? 1 : 0;
}


uint64_t bpf_task_execute(uint64_t task_ptr) {
    auto slot = reinterpret_cast<shared_ptr<Task>*>((uintptr_t)task_ptr);
    if (!slot || !*slot) {
        return (uint64_t)TaskExecutionResult::TASK_ERROR;
    }
    auto res = (*slot)->Execute(TaskExecutionMode::PROCESS_ALL);
    return (uint64_t)res;
}

void bpf_task_deschedule(uint64_t task_ptr) {
    auto slot = reinterpret_cast<shared_ptr<Task>*>((uintptr_t)task_ptr);
    if (slot && *slot) {
        (*slot)->Deschedule();
    }
}

void bpf_task_reset(uint64_t task_ptr) {
    auto slot = reinterpret_cast<shared_ptr<Task>*>((uintptr_t)task_ptr);
    if (slot) {
        slot->reset();
    }
}


void bpf_allocator_thread_idle() {
    Allocator::ThreadIdle();
}


ubpf_helper_descriptor my_helpers[] = {
    { "bpf_print",    (void *)bpf_print, 1 },
	{"bpf_memcpy" , (void*)bpf_memcpy, 2},
    { "bpf_load",  (void*)bpf_load,  3 },
    {"bpf_enqueue", (void*)bpf_task_enqueue, 4},
    {"handle_buffer", (void*)handle_buffer , 5},
    {"handle_decrement_readers", (void*)handle_decrement_readers , 6},
    {"verify_zero_readers", (void*)verify_zero_readers , 7},
    {"add_to_eviction_queue", (void*)add_to_eviction_queue , 8},
    {"bpf_purge_block", (void*)bpf_purge_block , 9},
    {"block_is_loaded", (void*)block_is_loaded, 10},
    {"id_less_than_max", (void*)id_less_than_max, 11},
    {"write_temprorary_file", (void*)write_temprorary_file, 12},
    {"get_arena_head", (void*)get_arena_head, 13},
    {"arena_prepend_chunk", (void*)arena_prepend_chunk, 14},
    {"bpf_get_estimated_cpu_id", (void*)bpf_get_estimated_cpu_id, 15},
    {"bpf_fetch_add", (void*)bpf_fetch_add, 16},
    {"bpf_exchange", (void*)bpf_exchange, 17},
    {"bpf_thread_flush", (void*)bpf_thread_flush, 18},
    {"bpf_queue_wait", (void*)bpf_queue_wait, 19},
    {"bpf_queue_wait_timeout", (void*)bpf_queue_wait_timeout, 20},
    {"bpf_allocator_decay_delay", (void*)bpf_allocator_decay_delay, 21},
    {"bpf_queue_dequeue", (void*)bpf_queue_dequeue, 22},
    {"bpf_task_execute", (void*)bpf_task_execute, 23},
    {"bpf_task_deschedule", (void*)bpf_task_deschedule, 24},
    {"bpf_task_reset", (void*)bpf_task_reset, 25},
    {"bpf_allocator_thread_idle", (void*)bpf_allocator_thread_idle, 26},
};

size_t num_helpers = sizeof(my_helpers)/sizeof(my_helpers[0]);




