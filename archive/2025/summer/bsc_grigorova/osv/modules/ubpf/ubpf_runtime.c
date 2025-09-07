#include <stdio.h>
#include <stdint.h>
#include <stdlib.h>
#include <sys/stat.h>
#include <string.h>

#include "vm/inc/ubpf.h"
#include "ubpf_runtime.h"


extern ubpf_helper_descriptor my_helpers[];
extern size_t num_helpers;
#define MAX_HOOKS 100
typedef uint64_t (*ubpf_jit_fn)(void *ctx, size_t ctx_size);



typedef struct {
    char name[32];
    struct ubpf_vm *vm;
    ubpf_jit_fn fn;
    char prog_path[256];
    char entry_symbol[64];
} compiled_hook;

static compiled_hook hooks[MAX_HOOKS];
static size_t hook_count = 0;


void register_helpers(struct ubpf_vm *vm) {
    for (size_t i = 0; i < num_helpers; ++i) {
        ubpf_register(vm, my_helpers[i].index, my_helpers[i].name, my_helpers[i].fn);
    }
}


ubpf_jit_fn load_and_compile(struct ubpf_vm *hook_vm,const char *prog_path, const char *entry_symbol ){
    FILE *file = fopen(prog_path, "rb");
    if (!file) {
        perror("Failed to open ELF file");
        return NULL;
    }
    fseek(file, 0, SEEK_END);
    size_t elf_size = ftell(file);
    fseek(file, 0, SEEK_SET);

    void *elf_data = malloc(elf_size);
    if (!elf_data || fread(elf_data, 1, elf_size, file) != elf_size) {
        perror("Failed to read ELF file");
        fclose(file);
        free(elf_data);
        return NULL;
    }
    fclose(file);

    char *errmsg = NULL;
    if (ubpf_load_elf_ex(hook_vm, elf_data, elf_size, entry_symbol, &errmsg) < 0) {
        fprintf(stderr, "Failed to load ELF: %s\n", errmsg);
        free(errmsg);
        free(elf_data);
        return NULL;
    }
    free(elf_data);

    ubpf_jit_fn jitted = ubpf_compile(hook_vm, &errmsg);
    if (!jitted) {
        fprintf(stderr, "JIT compile failed: %s\n", errmsg);
        free(errmsg);
        return NULL;
    }
    return jitted;
}

int initialize_hook(const char *hook_name, const char *prog_path, const char *entry_symbol) {
    for (size_t i = 0; i < hook_count; ++i) {
        if (strcmp(hooks[i].name, hook_name) == 0) {
            // Hook already exists, do nothing
            printf("hook exists\n");
            return i;  // Return existing hook ID
        }
    } 
    struct ubpf_vm* hook_vm = ubpf_create();
    if (!hook_vm || hook_count >= MAX_HOOKS)
        return -1;

    register_helpers(hook_vm);
    
    ubpf_jit_fn jitted = load_and_compile(hook_vm, prog_path, entry_symbol);
    if (!jitted) {
        ubpf_destroy(hook_vm);
        return -3;
    }
    compiled_hook *hook = &hooks[hook_count++];
    strncpy(hook->name, hook_name, sizeof(hook->name));
    strncpy(hook->prog_path, prog_path, sizeof(hook->prog_path));
    strncpy(hook->entry_symbol, entry_symbol, sizeof(hook->entry_symbol));
    hook->vm = hook_vm;
    hook->fn = jitted;


    return hook_count - 1;
}


int call_hook(const char* hook_name, void *ctx, size_t ctx_size, uint64_t *result) {
    int hook_id =-1;
    for (size_t i = 0; i < hook_count; ++i) {
        if (strcmp(hooks[i].name, hook_name) == 0) {
            hook_id = i;
        }
    }
    if(hook_id==-1){
        fprintf(stderr, "Invalid hook name: %s\n", hook_name);
        return -1;
    }
    
    *result = hooks[hook_id].fn(ctx, ctx_size);

    return 0;
}

void destroy_ubpf(void) {
    for(int i=0; i<hook_count; i++) {
        ubpf_destroy(hooks[i].vm);
        hooks[i].vm = NULL;
    }
    hook_count = 0;
}

int reload_hook(const char *hook_name) {
    for (size_t i = 0; i < hook_count; ++i) {
        if (strcmp(hooks[i].name, hook_name) == 0) {
            compiled_hook *hook = &hooks[i];      
            
            ubpf_unload_code(hook->vm);
            ubpf_jit_fn jitted = load_and_compile(hook->vm, hook->prog_path, hook->entry_symbol);
            if (!jitted) {
                ubpf_destroy(hook->vm);
                return -3;
            }

            hook->fn = jitted;

            return 0;
        }
    }
    return -1; // Hook not found
}



