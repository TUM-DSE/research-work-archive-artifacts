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

  services.openssh.enable = true;
  virtualisation.forwardPorts = [
      { from = "host"; host.port = 2345; guest.port = 22; }
    ];

  virtualisation.qemu.options = [
    #"-name NIXVM,debug-threads=on"
    "-enable-kvm"
    "-cpu host"
#    "-device vfio-pci,host=ca:00.0"
    "-device ivshmem-plain,memdev=shmBerkay,bus=pci.0,addr=0x12,master=on"
    "-object memory-backend-file,size=32M,share=on,mem-path=/dev/shm/shmBerkay,id=shmBerkay"
    "-smp 32,sockets=1,cores=32,threads=1,maxcpus=32"
    "-m 64G"
  ];

  nixos-shell.mounts.extraMounts = {
    # override options for each mount
    "/results" = {
      target = ../results;
      cache = "none";
    };
    "/vm_scripts" = {
      target = ./.;
      cache = "none";
    };
    "/models" = {
	  target=../../../../models;
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
	pkgs.phoronix-test-suite
	pkgs.neovim
	pkgs.cudaPackages.cuda_cudart.stubs
	pkgs.cudatoolkit
	pkgs.pciutils
	pkgs.openssh
	];
}


