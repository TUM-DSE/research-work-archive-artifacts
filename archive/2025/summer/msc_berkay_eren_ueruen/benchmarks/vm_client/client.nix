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
      { from = "host"; host.port = 2346; guest.port = 22; }
    ];

  virtualisation.qemu.options = [
    #"-name NIXVM,debug-threads=on"
    "-enable-kvm"
    "-cpu host"
    "-smp 16,sockets=1,cores=16,threads=1,maxcpus=16"
    "-device ivshmem-plain,memdev=shmBerkay,bus=pci.0,addr=0x12,master=on"
    "-object memory-backend-file,size=32M,share=on,mem-path=/dev/shm/shmBerkay,id=shmBerkay"
    "-m 64G"
  ];

  nixos-shell.mounts.extraMounts = {
    # override options for each mount
    "/llm-os" = {
      target = ../../.;
      cache = "none";
    };
    "/results" = {
      target = ../results;
      cache = "none";
    };
    "/vm_scripts" = {
      target = ../vm_scripts;
      cache = "none";
    };
    "/server_signals" = {
      target = ../server_signals;
      cache = "none";
    };
  };
  nixos-shell.mounts = {
    mountHome = false;
    mountNixProfile = false;
    cache = "none"; # default is "loose"
  };


  environment.systemPackages = [
	pkgs.git
	pkgs.neovim
	pkgs.fish
	pkgs.stow
	pkgs.tmux
	pkgs.phoronix-test-suite
	];
}


