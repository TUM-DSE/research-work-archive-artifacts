set -x

wait_for_server () {
  until [ -f "/server_signals/test$1" ]
  do
     sleep 3
  done
}

export FORCE_TIMES_TO_RUN=5 # Force Phoronix to repeat the same test specific amount of times
TEST_LENGTH=600

# compile client
cd intervm_comm
make clean
make client

# TEST 2.1 GPU: server <-> client (5 minutes)
wait_for_server "2_1_gpu"
./client -i 1 -p 1 -s 0 &

sleep $TEST_LENGTH
./client -w
pkill client

# TEST 2.2 GPU: server <-> 2 * client
wait_for_server "2_2_gpu"
for i in $(seq 1 2); do
    ./client -i "$i" -p 1 -s 0 &
    sleep 1
done

sleep $TEST_LENGTH
./client -w
pkill client

# TEST 2.3: server <-> 4 * client
wait_for_server "3"
for i in $(seq 1 4); do
    ./client -i "$i" -p 1 &
    sleep 1
done

sleep $TEST_LENGTH
./client -w
pkill client

# TEST 2.4: server <-> 8 * client
wait_for_server "4"
for i in $(seq 1 8); do
    ./client -i "$i" -p 1 &
    sleep 1
done

sleep $TEST_LENGTH
./client -w
pkill client

# TEST 2.5: server <-> 16 * client
wait_for_server "5"
for i in $(seq 1 16); do
    ./client -i "$i" -p 1 &
    sleep 1
done

sleep $TEST_LENGTH
./client -w
pkill client
# TEST 2.5.32: server <-> 32 * client
wait_for_server "5_32"
for i in $(seq 1 32); do
    ./client -i "$i" -p 1 &
    sleep 1
done

sleep $TEST_LENGTH
./client -w
pkill client
# TEST 2.5_64: server <-> 64 * client
wait_for_server "5_64"
for i in $(seq 1 64); do
    ./client -i "$i" -p 1 &
    sleep 1
done

sleep $TEST_LENGTH
./client -w
pkill client
# TEST 2.5.1: server <-> 16 * client, 50 n_token
wait_for_server "5_1"
for i in $(seq 1 16); do
    ./client -i "$i" -p 1 &
    sleep 1
done

sleep $TEST_LENGTH
./client -w
pkill client

# TEST 2.5.2: server <-> 16 * client, 100 n_token
wait_for_server "5_2"
for i in $(seq 1 16); do
    ./client -i "$i" -p 1 &
    sleep 1
done

sleep $TEST_LENGTH
./client -w
pkill client

# TEST 2.5.3: server <-> 16 * client, 20 n_token
wait_for_server "5_3"
for i in $(seq 1 16); do
    ./client -i "$i" -p 1 &
    sleep 1
done

sleep $TEST_LENGTH
./client -w
pkill client

# TEST 2.5.4: server <-> 16 * client, 30 n_token
wait_for_server "5_4"
for i in $(seq 1 16); do
    ./client -i "$i" -p 1 &
    sleep 1
done

sleep $TEST_LENGTH
./client -w
pkill client

# TEST 2.5.3: server <-> 16 * client, 40 n_token
wait_for_server "5_5"
for i in $(seq 1 16); do
    ./client -i "$i" -p 1 &
    sleep 1
done

sleep $TEST_LENGTH
./client -w
pkill client

# TEST 2.6: server <-> 2 * client
wait_for_server "6"
for i in $(seq 1 2); do
    ./client -i "$i" -p "$i" &
    sleep 1
done

sleep $TEST_LENGTH
./client -w
pkill client

# TEST 2.7: server <-> 4 * client
wait_for_server "7"
for i in $(seq 1 4); do
    ./client -i "$i" -p "$i" &
    sleep 1
done

sleep $TEST_LENGTH
./client -w
pkill client

# TEST 2.8: server <-> 8 * client
wait_for_server "8"
for i in $(seq 1 8); do
    ./client -i "$i" -p "$i" &
    sleep 1
done

sleep $TEST_LENGTH
./client -w
pkill client

# TEST 2.9: server <-> 16 * client
wait_for_server "9"
for i in $(seq 1 16); do
    ./client -i "$i" -p "$i" &
    sleep 1
done

sleep $TEST_LENGTH
./client -w
pkill client
sleep 5

# TEST 2.10: server <-> client + phoronix
printf 'y\nn\nn\nn\nn\nn\nn\n' | phoronix-test-suite batch-setup
phoronix-test-suite batch-install pts/openssl

export TEST_RESULTS_NAME=gpu-phoronix-result-openssl-vm-1-client
export OUTPUT_DIR=/results

wait_for_server "2_10_gpu"
./client -i 1 -p 1 -s 0 &

# start the benchmark
# 2 defines the encryption
printf '2\n' | phoronix-test-suite batch-run pts/openssl

# after the benchmark send stop request
./client -w

# kill the clients
pkill client

# save the result to shared directory
phoronix-test-suite result-file-to-csv $TEST_RESULTS_NAME

# TEST 2.11.1: server (throughput limit) <-> client + phoronux
export TEST_RESULTS_NAME=gpu-phoronix-result-openssl-vm-1-client-throttled-10
export OUTPUT_DIR=/results

wait_for_server "2_11_1_gpu"
./client -i 1 -p 1 -s 0 &

# start the benchmark
# 2 defines the encryption
printf '2\n' | phoronix-test-suite batch-run pts/openssl

# after the benchmark send stop request
./client -w

# kill the clients
pkill client

# save the result to shared directory
phoronix-test-suite result-file-to-csv $TEST_RESULTS_NAME
# TEST 2.11.2: server (throughput limit) <-> client + phoronux
export TEST_RESULTS_NAME=gpu-phoronix-result-openssl-vm-1-client-throttled-20
export OUTPUT_DIR=/results

wait_for_server "2_11_2_gpu"
./client -i 1 -p 1 -s 0 &

# start the benchmark
# 2 defines the encryption
printf '2\n' | phoronix-test-suite batch-run pts/openssl

# after the benchmark send stop request
./client -w

# kill the clients
pkill client

# save the result to shared directory
phoronix-test-suite result-file-to-csv $TEST_RESULTS_NAME
# TEST 2.11.3: server (throughput limit) <-> client + phoronux
export TEST_RESULTS_NAME=gpu-phoronix-result-openssl-vm-1-client-throttled-30
export OUTPUT_DIR=/results

wait_for_server "2_11_3_gpu"
./client -i 1 -p 1 -s 0 &

# start the benchmark
# 2 defines the encryption
printf '2\n' | phoronix-test-suite batch-run pts/openssl

# after the benchmark send stop request
./client -w

# kill the clients
pkill client

# save the result to shared directory
phoronix-test-suite result-file-to-csv $TEST_RESULTS_NAME
# TEST 2.11.4: server (throughput limit) <-> client + phoronux
export TEST_RESULTS_NAME=gpu-phoronix-result-openssl-vm-1-client-throttled-40
export OUTPUT_DIR=/results

wait_for_server "2_11_4_gpu"
./client -i 1 -p 1 -s 0 &

# start the benchmark
# 2 defines the encryption
printf '2\n' | phoronix-test-suite batch-run pts/openssl

# after the benchmark send stop request
./client -w

# kill the clients
pkill client

# save the result to shared directory
phoronix-test-suite result-file-to-csv $TEST_RESULTS_NAME

# TEST 2.12: none <-> phoronix
export TEST_RESULTS_NAME=phoronix-result-vm-only
export OUTPUT_DIR=/results

# start the benchmark
# 2 defines the encryption
printf '2\n' | phoronix-test-suite batch-run pts/openssl

# save the result to shared directory
phoronix-test-suite result-file-to-csv $TEST_RESULTS_NAME
touch /server_signals/test2_12_gpu_complete

# TEST 2.13 GPU: llama-bench gpu
# runs on server

# TEST 2.1 CPU: server <-> client (5 minutes)
wait_for_server "2_1_cpu"
./client -i 1 -p 1 -s 0 &

./client -w
pkill client

# TEST 2.2 cPU: server <-> 2 * client
wait_for_server "2_2_cpu"
parallel ./client -p 1 -s 0 -i ::: {1..2}

./client -w
pkill client

# TEST 2.3 CPU: server <-> 4 * client
wait_for_server "3"
parallel ./client -p 1 -s 0 -i ::: {1..4}

./client -w
pkill client

# TEST 2.4: server <-> 8 * client
wait_for_server "4"
parallel ./client -p 1 -s 0 -i ::: {1..8}

./client -w
pkill client

# TEST 2.5: server <-> 16 * client
wait_for_server "5"
parallel ./client -p 1 -s 0 -i ::: {1..16}

./client -w
pkill client
# TEST 2.5.32: server <-> 32 * client
wait_for_server "5_20m"
for i in $(seq 1 16); do
    ./client -i "$i" -p 1 > ~/client"$i" &
    sleep 1
done

sleep 1200
./client -w
pkill client
# TEST 2.5_64: server <-> 64 * client
wait_for_server "5_30m"
for i in $(seq 1 16); do
    ./client -i "$i" -p 1 &
    sleep 1
done

sleep 1800
./client -w
pkill client
# TEST 2.5_64: server <-> 64 * client
wait_for_server "5_40m"
for i in $(seq 1 16); do
    ./client -i "$i" -p 1 &
    sleep 1
done

sleep 2400
./client -w
pkill client
# TEST 2.5_64: server <-> 64 * client
wait_for_server "5_50m"
for i in $(seq 1 16); do
    ./client -i "$i" -p 1 &
    sleep 1
done

sleep 3000
./client -w
pkill client
# TEST 2.5.1: server <-> 16 * client 50
wait_for_server "5_1"
for i in $(seq 1 16); do
    ./client -i "$i" -p 1 &
    sleep 1
done

sleep $TEST_LENGTH
./client -w
pkill client

# TEST 2.5.2: server <-> 16 * client 100
wait_for_server "5_2"
for i in $(seq 1 16); do
    ./client -i "$i" -p 1 &
    sleep 1
done

sleep $TEST_LENGTH
./client -w
pkill client

# TEST 2.5.3: server <-> 16 * client, 20 n_token
wait_for_server "5_3"
for i in $(seq 1 16); do
    ./client -i "$i" -p 1 &
    sleep 1
done

sleep $TEST_LENGTH
./client -w
pkill client

# TEST 2.5.4: server <-> 16 * client, 30 n_token
wait_for_server "5_4"
for i in $(seq 1 16); do
    ./client -i "$i" -p 1 &
    sleep 1
done

sleep $TEST_LENGTH
./client -w
pkill client

# TEST 2.5.3: server <-> 16 * client, 40 n_token
wait_for_server "5_5"
for i in $(seq 1 16); do
    ./client -i "$i" -p 1 &
    sleep 1
done

sleep $TEST_LENGTH
./client -w
pkill client

# TEST 2.6: server <-> 2 * client
wait_for_server "6"
for i in $(seq 1 2); do
    ./client -i "$i" -p "$i" &
    sleep 1
done

sleep $TEST_LENGTH
./client -w
pkill client

# TEST 2.7: server <-> 4 * client
wait_for_server "7"
for i in $(seq 1 4); do
    ./client -i "$i" -p "$i" &
    sleep 1
done

sleep $TEST_LENGTH
./client -w
pkill client

# TEST 2.8: server <-> 8 * client
wait_for_server "8"
for i in $(seq 1 8); do
    ./client -i "$i" -p "$i" &
    sleep 1
done

sleep $TEST_LENGTH
./client -w
pkill client

# TEST 2.9: server <-> 16 * client
wait_for_server "9"
for i in $(seq 1 16); do
    ./client -i "$i" -p "$i" &
    sleep 1
done

sleep $TEST_LENGTH
./client -w
pkill client
sleep 5

# TEST 2.10 CPU: server <-> client + phoronix
printf 'y\nn\nn\nn\nn\nn\nn\n' | phoronix-test-suite batch-setup
phoronix-test-suite batch-install pts/openssl

export TEST_RESULTS_NAME=cpu-phoronix-result-openssl-vm-1-client
export OUTPUT_DIR=/results

wait_for_server "2_10_cpu"
./client -i 1 -p 1 -s 0 &

# start the benchmark
# 2 defines the encryption
printf '2\n' | phoronix-test-suite batch-run pts/openssl

# after the benchmark send stop request
./client -w

# kill the clients
pkill client

# save the result to shared directory
phoronix-test-suite result-file-to-csv $TEST_RESULTS_NAME

# TEST 2.11.1 CPU: server (throughput limit) <-> client + phoronux
export TEST_RESULTS_NAME=cpu-phoronix-result-openssl-vm-1-client-throttled-05
export OUTPUT_DIR=/results

wait_for_server "2_11_1_cpu"
./client -i 1 -p 1 -s 0 &

# start the benchmark
# 2 defines the encryption
printf '2\n' | phoronix-test-suite batch-run pts/openssl

# after the benchmark send stop request
./client -w

# kill the clients
pkill client

# save the result to shared directory
phoronix-test-suite result-file-to-csv $TEST_RESULTS_NAME
# TEST 2.11.2 CPU: server (throughput limit) <-> client + phoronux
export TEST_RESULTS_NAME=cpu-phoronix-result-openssl-vm-1-client-throttled-1
export OUTPUT_DIR=/results

wait_for_server "2_11_2_cpu"
./client -i 1 -p 1 -s 0 &

# start the benchmark
# 2 defines the encryption
printf '2\n' | phoronix-test-suite batch-run pts/openssl

# after the benchmark send stop request
./client -w

# kill the clients
pkill client

# save the result to shared directory
phoronix-test-suite result-file-to-csv $TEST_RESULTS_NAME
# TEST 2.11.3 CPU: server (throughput limit) <-> client + phoronux
export TEST_RESULTS_NAME=cpu-phoronix-result-openssl-vm-1-client-throttled-15
export OUTPUT_DIR=/results

wait_for_server "2_11_3_cpu"
./client -i 1 -p 1 -s 0 &

# start the benchmark
# 2 defines the encryption
printf '2\n' | phoronix-test-suite batch-run pts/openssl

# after the benchmark send stop request
./client -w

# kill the clients
pkill client

# save the result to shared directory
phoronix-test-suite result-file-to-csv $TEST_RESULTS_NAME
# TEST 2.11.4 CPU: server (throughput limit) <-> client + phoronux
export TEST_RESULTS_NAME=cpu-phoronix-result-openssl-vm-1-client-throttled-2
export OUTPUT_DIR=/results

wait_for_server "2_11_4_cpu"
./client -i 1 -p 1 -s 0 &

# start the benchmark
# 2 defines the encryption
printf '2\n' | phoronix-test-suite batch-run pts/openssl

# after the benchmark send stop request
./client -w

# kill the clients
pkill client

# save the result to shared directory
phoronix-test-suite result-file-to-csv $TEST_RESULTS_NAME

exit

