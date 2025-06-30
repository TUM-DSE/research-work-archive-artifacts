# vDPDK: A Para-Virtualized DPDK Device Model for vMux

## vMux Code

https://github.com/vmuxIO/vmuxIO/tree/159c33acdf3617e214a9f118a2d64a30db279854

Main code can be found in `src/devices/vdpdk*`.

Code for DPDK-TAP forwarding used for non-DPDK benchmarks is in `subprojects/dpdk-tap-fwd`.

Measurement scripts can be found in `test/src`. The sections in my thesis
correspond to the following scripts:

* DPDK benchmarks

  * Throughput and latency

    `test/autotest test-load-lat-file -t test/conf/tests_multihost.cfg`

  * Packet classification

    `test/src/measure_mediation.py`

* Non-DPDK benchmarks

  * TCP throughput

    `test/src/measure_iperf.py`

  * Cloud serving benchmark

    `test/src/measure_ycsb.py`

  * Microservice benchmark

    `test/src/measure_hotel.py`

## DPDK Code

https://github.com/vmuxIO/dpdk/tree/89837a1ce08227dbc3ccadaa203ae5f72300295a

All code can be found in `drivers/net/vdpdk`.

## Plots

Iperf plots were generated with the script found in the `iperf` subdirectory.

All remaining plots were generated with scripts from the https://github.com/vmuxIO/plots repository.
