BPF_CC  = clang
UBPF_DIR = ../../modules/ubpf
BPF_CFLAGS = \
    -O2 -target bpf \
    -nostdinc \
    -I$(shell $(BPF_CC) --print-resource-dir)/include
ENABLE_HOOKS ?= OFF
HOOK_DIR     := hooks
HOOK_OBJDIR  := $(HOOK_DIR)/objects
HOOK_SRCS   := $(wildcard $(HOOK_DIR)/*.c)
HOOK_OBJS   := $(patsubst $(HOOK_DIR)/%.c,$(HOOK_OBJDIR)/%.o,$(HOOK_SRCS))
.PHONY: module clean distclean

module: duckdb/duckdb hooks

hooks: | $(HOOK_OBJDIR)
hooks: $(HOOK_OBJS)

$(HOOK_OBJDIR):
	@mkdir -p $@
	
$(HOOK_OBJDIR)/%.o: hooks/%.c
	$(BPF_CC) $(BPF_CFLAGS) -c $< -o $@

duckdb/duckdb: duckdb 
	cd duckdb; DUCKDB_PLATFORM=linux_amd64 BUILD_BENCHMARK=1 BUILD_TPCH=1 make ENABLE_HOOKS=$(ENABLE_HOOKS);

clean:
	rm -rf hooks/*.o
	rm -rf ../../modules/ubpf/ubpf.so
	rm -rf ../../modules/ubpf/libubpf.a
	rm -rf ../../modules/ubpf/libubpf.so
	if [ -d duckdb ]; then cd duckdb && make clean; fi
	

distclean:
	rm -rf duckdb
	rm -rf hooks/objects/*.o
	rm -rf ../../modules/ubpf/ubpf.so
	rm -rf ../../modules/ubpf/libubpf.a
	rm -rf ../../modules/ubpf/libubpf.so
