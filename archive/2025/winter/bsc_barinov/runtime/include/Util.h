#include <iostream>
#include <cstdint>
#include <vector>

typedef struct State {
  bool bools[4];
  std::uint64_t Registers[31];
  std::uint64_t SP_EL0, SP_EL1, SP_EL2, SP_EL3;
  std::uint64_t Bigs[32][32];
  std::uint64_t Mediums[16][4];
  std::uint64_t Medium[4];
} State;

extern std::vector<const char*> DynLibs;

#define LOG(msg) \
    std::cout << __FILE__ << "(" << __LINE__ << "): " << msg << std::endl 