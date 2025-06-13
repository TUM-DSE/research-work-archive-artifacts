source config.sh

#echo $(taskset -pc 0-"$N_CPUS" $$)

# save the last result
mkdir -p ./results/backup
mv ./results/* ./results/backup

cd ..
git submodule update --init --recursive
cd -

# check if a model exists
if [ ! -f ./models/model_file ]; then
    echo "Model file not found! Plese put a model in 'models' directory with name 'model_file'"
    exit
fi

# bind the GPU
sudo bash ../qemu_scripts/bind.sh;

# test the base with phoronix:
# native openssl bench WITHOUT server and clients
nix-shell bench_native.nix

# unbind the GPU
sudo bash ../qemu_scripts/unbind.sh;

# test server VM and client VM with phoronix
# phoronix is on client side
nix-shell ./bench_vm.nix
