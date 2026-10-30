#!/usr/bin/env bash
# To run the benchmarks with the original implementation do this:
# $ GEM5=/gem5/build/VEGA_X86/gem5.opt SUFFIX=orig ./run.sh all

GEM5=${GEM5:-/gem5/build/cxl_apu/gem5.opt}
CONF=${CONF:-/gem5/configs/example/apu_se.py}
CHAI=${CHAI:-/chai/HIP-U-gem5}
CHAI_BIN=${CHAI_BIN:-${CHAI}/bin}
CHAI_BIN_NO_UVM=${CHAI_BIN_NO_UVM:-${CHAI}/bin_no_uvm}
OUTDIR=$CHAI/outdir

if [[ -z ${LATENCY+x} ]]; then
  lat=""
else
  lat="--gpu-to-dir-latency=$LATENCY"
fi

CONF_OPTIONS='--mem-size=8GiB'
CONF_UVM='--gfx-version=gfx902'
CONF_NO_UVM='--dgpu --gfx-version=gfx900'

if [[ ${NO_UVM} == "y" ]];then
    CONF_OPTIONS=$CONF_OPTIONS" "$CONF_NO_UVM
    BIN_DIR=$CHAI_BIN_NO_UVM
else
    CONF_OPTIONS=$CONF_OPTIONS" "$CONF_UVM
    BIN_DIR=$CHAI_BIN
fi

all=("bfs" "bs" "cedd" "cedt" "hsti" "hsto" "pad" "rscd" "rsct" "sc" "sssp" "tq" "tqh" "trns")
declare -A options=(
  [trns]='"-t 0"'
)

run_bench() {
    BIN=$1
    $GEM5 -d ${OUTDIR}/${BIN}_${LATENCY}_${SUFFIX}$NO_UVM \
	    -r -e \
	    $CONF -n 8 -u 8 $lat \
	    $CONF_OPTIONS \
	    --cmd $BIN_DIR/$BIN.gem5 \
	    "--options=${options[${BIN}]}"
}

if [ "$#" -eq 1 ]; then
  if [[ " ${all[*]} " =~ " $1 " ]]; then
    printf "Running the $1 benchmark\n"
    run_bench $1
    exit $?
  elif [ "$1" == "all" ]; then
    printf "Running all benchmarks\n"
    for bench in "${all[@]}"; do
      run_bench $bench &
    done
    wait
    printf "All benchmarks finished\n"
    exit 0
  fi
else
  printf "Usage: $0 <benchmark>\n"
  printf "Where <benchmark> is one of:\n"
  printf "  - all (to run all benchmarks)\n"
  for b in "${all[@]}"; do
    printf "  - $b\n"
  done
  exit 1
fi