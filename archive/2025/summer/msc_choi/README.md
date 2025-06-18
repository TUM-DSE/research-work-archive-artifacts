# Airlift: A Binary Lifter Based on a Machine-Readable Architecture Specification

## Setup and Updates

For a fresh setup:

```
git clone https://github.com/TUM-DSE/airlift.git
cd airlift
git submodule update --init
```

For updates to the repository and the submodules:

```
git fetch origin
git pull
git submodule update
```

## Running the Pipeline

Extract ARM's Machine Readable Architecture Specification:

```
cd external/mra_tools
mkdir -p v8.6
cd v8.6
wget https://developer.arm.com/-/media/developer/products/architecture/armv8-a-architecture/2019-12/SysReg_xml_v86A-2019-12.tar.gz
wget https://developer.arm.com/-/media/developer/products/architecture/armv8-a-architecture/2019-12/A64_ISA_xml_v86A-2019-12.tar.gz
wget https://developer.arm.com/-/media/developer/products/architecture/armv8-a-architecture/2019-12/AArch32_ISA_xml_v86A-2019-12.tar.gz
tar zxf A64_ISA_xml_v86A-2019-12.tar.gz
tar zxf AArch32_ISA_xml_v86A-2019-12.tar.gz
tar zxf SysReg_xml_v86A-2019-12.tar.gz
cd ..
make all
cd ../..
```

Make sure to install dependencies as instructed in `external/asl-interpreter/README.md` before moving on to the next step.

Parse and generate the AST of `arch_instrs.asl` and its dependency files in JSON format:

```
cd external/asl-interpreter
make install
dune exec bin/testparser.exe \
    prelude.asl \
    ../mra_tools/arch/regs.asl \
    ../mra_tools/types.asl \
    ../mra_tools/arch/arch.asl \
    ../mra_tools/arch/arch_instrs.asl \
    ../mra_tools/arch/arch_decode.asl \
    ../mra_tools/support/aes.asl \
    ../mra_tools/support/barriers.asl \
    ../mra_tools/support/debug.asl \
    ../mra_tools/support/feature.asl \
    ../mra_tools/support/hints.asl \
    ../mra_tools/support/interrupts.asl \
    ../mra_tools/support/memory.asl \
    ../mra_tools/support/stubs.asl \
    ../mra_tools/support/fetchdecode.asl
cd ../..
```

The output should be generated at `external/asl-interpreter/ast.json`.

Code generator can be run with the simple command:

```
cargo build
```

This will analyze the AST from the previous step and generate the necessary files in `lifter/src/arm64/[common,decode,lift]/generated`.

Benchmarks can be run with the command:

```
cargo run --release
```

Plots can be generated with the command:

```
cd lifter/results
pip install matplotlib pandas
python plot.py
```

Finally, you can test the lifter by running the tests:

```
cargo nextest run --workspace
```

## Helpful Commands

Format code:

```
cargo fmt -- --config max_width=150
```
