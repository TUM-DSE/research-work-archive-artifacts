# Directories
LLAMA_CPP_DIR := ../llama.cpp
LLAMA_CPP_INC := ../llama.cpp/include
LLAMA_CPP_COMMON_INC := $(LLAMA_CPP_DIR)/common
LLAMA_CPP_BENCH := $(LLAMA_CPP_DIR)/examples/llama_bench/llama_bench.cpp
GGML_INC := $(LLAMA_CPP_DIR)/ggml/include
LIB_DIR := $(PWD)/libs
SRC_DIR = src
SERVER_DIR = $(SRC_DIR)/server
CLIENT_DIR = $(SRC_DIR)/client

# Compiler
CC = g++
CFLAGS = -Wall -Wno-deprecated-declarations -g -Isrc/include -std=c++20 -O3
LLAMA_FLAGS = -I$(LLAMA_CPP_INC) -I$(LLAMA_CPP_COMMON_INC) -I$(GGML_INC) 
# Set the libary path for the runtime shared libraries
LDFLAGS = -L$(LIB_DIR) -Wl,-rpath,$(LIB_DIR) -lllama -lggml -lcommon


# Source files
SERVER_SRC = $(SERVER_DIR)/llama_server.cpp src/shm.cpp
CLIENT_SRC = $(CLIENT_DIR)/write.c src/shm.cpp

# Output executables
SERVER_OUT = server
CLIENT_OUT = client

# Targets
all: llama.cpp $(SERVER_OUT) $(CLIENT_OUT)

# Get llama.cpp libraries
llama.cpp:
	mkdir -p libs; \
	find $(LLAMA_CPP_DIR) -name "lib*" -exec cp {} $(LIB_DIR) \;

# Build server
$(SERVER_OUT): $(SERVER_SRC) llama.cpp
	$(CC) $(CFLAGS) $(LLAMA_FLAGS) -o $(SERVER_OUT) $(SERVER_SRC) libs/libcommon.a $(LDFLAGS) 

# Build client
$(CLIENT_OUT): $(CLIENT_SRC)
	$(CC) $(CFLAGS) -o $(CLIENT_OUT) $(CLIENT_SRC)

# Clean
clean:
	rm -f $(SERVER_OUT) $(CLIENT_OUT)

# Phony targets
.PHONY: all clean

