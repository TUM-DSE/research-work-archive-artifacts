#include <Util.h>
#include <cassert>
#include <stdio.h>
#include <inttypes.h>
#include <link.h>

extern "C" void airlift_entry(State *);
extern "C" void airlift_init_state(State *);

/* 8 MiB of stack for guest... */
constexpr std::uint32_t STACKSIZE = 1024 * 1024 * 8;

/* ARM wants stack to be 16-byte aligned. */
alignas(16) std::byte GuestStack[STACKSIZE];

/* Called from patched binary _start function */
extern "C" int airlift_init() {
  /* Run runtime initialization code */
  State S;
  airlift_init_state(&S);

  /* Point to the last cell of the stack since it grows down. */
  S.SP_EL0 = reinterpret_cast<std::uint64_t>(GuestStack + STACKSIZE);

  /* At this point we should call _start or main depending on whether we are
   * translating a C binary or some assembly-written tests... */
  airlift_entry(&S);
  return S.Registers[0];
}