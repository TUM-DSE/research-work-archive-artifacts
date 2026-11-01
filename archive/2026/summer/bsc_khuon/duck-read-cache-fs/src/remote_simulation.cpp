#include "remote_simulation.hpp"

#include <chrono>
#include <thread>

#include "cache_httpfs_instance_state.hpp"

namespace duckdb {

	/*
void SimulateRemoteRead(const InstanceConfig &config, idx_t bytes) {
	const idx_t latency_us = config.sim_latency_us;
	const double bandwidth_gbps = config.sim_bandwidth_gbps;
	if (latency_us == 0 && bandwidth_gbps <= 0.0) {
		return;
	}

	// bytes / (GB/s) gives nanoseconds; / 1000 gives microseconds.
	double delay_us = static_cast<double>(latency_us);
	if (bandwidth_gbps > 0.0) {
		delay_us += static_cast<double>(bytes) / (bandwidth_gbps * 1000.0);
	}
	if (delay_us <= 0.0) {
		return;
	}
	std::this_thread::sleep_for(std::chrono::microseconds(static_cast<int64_t>(delay_us)));
}
*/
void SimulateRemoteRead(const InstanceConfig &config, idx_t bytes,
                        std::chrono::microseconds elapsed_read_time) {
	const idx_t latency_us = config.sim_latency_us;
	const double bandwidth_gbps = config.sim_bandwidth_gbps;

	if (latency_us == 0 && bandwidth_gbps <= 0.0) {
		return;
	}

	// bytes / (GB/s) gives nanoseconds; / 1000 gives microseconds.
	double simulated_delay_us = static_cast<double>(latency_us);

	if (bandwidth_gbps > 0.0) {
		simulated_delay_us +=
		    static_cast<double>(bytes) / (bandwidth_gbps * 1000.0);
	}

	const double remaining_delay_us =
	    simulated_delay_us - static_cast<double>(elapsed_read_time.count());

	if (remaining_delay_us <= 0.0) {
//		dprintf(2,"NOSLEEP");
		return;
	}

	std::this_thread::sleep_for(
	    std::chrono::microseconds(static_cast<int64_t>(remaining_delay_us)));
}

} // namespace duckdb
