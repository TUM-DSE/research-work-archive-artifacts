#include <ArmSyscall.h>
#include <Util.h>
#include <cassert>
#include <cerrno>
#include <cstdint>
#include <cstdio>
#include <cstring>
#include <linux/aio_abi.h>
#include <unistd.h>

void translateSyscall(State *S, std::uint16_t ARMSyscallNumber) {
  std::uint16_t X86SyscallNumber = AARCH64Syscalls::X86SCInfo[ARMSyscallNumber].first;
  assert(X86SyscallNumber != 1337 && "Unsupported yet!");

  std::int32_t SyscallResult = -1;
  SyscallResult = syscall(X86SyscallNumber, S->Registers[0], S->Registers[1], S->Registers[2], S->Registers[3], S->Registers[4], S->Registers[5]);
  S->Registers[0] = SyscallResult;
}

extern "C" int airlift_syscall(State *S) {
  std::uint16_t ArmSyscallNumber = static_cast<std::uint16_t>(S->Registers[8]);
  translateSyscall(S, ArmSyscallNumber);
  return 0;
}