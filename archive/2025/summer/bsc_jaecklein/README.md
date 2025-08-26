# Protecting H/W and S/W Intercations for Network-Attached Accelerators

This file explains how to run the software part (CVM + Shared Memory + Device Emulation) for the network-attached accelerator.

The implementation can be divided by the transportation method into Network and **Same Host** setup. The first runs a network stack (**RDMA**, **Ethernet**, or **TCP**) to connect the hosts of the Shared Memory and the Device Emulation. The latter runs all applications on one host, thus no data has to travel the network.

All four types of transportation implement a **secure** version, where all data concerning MMIO and DMA is going through a authenticated Encryption procedure, and a **non-secure** version, which removes those security measures and operates on plaintext.

The explanation is structured as follows. First, a describtion on how to initially set up the (C)VM (QEMU + Kernel). This has to only be done once. Next, the building instructions for the four different transportation types. Last, the instructions on how to run the system.

To get the necessary repositories:

```bash
git clone https://github.com/maxjae/jigsaw-overall.git
cd ./jigsaw-overall
direnv allow
git clone https://github.com/maxjae/spdm-linux.git
git clone https://github.com/maxjae/qemu.git
```

## VM Setup

The virtual machine is hosted with QEMU and a custom kernel to accommodate for the changes to the communication layer. To run the VM in a confidential version, the extensions of AMD SEV are used. Further, there are some basic driver modules for testing and evaluation.

### QEMU

```bash
cd qemu
git checkout disagg-fake-device
mkdir build
cd build
../configure --target-list=x86_64-softmmu --enable-kvm
make -j$(nproc)
```

### Kernel

Change to the custom kernel:

```bash
cd spdm-linux
```

The code base offers two different branches for the secure and non-secure version of the communication. This is independent of the VM itself and only changes the MMIO and DMA communication.

```bash
git checkout disagg-secure
```

**or**:

```bash
git checkout disagg-non_secure
```

Get the custom config file and build:

```bash
cp linux-disagg-shmem-config .config
make olddefconfig
make -j$(nproc)
```

### Miscellaneous

#### Image

To run the VM, a image is needed. You can follow the instructions from here: <https://nixos.wiki/wiki/Kernel_Debugging_with_QEMU>.

#### Driver Modules

Some test drivers and evaluation modules are in the `spdm-linux/qemu_edu` directory. To copy the build modules into your image you can use the provided script:

```bash
./copy-modules.sh <path of image>
```

#### OVMF

Running a **confidential** VM requires a suitable OVMF. The provided QEMU script also uses this to run the non-confidential VM, even if it is not strictly necessary. For now, you can use this rather weird approach to create an `OVMF.fd` file. This file can later be used when running the qemu script.

```bash
git clone --recursive https://github.com/dimstav23/GDPRuler.git
cd GDPRuler && git checkout dev
git submodule update --init --recursive
nix develop
cd CVM_setup/AMDSEV
bash build.sh ovmf
cp ./usr/local/share/qemu/OVMF.fd <path of jigsaw-overall repo>
```

## Remaining System

The `jigsaw-overall` repository offers 8 different branches for each pair of {RDMA, Ethernet, TCP, Same Host} and {secure, non-secure}. The Same Host setup requires no second sever (i.e., the device emulation runs on the same host as the CVM). This also means that for the {RDMA, Eternet, TCP} setups, a second `jigsaw-overall` repository has to be cloned on another server.

So, follow either "Network Setups" **or** "Same Host Setup".

### Network Setups

On the CVM server:

```bash
git checkout (rdma|tcp|ethernet)(-non_secure)?
cd src/proxy
make
```

The device emulation runs on a different server:

```bash
git clone https://github.com/maxjae/jigsaw-overall.git
cd ./jigsaw-overall
direnv allow
git checkout (rdma|tcp|ethernet)(-non_secure)?
cd src/edu_simple
make
```

### Same Host Setups

Run this on the same server as the CVM.

```bash
git checkout same-host-simple-device(-non_secure)?
cd src/edu-simple
make
```

## Running the system

The different applications have to be started in a specific order:

1. Device emulation
2. Proxy (not for Same Host)
3. CVM

### Device and (Proxy)

Again, there is difference between the Same Host setups and the rest. Either follow "RDMA, TCP, Ethernet" **or** "Same Host".

#### RDMA, TCP, Ethernet

On the non-CVM server:

```bash
cd src/edu_simple
./bin/edu_device_emulation
```

The execution will print a usage message on how to specify the different command line options. This differs for the network types.

On the CVM server:

```bash
cd src/proxy
./bin/proxy
```

The execution will print a usage message on how to specify the different command line options. This differs for the network types.

#### Same Host

```bash
cd src/edu_simple
./bin/edu_device_emulation
```

### CVM

The jigsaw-overall repo provides the `qemu-run.sh` script to start the CVM:

```bash
sudo ./qemu-run <Type of VM: [VM|CVM]> <path of image file> <path of OVMF.fd>
```

The type argument specifies if the VM uses SEV extensions (option "CVM") or just a plain VM (option "VM").

### Running modules

Inside the CVM, the modules can now be run with:

```bash
cd /modules
insmod ./<name of module>.ko
rmmod ./<name of module>.ko
```

## Note

- As the evaluation works with 8 KiB payload, the link has to be able to support this. This is especially relevant for the **Ethernet** setup. Therefore, you have to increase the MTU for your network interfaces to a value higher than the standard 1500 Bytes. We used 9000 Bytes for the MTU of both NICs. (e.g., use the `ip` command)
- To run the **RDMA** setup the NICs have to support this. To check for available RDMA devices and their corresponding interface link name, use `rdma link`.
