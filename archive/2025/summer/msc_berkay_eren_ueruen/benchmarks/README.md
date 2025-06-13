The complete benchmark suite can be started by calling the script `bench_main.sh`. If no model file is specified, script will ask for one. It is possible to use any model that llama.cpp supports.

During its execution, `bench_main.sh` will first run native tests by calling `bench_native.sh`. Later on, tests that require VMs will be run via the script `bench_vm.sh`. The latter will spawn two VMs using QEMU, one with a GPU and one without. Created resuts will be written to the `results` directory in the root of the `benchmark` folder.

It is worth noting that both `bench_native.sh` and `bench_vm.sh` can also called seperatly.

Details of the each test resting inside the scripts can be found either directly in the scripts, or in the scripts that they call. For the native tests, the exact information can be found in `bench_native.sh`. The details for VM tests can be found in `vm_scripts` directory.

### Issues
Phoronix test suite has some weird test result saving behaviour. In some cases, it might be possible that the result written to `results` folder is not the latest result. However, inside the VM, it is possible to find all the results so far. Therefore getting the Phoronix results from the VM itself is more reliable. This can be done by launching the VM inside the `vm_client` folder via the command `nixos-shell client.nix`.
