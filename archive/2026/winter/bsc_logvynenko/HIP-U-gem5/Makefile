export ROCM_PATH=/opt/rocm
export GEM5_PATH=/gem5
RODINIA_BASE_DIR := $(shell pwd)
export BIN_DIR := $(RODINIA_BASE_DIR)/bin
export BIN_DIR_NO_UVM := $(RODINIA_BASE_DIR)/bin_no_uvm

HIP_DIRS := bfs bs cedd cedt hsti hsto pad rscd rsct sc sssp tq tqh trns

all: HIP

HIP:
	export GFX_ARCH="gfx902" && for dir in $(HIP_DIRS) ; do $(MAKE) -B -C $$dir ; done

no_uvm:
	export OPTS="-DDISABLE_UVM" && export GFX_ARCH="gfx900" && export BIN_DIR=$(BIN_DIR_NO_UVM) && for dir in $(HIP_DIRS) ; do $(MAKE) -B -C $$dir ; done

clean:
	rm -f $(BIN_DIR)/*
	rm -f $(BIN_DIR_NO_UVM)/*
	for dir in $(HIP_DIRS) ; do $(MAKE) -B -C $$dir clean ; done