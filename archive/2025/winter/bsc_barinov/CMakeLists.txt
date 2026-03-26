cmake_minimum_required(VERSION 3.31.6)
project(sgtranslator)
enable_language(ASM)

set(CMAKE_CXX_STANDARD 23)
set(CMAKE_CXX_STANDARD_REQUIRED True)

set(EXECUTABLE_OUTPUT_PATH ${PROJECT_BINARY_DIR}/bin)

# NixOS related: clangd has problems finding system headers because they are wrapped on NixOS...
# This dumps them along with other compile_commands.json
if(CMAKE_EXPORT_COMPILE_COMMANDS)
  set(CMAKE_CXX_STANDARD_INCLUDE_DIRECTORIES
    ${CMAKE_CXX_IMPLICIT_INCLUDE_DIRECTORIES})
endif()

# Required because LLVM is compiled without RTTI to avoid conflicts
set(NO_RTTI "-fno-rtti")
add_definitions(${NO_RTTI})

add_subdirectory(externals/argparse argparse)
add_subdirectory(externals/json json)
add_subdirectory(translator)
add_subdirectory(runtime)