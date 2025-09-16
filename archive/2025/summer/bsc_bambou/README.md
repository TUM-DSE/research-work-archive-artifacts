# Artifact Archive: BSc Thesis Steve Bambou

This folder contains the research artifacts for my Bachelor thesis at [TUM DSE](https://dse.in.tum.de/).

## Code Base

The main source code for my thesis is available in the following repository:

- [Code Repository](https://github.com/dhschall/gem5-fdp/tree/pf-rework-multi-pred)

This repository includes:
- Source code
- Build system files

## Evaluation Code

The evaluation scripts and analysis tools are maintained separately in:

- [Evaluation Repository](https://github.com/dhschall/gem5-svr-bench/tree/multi-pred)

This repository contains:
- Gem5 configuration scripts
- Benchmark configuration scripts
- Data processing scripts


## How to Reproduce Results

1. Clone the code base repository.
2. Clone the evaluation repository inside of the code base repository.
3. Follow the instructions in each repository’s `README.md` to build and run the code.
4. Use the evaluation scripts to run the benchmarks.
5. Use the data collection script to aggregate the benchmark results in single .csv files for SPEC and non SPEC workloads.  
6. Use this [script](results_notebook.ipynb) to generate plots.