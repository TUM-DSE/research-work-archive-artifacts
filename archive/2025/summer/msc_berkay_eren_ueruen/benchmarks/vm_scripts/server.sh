set -x

cd ~

mkdir -p llm-os
rsync --progress --whole-file -a /llm-os ./ --exclude /llm-os/benchmarks/models --exclude /llm-os/loras --exclude /llm-os/benchmarks/vm_server --exclude /llm-os/benchmarks/vm_client --exclude /llm-os/.git
cd llm-os

# copy model
mkdir -p models
if [ ! -f "./models/model_file" ] ; then
    cp /models/model_file "./models/model_file"
fi

# cleanup
rm /server_signals/*

export NIXPKGS_ALLOW_UNFREE=1
echo "Entering nix env"
nix-shell /vm_scripts/vm-server-shell.nix

# cleanup
rm /server_signals/*

#shutdown now

