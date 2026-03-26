find_package(LLVM REQUIRED CONFIG)
message(STATUS "Found LLVM ${LLVM_PACKAGE_VERSION}")
message(STATUS "Using LLVMConfig.cmake in: ${LLVM_DIR}")

# Find airlift
find_library(LIFTER_LIB NAMES lifter)
if(NOT LIFTER_LIB)
  message(FATAL_ERROR "Airlift (liblifter.so) not found!  Please specify the directory to search for it by setting -DCMAKE_PREFIX_PATH=/path/to/airlift/")
endif()
message(STATUS "Found Airlift ${LIFTER_LIB}")

# Find the linker
if(NOT LLD_INCLUDE_DIR)
  message(FATAL_ERROR "Specify the path to lld includes via -DLLD_INCLUDE_DIR")
endif()
message(STATUS "LLD INCLUDE DIR: ${LLD_INCLUDE_DIR}")

find_library(LLD_COMMON_LIB NAMES lldCommon PATHS ${LLVM_LIBRARY_DIRS})
if(NOT LLD_COMMON_LIB)
  message(FATAL_ERROR "LLD COMMON library not found")
endif()
message(STATUS "Found COMMON ELF library: ${LLD_COMMON_LIB}")

find_package(Clang REQUIRED CONFIG)
message(STATUS "Found Clang ${CLANG_PACKAGE_VERSION}")

set(CLANG_LIBS
  clangTooling
  clangFrontend
  clangAST
  clangBasic
  clangSerialization
  clangDriver
  clangParse
  clangSema
  clangAnalysis
  clangEdit
  clangLex
)


find_library(LLD_ELF_LIB NAMES lldELF PATHS ${LLVM_LIBRARY_DIRS})
if(NOT LLD_ELF_LIB)
  message(FATAL_ERROR "LLD ELF library not found")
endif()
message(STATUS "Found LLD ELF library: ${LLD_ELF_LIB}")

# Keep at end of the file
set(SOURCES
  src/sgtranslator.cpp
  src/LifterHelper.cpp
  src/HeaderParser.cpp
)

find_package(ZLIB REQUIRED)

add_executable(sgtranslator ${SOURCES})

target_include_directories(sgtranslator PRIVATE
   ${LLVM_INCLUDE_DIRS}
   include ../externals/argparse/include/argparse
   ../externals/json/include/
   ${LLD_INCLUDE_DIR}
   ${CLANG_INCLUDE_DIRS}
  )

target_link_libraries(sgtranslator PRIVATE
  ${LIFTER_LIB}
  ${LLD_ELF_LIB}
  ${LLD_COMMON_LIB}
  ${CLANG_LIBS}
  LLVM
  ZLIB::ZLIB
)
