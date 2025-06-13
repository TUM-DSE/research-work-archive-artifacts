set -x

source ./config.sh

SHARED_MEM_LOCATION="/dev/shm/shmBench"

wait_for_server () {
  until [ -f "../benchmarks/server_signals/test$1" ]
  do
     sleep 3
  done
}

source N_THREADS=16
echo "$N_THREADS"

# TEST 1.1: phoronix native
export TEST_RESULTS_NAME=openssl-native-no-clients
export OUTPUT_DIR=$(pwd)/results

printf 'y\nn\nn\nn\nn\nn\nn\n' | phoronix-test-suite batch-setup
phoronix-test-suite batch-install pts/openssl

# start the benchmark
# 2 defines the encryption
printf '2\n' | phoronix-test-suite batch-run pts/openssl

# save the result to shared directory
phoronix-test-suite result-file-to-csv $TEST_RESULTS_NAME

# TEST 1.2.1 server (GPU) <-native-> client
# compile llama.cpp
export GGML_CUDA=1
cd ../llama.cpp
make clean
make libllama.so
make libcommon.a
make libggml.so

# compile server/client
cd ../intervm_comm
make clean
make

# get the shared memory ready
rm $SHARED_MEM_LOCATION
touch $SHARED_MEM_LOCATION

# clean signals
mkdir -p ../benchmarks/server_signals
rm ../benchmarks/server_signals/*

./server -m ../benchmarks/models/model_file -p ../benchmarks/results/native_gpu_1_client_server -f ../benchmarks/server_signals/test1_2_1 --shared-mem=$SHARED_MEM_LOCATION --n_threads=$N_THREADS &
wait_for_server "1_2_1"
./client -i 1 -p 1 --sleep 0 --shared-mem=$SHARED_MEM_LOCATION &
CLIENT_PID=$!
sleep 300

# kill the server and clients
./client -w --shared-mem=$SHARED_MEM_LOCATION
kill $CLIENT_PID

# TEST 1.2.2 server (CPU) <-native-> client
# compile llama.cpp
unset GGML_CUDA
cd ../llama.cpp
make clean
make libllama.so -j8
make libcommon.a -j8
make libggml.so -j8

# compile server/client
cd ../intervm_comm
make clean
make

# get the shared memory ready
rm $SHARED_MEM_LOCATION
touch $SHARED_MEM_LOCATION

# clean signals
mkdir -p ../benchmarks/server_signals
rm ../benchmarks/server_signals/*

./server -m ../benchmarks/models/model_file -p ../benchmarks/results/native_cpu_1_client_server -f ../benchmarks/server_signals/test1_2_2 --shared-mem=$SHARED_MEM_LOCATION --n_threads=$N_THREADS &
wait_for_server "1_2_2"
./client -i 1 -p 1 --sleep 0 --shared-mem=$SHARED_MEM_LOCATION &
CLIENT_PID=$!
sleep 300

# kill the server and clients
./client -w --shared-mem=$SHARED_MEM_LOCATION
kill $CLIENT_PID

# TEST 1.3: llama-bench cpu
cd ../llama.cpp
unset GGML_CUDA
make clean
make llama-bench -j8
./llama-bench -m ../benchmarks/models/model_file -t $N_THREADS -p 0 -n 64,128,256,512,1024 -ngl 0 -o json --progress > ../benchmarks/results/llama_bench_Q4_K_M_cpu_native.json

# TEST 1.4: llama-bench gpu
export GGML_CUDA=1
make clean
make llama-bench -j8
./llama-bench -m ../benchmarks/models/model_file -t $N_THREADS -p 0 -n 64,128,256,512,1024 -ngl 999 -o json --progress > ../benchmarks/results/llama_bench_Q4_K_M_gpu_native.json

