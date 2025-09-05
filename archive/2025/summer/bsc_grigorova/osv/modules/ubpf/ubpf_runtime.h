#ifndef UBPF_RUNTIME_H
#define UBPF_RUNTIME_H

#include <stdint.h>
#include <stddef.h>

#ifdef __cplusplus
extern "C" {
#endif


typedef struct {
    const char *name;
    void *fn;
    int index;
} ubpf_helper_descriptor;



int initialize_hook(const char *hook_name, const char *prog_path, const char *entry_symbol);
int call_hook(const char* hook_name, void *ctx, size_t ctx_size, uint64_t *result);
void destroy_ubpf(void);
int reload_hook(const char *hook_name);


#ifdef __cplusplus
}
#endif

#endif // UBPF_RUNTIME_H
