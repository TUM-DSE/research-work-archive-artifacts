# jigsaw-overall

## One-time setup (Kernel and QEMU):

### Setup packages
```
direnv allow
``` 

### Build a bootable image:

- Follow the instructions from here: https://nixos.wiki/wiki/Kernel_Debugging_with_QEMU

### Clone repositories
```
git clone --recursive https://github.com/maxjae/spdm-linux.git
git clone --recursive https://github.com/maxjae/qemu.git
```

### Build qemu

```
cd qemu
git checkout disagg-fake-device
mkdir build
cd build
../configure --target-list=x86_64-softmmu --enable-kvm --disable-werror
make -j$(nproc)
```

### Build linux

#### Initial setup: 

```
cd spdm-linux
git checkout disagg-secure
cp linux-disagg-shmem-config .config
make olddefconfig
```

#### Additional configuration:

```
make menuconfig
```

If wanted, select following configs with =y:

For CVM setup: 
- Processor type and features -> AMD Secure Memory Encryption (SME) support

For faster cryptography:
- Cryptographic API -> Accelerated Cryptographic Algorithms for CPU (x86) -> Ciphers: AES, modes: ECB, CBC, CTS, XTR, XTS, GCM (AES-NI)

#### Build:

```
make -j$(nproc)
```

### Building kernel modules

Compiled modules have to be copied into image.

#### 1. Option: Use provided script
```
./copy-modules <path to image>
```

#### 2. Option: Build modules and insert yourself
```
cd spdm-linux/qemu_edu
make
```
Copy *.ko files into image. 

## Build OVMF

You have to create your own OVMF with AMD SEV-SNP support.

## Build the Proxy and Device emulation

### For RDMA, Ethernet, and TCP
```
cd {rdma,ethernet,tcp}/{secure,non_secure}/proxy
make
```

On different host:
```
cd {rdma,ethernet,tcp}/{secure,non_secure}/edu_simple
make
```

### For Same host setup
```
cd same_host/{secure,non_secure}
make
```

## Run the system

### Device Emulation
```
cd {rdma,ethernet,tcp}/{secure,non_secure}/edu_simple
```

Execute the binary in `./bin/` and follow the CLI instructions.

### Proxy
Omit this for Same host setup.

Do on different host than Device emulation.

```
cd {rdma,ethernet,tcp}/{secure,non_secure}/proxy
```

Execute the binary in `./bin/` and follow the CLI instructions.

### CVM
Do on different host than Device emulation.
```
sudo ./qemu-run.sh <path to spdm-linux> <path to image> <path to OVMF.fd>
```

