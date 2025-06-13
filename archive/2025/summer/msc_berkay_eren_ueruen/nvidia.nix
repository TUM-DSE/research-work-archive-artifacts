{ pkgs, config, ... }:
{
  nixpkgs.config.allowUnfree = true;
  nixpkgs.config.nvidia.acceptLicense = true;
  # enable the nvidia driver
  services.xserver.videoDrivers = [ "nvidia" ];
  hardware.opengl.enable = true;
  hardware.nvidia.datacenter.enable = true;
  hardware.nvidia.package = config.boot.kernelPackages.nvidiaPackages.dc_535;

  hardware.nvidia.open = true;

  virtualisation.docker.enable = true;
  hardware.nvidia-container-toolkit.enable = true;
  hardware.opengl.driSupport32Bit = true;

  virtualisation.memorySize = 64 * 1024;
  virtualisation.diskSize = 128 * 1024;
  # Nix Store must be persistent across all QEMU
  # executions
  virtualisation.writableStoreUseTmpfs = false;

  virtualisation.qemu.options = [
    #"-name NIXVM,debug-threads=on"
    "-enable-kvm"
    "-cpu host"
    "-device vfio-pci,host=ca:00.0"
    # addr must match the address in llama_server.cpp and write.c:
    # int fd = open("/sys/bus/pci/devices/0000:00:12.0/resource2", O_RDWR);
    # or if the addr were 0x11 then:
    # int fd = open("/sys/bus/pci/devices/0000:00:11.0/resource2", O_RDWR);
    # It is worth mentioning that for a two VM test setup addr must be same for both VMs.
    # At the same time, for any other VMs outside of the test setup, addr must
    # be different. So I generally just change the addr locally before launching any VMs.
    "-device ivshmem-plain,memdev=shm1,bus=pci.0,addr=0x12,master=on"
    "-object memory-backend-file,size=32M,share=on,mem-path=/dev/shm/shm1,id=shm1"
    "-smp 32,sockets=1,cores=32,threads=1,maxcpus=32"
    "-m 64G"
  ];

  nixos-shell.mounts.extraMounts = {
    # override options for each mount
    "/llm-os" = {
      target = ../.;
      cache = "none";
    };

  };
  nixos-shell.mounts = {
    mountHome = false;
    mountNixProfile = false;
    cache = "none"; # default is "loose"
  };


  environment.systemPackages = [
  	config.boot.kernelPackages.nvidiaPackages.dc_535
#	config.boot.kernelPackages.nvidia_x11
	pkgs.git
	pkgs.neovim
	pkgs.cudaPackages.cuda_cudart.stubs
	pkgs.cudatoolkit
	pkgs.vim
	];
}


