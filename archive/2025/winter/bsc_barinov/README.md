# sgtranslator - A specification driven AARCH64 to x86-64 Static Binary Translator

A specification-driven binary translator that converts AARCH64 executables to x86-64, enabling AARCH64 binaries to run natively on x86 systems.

## Requirements

### Build Dependencies
- Airlift (https://github.com/TUM-DSE/airlift)
- A C++23 compliant compiler
- LLVM development libraries

### Test Suite Dependencies
- An AARCH64 (cross-)compiler
- `qemu-aarch64` - AARCH64 emulator for translated binary comparison 

## Building

### Using CMake (Standard)

From project root directory:

```bash
mkdir build

cmake \
  -S . \
  -B ./build \
  -DLLVM_DIR=PATH_TO_LLVM/llvm-project/build/lib/cmake/llvm \
  -DLLD_INCLUDE_DIR=PATH_TO_LLVM/llvm-project/lld/include \
  -DCMAKE_PREFIX_PATH=PATH_TO_AIRLIFT/airlift/target/debug \

cmake --build ./build -j
```

## Usage

### Basic Translation

Translate an AARCH64 binary to x86:

```bash
sgtranslator <arm_binary> -o <output_basename> -c config.json
```

### Running Tests

The test runner works by compiling the test source code (C or ARM assembly) with a cross-compiler to AARCH64, executing it with QEMU, translating the test, executing it natively and comparing the results (stdout, stderr, return code).

#### Running all tests

```bash
./test_runner.py --test-dir ./tests
```

#### Filtering the tests to be run 

```bash
./test_runner.py --test-dir ./tests --filter hello_world
```

## Configuration

The `config.json` file specifies paths to C runtime files and the translator's runtime library:

```json
{
  "crt1": "/path/to/crt1.o",
  "crti": "/path/to/crti.o",
  "crtn": "/path/to/crtn.o",
  "libc": "/path/to/libc.so.6",
  "dynamiclinker": "/path/to/ld-linux-x86-64.so.2",
  "runtime": "./build/runtime/libsgtruntime.so"
}
```

Adjust paths based on your system's libc installation.

## Project Structure

```
.
├── translator/             # Translator
│   ├── include/            
│   └── src/                
├── runtime/                # Translator runtime
│   ├── include/
│   └── src/
├── examples/               # Test cases (C and AARCH64 assembly)
├── externals/              # Git submodules
│   ├── argparse/           
│   └── json/               
├── config.json             # Translator configuration
└── test_runner.py          # Test harness

```