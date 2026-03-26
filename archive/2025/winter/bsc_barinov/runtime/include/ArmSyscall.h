#include <array>
#include <cstdint>

namespace AARCH64Syscalls {

constexpr std::uint8_t io_setup = 0;
constexpr std::uint8_t io_destroy = 1;
constexpr std::uint8_t io_submit = 2;
constexpr std::uint8_t io_cancel = 3;
constexpr std::uint8_t io_getevents = 4;
constexpr std::uint8_t setxattr = 5;
constexpr std::uint8_t lsetxattr = 6;
constexpr std::uint8_t fsetxattr = 7;
constexpr std::uint8_t getxattr = 8;
constexpr std::uint8_t lgetxattr = 9;
constexpr std::uint8_t fgetxattr = 10;
constexpr std::uint8_t listxattr = 11;
constexpr std::uint8_t llistxattr = 12;
constexpr std::uint8_t flistxattr = 13;
constexpr std::uint8_t removexattr = 14;
constexpr std::uint8_t lremovexattr = 15;
constexpr std::uint8_t fremovexattr = 16;
constexpr std::uint8_t getcwd = 17;
constexpr std::uint8_t lookup_dcookie = 18;
constexpr std::uint8_t eventfd2 = 19;
constexpr std::uint8_t epoll_create1 = 20;
constexpr std::uint8_t epoll_ctl = 21;
constexpr std::uint8_t epoll_pwait = 22;
constexpr std::uint8_t dup = 23;
constexpr std::uint8_t dup3 = 24;
constexpr std::uint8_t fcntl = 25;
constexpr std::uint8_t inotify_init1 = 26;
constexpr std::uint8_t inotify_add_watch = 27;
constexpr std::uint8_t inotify_rm_watch = 28;
constexpr std::uint8_t ioctl = 29;
constexpr std::uint8_t ioprio_set = 30;
constexpr std::uint8_t ioprio_get = 31;
constexpr std::uint8_t flock = 32;
constexpr std::uint8_t mknodat = 33;
constexpr std::uint8_t mkdirat = 34;
constexpr std::uint8_t unlinkat = 35;
constexpr std::uint8_t symlinkat = 36;
constexpr std::uint8_t linkat = 37;
constexpr std::uint8_t renameat = 38;
constexpr std::uint8_t umount2 = 39;
constexpr std::uint8_t mount = 40;
constexpr std::uint8_t pivot_root = 41;
constexpr std::uint8_t nfsservctl = 42;
constexpr std::uint8_t statfs = 43;
constexpr std::uint8_t fstatfs = 44;
constexpr std::uint8_t truncate = 45;
constexpr std::uint8_t ftruncate = 46;
constexpr std::uint8_t fallocate = 47;
constexpr std::uint8_t faccessat = 48;
constexpr std::uint8_t chdir = 49;
constexpr std::uint8_t fchdir = 50;
constexpr std::uint8_t chroot = 51;
constexpr std::uint8_t fchmod = 52;
constexpr std::uint8_t fchmodat = 53;
constexpr std::uint8_t fchownat = 54;
constexpr std::uint8_t fchown = 55;
constexpr std::uint8_t openat = 56;
constexpr std::uint8_t close = 57;
constexpr std::uint8_t vhangup = 58;
constexpr std::uint8_t pipe2 = 59;
constexpr std::uint8_t quotactl = 60;
constexpr std::uint8_t getdents64 = 61;
constexpr std::uint8_t lseek = 62;
constexpr std::uint8_t read = 63;
constexpr std::uint8_t write = 64;
constexpr std::uint8_t readv = 65;
constexpr std::uint8_t writev = 66;
constexpr std::uint8_t pread64 = 67;
constexpr std::uint8_t pwrite64 = 68;
constexpr std::uint8_t preadv = 69;
constexpr std::uint8_t pwritev = 70;
constexpr std::uint8_t sendfile = 71;
constexpr std::uint8_t pselect6 = 72;
constexpr std::uint8_t ppoll = 73;
constexpr std::uint8_t signalfd4 = 74;
constexpr std::uint8_t vmsplice = 75;
constexpr std::uint8_t splice = 76;
constexpr std::uint8_t tee = 77;
constexpr std::uint8_t readlinkat = 78;
constexpr std::uint8_t newfstatat = 79;
constexpr std::uint8_t fstat = 80;
constexpr std::uint8_t sync = 81;
constexpr std::uint8_t fsync = 82;
constexpr std::uint8_t exit = 93;

/* To get information about an x86-64 syscall, index this array using ARM syscall number */
/* First element of the pair is corresponding x86-64 call number */
/* Second element of the pair is how many arguments the syscall accept (on x86-64) */
constexpr std::array<std::pair<std::uint16_t, std::uint8_t>, 256> X86SCInfo = {{
  {206, 2}, /* io_setup */
  {207, 1}, /* io_destroy */
  {209, 3}, /* io_submit */
  {210, 3}, /* io_cancel */
  {208, 5}, /* io_getevents */
  {188, 5}, /* setxattr */
  {1337, 5}, /* lsetxattr */
  {1337, 5}, /* fsetxattr */
  {1337, 4}, /* getxattr */
  {1337, 4},
  {1337, 4},
  {1337, 3},
  {1337, 3},
  {1337, 3},
  {1337, 2},
  {1337, 2},
  {1337, 2},
  {1337, 2},
  {1337, 3},
  {1337, 2},
  {1337, 1},
  {1337, 4},
  {1337, 6},
  {1337, 1},
  {1337, 3},
  {1337, 3},
  {1337, 1},
  {1337, 3},
  {1337, 2},
  {1337, 3},
  {1337, 3},
  {1337, 2},
  {1337, 2},
  {1337, 4},
  {1337, 3},
  {1337, 3},
  {1337, 3},
  {1337, 5},
  {1337, 4},
  {1337, 0},
  {1337, 5},
  {1337, 2},
  {1337, 0},
  {1337, 2},
  {1337, 2},
  {1337, 2},
  {1337, 2},
  {1337, 4},
  {1337, 3},
  {1337, 1},
  {1337, 1},
  {1337, 1},
  {1337, 2},
  {1337, 3},
  {1337, 5},
  {1337, 3},
  {1337, 4},
  {1337, 1},
  {1337, 0},
  {1337, 2},
  {1337, 4},
  {1337, 3},
  {1337, 3},
  {0, 3}, /* read */
  {1, 3}, /* write */
  {1337, 3},
  {1337, 3},
  {1337, 4},
  {1337, 4},
  {1337, 5},
  {1337, 5},
  {1337, 4},
  {1337, 6},
  {1337, 5},
  {1337, 4},
  {1337, 4},
  {1337, 6},
  {1337, 4},
  {1337, 4},
  {1337, 4},
  {1337, 2},
  {1337, 0},
  {1337, 1},
  {1337, 1},
  {1337, 4},
  {1337, 2},
  {1337, 4},
  {1337, 2},
  {1337, 4},
  {1337, 1},
  {1337, 2},
  {1337, 2},
  {1337, 1},
  {60, 1}, /* exit */
  {1337, 1},
  {1337, 5},
  {1337, 1},
  {1337, 1},
  {1337, 6},
  {1337, 2},
  {1337, 3},
}};

} // namespace AARCH64Syscalls