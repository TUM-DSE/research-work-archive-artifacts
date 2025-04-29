# CHERI Microanalysis

This project contains a suite of microbenchmarks for analyzing performance characteristics of CHERI (Capability Hardware Enhanced RISC Instructions) on Morello hardware. It focuses on two main areas:

1. Instruction-level benchmarks
2. Memory access latency benchmarks
3. Application-level benchmarks using Polybench/C

## Project Structure

```
cheri-microanalysis/
├── instructions/         # Instruction-level benchmarking
├── memory_latency/      # Memory access latency analysis
├── applications/        # Application-level benchmarks
└── flake.nix           # Development environment setup
```

## Components

### Instructions Benchmark

Tests performance characteristics of various CHERI instructions, measuring both latency and throughput:
- Regular load/store operations (LDR, STR)
- Capability load/store operations (LDR.CAP, STR.CAP)
- Pair operations (LDP, STP)
- Atomic operations (LDAR, STLR)
- Capability manipulation instructions (CVTD, CFHI, CTHI, etc.)

### Memory Latency Benchmark

Measures memory access latency across different working set sizes to analyze:
- Cache hierarchy performance
- Impact of capability overhead on memory access

### Polybench Applications

Runs the Polybench/C benchmark suite compiled for:
- Regular AArch64 (nocap)
- Pure-capability CHERI (purecap)

Includes various computational kernels:
- Linear algebra computations
- Stencil computations
- Data mining
- Dynamic programming

## Usage

The project uses `just` as its command runner. Key commands:

```bash
# Build and run all benchmarks
just run_all

# Build and run specific benchmark
just run_memory_latency # Or instructions/applications

# Build specific benchmark
just build_memory_latency # Or instructions/applications

# Plot benchmark results. Only possible after run!
just plot_memory_latency # Or applications

```

## Development Environment

The project uses Nix flakes to provide a reproducible development environment with:
- CHERI-aware LLVM/Clang
- CHERI-MUSL C library
- Python with data analysis packages
- Build tools and utilities

To enter the development environment:

```bash
nix develop
```

## Results

The benchmarks generate CSV/TEX files and SVG plots showing:
- Instruction latency and throughput metrics
- Memory access latency curves
- Application performance comparisons between regular and CHERI versions

Results are generated in each component's respective output directory.
