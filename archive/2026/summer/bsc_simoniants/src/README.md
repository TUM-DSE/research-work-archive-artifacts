# Measurement and reference implementation source code

The directories directly implement the layered implementation described in the thesis.

- [`net/`](net/) implements the `hot_[sender|replier|receiver]` applications, with the [`hot_replier`](net/sv_hot_replier.c) acting as the upper networking layer of the representative protection workload.

- [`measurement/`](measurement/) implements the measurement preprocessing layer.

- [`protection/`](protection/) implements the ANSI protection computation kernel layer.

`timing.[c|h]` implement general benchmarking capability of function invocations.

## `main` and `multivm` branches (in this repo: directories)

The only difference between the two branches (directories) consists of changes to the `hot_receiver` application to enable experiments containing multiple concurrently responding VMs.

The `multivm` file is found in [`multivm/net/`](multivm/net/) at [`hot_receiver`](multivm/net/sv_hot_receiver.c)
