CC  = gcc
CXX = g++

INCLUDES = -Ivm -Ivm/inc -Ivm/build -I../../benchmarks/duckdb/duckdb/src/include -I../../benchmarks/duckdb/duckdb/third_party/concurrentqueue


CFLAGS   = -std=c11 -fPIC -g -Wall -DUBPF_HAS_ELF_H $(INCLUDES)
CXXFLAGS = -std=c++17 -fPIC -g -Wall -DUBPF_HAS_ELF_H $(INCLUDES)


%.o: %.c
	$(CC) $(CFLAGS) -c $< -o $@

%.o: %.cpp
	$(CXX) $(CXXFLAGS) -c $< -o $@

vm/ubpf_loader.o: vm/ubpf_loader.c
	$(CC) $(CFLAGS) -c $< -o $@


libubpf.so: ubpf_runtime.o vm/ubpf_vm.o vm/ubpf_loader.o vm/ubpf_instruction_valid.o ubpf_helpers.o vm/ubpf_jit.o vm/ubpf_jit_x86_64.o vm/ubpf_jit_support.o ubpf_helpers.o
	$(CXX) $(CXXFLAGS) -shared -o $@ $^



module: libubpf.so 

clean:
	rm -f vm/*.o libubpf.so 
