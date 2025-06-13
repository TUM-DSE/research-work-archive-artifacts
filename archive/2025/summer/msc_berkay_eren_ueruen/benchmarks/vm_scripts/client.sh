set -x

cd ~

mkdir -p llm-os
rsync --progress --whole-file -a /llm-os ./ --exclude /llm-os/benchmarks/models --exclude /llm-os/loras --exclude /llm-os/benchmarks/vm_server --exclude /llm-os/benchmarks/vm_client --exclude /llm-os/.git
cd llm-os

# run the benchmark
export NIXPKGS_ALLOW_UNFREE=1
nix-shell /vm_scripts/vm-client-shell.nix

shutdown now

