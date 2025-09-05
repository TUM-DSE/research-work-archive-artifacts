# DuckDB Benchmarks + uBPF for OSv

This repository contains two components intended to be used inside an [OSv](https://github.com/cloudius-systems/osv) checkout:
- **DuckDB intrumented for hook support** → `benchmarks/duckdb`
- **uBPF Runtime** → `modules/ubpf` (for dynamic eBPF program execution)

These modules extend OSv with:
- A benchmark harness for DuckDB 
- uBPF integration that allows attaching eBPF programs at runtime for adaptive system behavior



