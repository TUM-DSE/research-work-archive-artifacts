set -x

source /llm-os/benchmarks/config.sh
export N_THREADS=16
export OUTPUT_DIR=/results

wait_for_server () {
  until [ -f "/server_signals/test$1" ]
  do
     sleep 3
  done
}

# compile llama.cpp
export GGML_CUDA=1
cd llama.cpp
make clean
make libllama.so -j8
make libcommon.a -j8
make libggml.so -j8

# compile server
cd ../intervm_comm
make clean
make server

# TEST 2.1: server <-> client (5 minutes)
./server -m ../models/model_file -p /results/gpu_1_client_server -f /server_signals/test2_1_gpu --n_threads=$N_THREADS

# TEST 2.2: server <-> 2 * client
./server -m ../models/model_file -p /results/gpu_2_client_server -f /server_signals/test2_2_gpu --n_threads=$N_THREADS

# TEST 2.3: server <-> 4 * client
./server -m ../models/model_file -p /results/gpu_4_client_server -f /server_signals/test3 --n_threads=$N_THREADS

# TEST 2.4: server <-> 8 * client
./server -m ../models/model_file -p /results/gpu_8_client_server -f /server_signals/test4 --n_threads=$N_THREADS

# TEST 2.5: server <-> 16 * client
./server -m ../models/model_file -p /results/gpu_16_client_server -f /server_signals/test5 --n_threads=$N_THREADS
# TEST 2.5.32: server <-> 32 * client
./server -m ../models/model_file -p /results/gpu_32_client_server -f /server_signals/test5_32 --n_threads=$N_THREADS
# TEST 2.5.64: server <-> 64 * client
./server -m ../models/model_file -p /results/gpu_64_client_server -f /server_signals/test5_64 --n_threads=$N_THREADS
# TEST 2.5.1: server <-> 16 * client n_token 50
./server -m ../models/model_file -p /results/gpu_16_client_server_50_tokens -f /server_signals/test5_1 --n_threads=$N_THREADS --tokens-to-gen=50
# TEST 2.5.2: server <-> 16 * client n_token 100
./server -m ../models/model_file -p /results/gpu_16_client_server_100_tokens -f /server_signals/test5_2 --n_threads=$N_THREADS --tokens-to-gen=100
# TEST 2.5.3: server <-> 16 * client n_token 20
./server -m ../models/model_file -p /results/gpu_16_client_server_20_tokens -f /server_signals/test5_3 --n_threads=$N_THREADS --tokens-to-gen=20
# TEST 2.5.4: server <-> 16 * client n_token 30
./server -m ../models/model_file -p /results/gpu_16_client_server_30_tokens -f /server_signals/test5_4 --n_threads=$N_THREADS --tokens-to-gen=30
# TEST 2.5.5: server <-> 16 * client n_token 40
./server -m ../models/model_file -p /results/gpu_16_client_server_40_tokens -f /server_signals/test5_5 --n_threads=$N_THREADS --tokens-to-gen=40

# TEST 2.6: server <-> 2 * client (diff prio)
./server -m ../models/model_file -p /results/gpu_2_client_diff_prio_server -f /server_signals/test6 --n_threads=$N_THREADS

# TEST 2.7: server <-> 4 * client (diff prio)
./server -m ../models/model_file -p /results/gpu_4_client_diff_prio_server -f /server_signals/test7 --n_threads=$N_THREADS

# TEST 2.8: server <-> 8 * client (diff prio)
./server -m ../models/model_file -p /results/gpu_8_client_diff_prio_server -f /server_signals/test8 --n_threads=$N_THREADS

# TEST 2.9: server <-> 16 * client (diff prio)
./server -m ../models/model_file -p /results/gpu_16_client_diff_prio_server -f /server_signals/test9 --n_threads=$N_THREADS
# TEST 2.10: server <-> client + phoronix
./server -m ../models/model_file -p /results/gpu_openssl_1_client_server -f /server_signals/test2_10_gpu --n_threads=$N_THREADS

# TEST 2.11.1: server (throughput limit) <-> client + phoronix
./server -m ../models/model_file -p /results/gpu_openssl_1_client_server_throtthle_10 -f /server_signals/test2_11_1_gpu --throughput-limit=10 --n_threads=$N_THREADS
# TEST 2.11.2: server (throughput limit) <-> client + phoronix
./server -m ../models/model_file -p /results/gpu_openssl_1_client_server_throtthle_20 -f /server_signals/test2_11_2_gpu --throughput-limit=20 --n_threads=$N_THREADS
# TEST 2.11.3: server (throughput limit) <-> client + phoronix
./server -m ../models/model_file -p /results/gpu_openssl_1_client_server_throtthle_30 -f /server_signals/test2_11_3_gpu --throughput-limit=30 --n_threads=$N_THREADS
# TEST 2.11.4: server (throughput limit) <-> client + phoronix
./server -m ../models/model_file -p /results/gpu_openssl_1_client_server_throtthle_40 -f /server_signals/test2_11_4_gpu --throughput-limit=40 --n_threads=$N_THREADS

# TEST 2.12: client side
wait_for_server "2_12_gpu_complete"

# TEST 2.13 GPU: llama-bench gpu
cd ../llama.cpp
make llama-bench
./llama-bench -m ../models/model_file -n 64,128,256,512 -ngl 999 --threads $N_THREADS -o json --progress > $OUTPUT_DIR/llama_bench_Q4_K_M_gpu_vm.json

# compile llama.cpp
unset GGML_CUDA
cd ../llama.cpp
make clean
make libllama.so -j8
make libcommon.a -j8
make libggml.so -j8

# compile server
cd ../intervm_comm
make clean
make server

# TEST 2.1: server <-> client (5 minutes)
./server -m ../models/model_file -p /results/cpu_1_client_server_zero_sleep -f /server_signals/test2_1_cpu --n_threads=$N_THREADS

# TEST 2.2: server <-> 2 * client
./server -m ../models/model_file -p /results/cpu_2_client_server_zero_sleep -f /server_signals/test2_2_cpu --n_threads=$N_THREADS

# TEST 2.3: server <-> 4 * client
./server -m ../models/model_file -p /results/4_client_server_zero_sleep -f /server_signals/test3 --n_threads=$N_THREADS

# TEST 2.4: server <-> 8 * client
./server -m ../models/model_file -p /results/8_client_server_zero_sleep -f /server_signals/test4 --n_threads=$N_THREADS

# TEST 2.5: server <-> 16 * client
./server -m ../models/model_file -p /results/16_client_server_zero_sleep -f /server_signals/test5 --n_threads=$N_THREADS

# TEST 2.5: server <-> 16 * client
./server -m ../models/model_file -p /results/16_client_server_20_minutes -f /server_signals/test5_20m --n_threads=$N_THREADS
# TEST 2.5: server <-> 16 * client
./server -m ../models/model_file -p /results/16_client_server_30_minutes -f /server_signals/test5_30m --n_threads=$N_THREADS
# TEST 2.5: server <-> 16 * client
./server -m ../models/model_file -p /results/16_client_server_40_minutes -f /server_signals/test5_40m --n_threads=$N_THREADS
# TEST 2.5: server <-> 16 * client
./server -m ../models/model_file -p /results/16_client_server_50_minutes -f /server_signals/test5_50m --n_threads=$N_THREADS

# TEST 2.5.1: server <-> 16 * client n_token 50
./server -m ../models/model_file -p /results/16_client_server_50_tokens -f /server_signals/test5_1 --n_threads=$N_THREADS --tokens-to-gen=50
# TEST 2.5.2: server <-> 16 * client n_token 100
./server -m ../models/model_file -p /results/16_client_server_100_tokens -f /server_signals/test5_2 --n_threads=$N_THREADS --tokens-to-gen=100
# TEST 2.5.3: server <-> 16 * client n_token 20
./server -m ../models/model_file -p /results/16_client_server_20_tokens -f /server_signals/test5_3 --n_threads=$N_THREADS --tokens-to-gen=20
# TEST 2.5.4: server <-> 16 * client n_token 30
./server -m ../models/model_file -p /results/16_client_server_30_tokens -f /server_signals/test5_4 --n_threads=$N_THREADS --tokens-to-gen=30
# TEST 2.5.5: server <-> 16 * client n_token 40
./server -m ../models/model_file -p /results/16_client_server_40_tokens -f /server_signals/test5_5 --n_threads=$N_THREADS --tokens-to-gen=40

# TEST 2.6: server <-> 2 * client (diff prio)
./server -m ../models/model_file -p /results/2_client_diff_prio_server -f /server_signals/test6 --n_threads=$N_THREADS

# TEST 2.7: server <-> 4 * client (diff prio)
./server -m ../models/model_file -p /results/4_client_diff_prio_server -f /server_signals/test7 --n_threads=$N_THREADS

# TEST 2.8: server <-> 8 * client (diff prio)
./server -m ../models/model_file -p /results/8_client_diff_prio_server -f /server_signals/test8 --n_threads=$N_THREADS

# TEST 2.9: server <-> 16 * client (diff prio)
./server -m ../models/model_file -p /results/16_client_diff_prio_server -f /server_signals/test9 --n_threads=$N_THREADS

# TEST 2.10: server <-> client + phoronix
./server -m ../models/model_file -p /results/cpu_openssl_1_client_server -f /server_signals/test2_10_cpu --n_threads=$N_THREADS

# TEST 2.11.1: server (throughput limit) <-> client + phoronix
./server -m ../models/model_file -p /results/cpu_openssl_1_client_server_throtthle -f /server_signals/test2_11_1_cpu --throughput-limit=1 --n_threads=$N_THREADS
# TEST 2.11.2: server (throughput limit) <-> client + phoronix
./server -m ../models/model_file -p /results/cpu_openssl_1_client_server_throtthle -f /server_signals/test2_11_2_cpu --throughput-limit=2 --n_threads=$N_THREADS
# TEST 2.11.1: server (throughput limit) <-> client + phoronix
./server -m ../models/model_file -p /results/cpu_openssl_1_client_server_throtthle -f /server_signals/test2_11_3_cpu --throughput-limit=3 --n_threads=$N_THREADS
# TEST 2.11.1: server (throughput limit) <-> client + phoronix
./server -m ../models/model_file -p /results/cpu_openssl_1_client_server_throtthle -f /server_signals/test2_11_4_cpu --throughput-limit=4 --n_threads=$N_THREADS

# TEST 2.12: none <-> phoronix
# this is alreday executed

# TEST 2.13 CPU: llama-bench
cd ../llama.cpp
make llama-bench
./llama-bench -m ../models/model_file -p 0 -n 64,128,256,512 -ngl 0 --threads $N_THREADS -o json --progress > $OUTPUT_DIR/llama_bench_Q4_K_M_cpu_vm.json

exit

