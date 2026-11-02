#include "policy/eviction_policy.hpp"

#include "duckdb/common/exception.hpp"
#include "duckdb/common/helper.hpp"
#include "policy/clock_policy.hpp"
#include "policy/fifo_policy.hpp"
#include "policy/lru_policy.hpp"
#include "policy/mru_policy.hpp"
#include "policy/lru_policy_sizeaware.hpp"
#include "policy/mru_policy_sizeaware.hpp"
#include "policy/lru_policy_sizeaware_inv.hpp"
#include "policy/mru_policy_sizeaware_inv.hpp"
#include "policy/s3fifo_policy.hpp"
#include "policy/s3fifo_pro_policy.hpp"
#include "policy/s3fifo_both_policy.hpp"
#include "policy/s3fifo_roun_policy.hpp"

namespace duckdb {

bool IsValidEvictionPolicy(const string &name) {
	return name == "slru512" || name == "slru1536" || name == "s3fifo_roun512" || name == "s3fifo_roun1536" || name == "s3fifo_roun1024" || name == "s3fifo_both1024" || name == "s3fifo_pro1024" || name == "s3fifo_both128" || name == "s3fifo_pro128" || name == "s3fifo_both512" || name == "s3fifo_pro512" || name == "s3fifo" || name == "smrui" || name == "slrui" || name == "smru" || name == "slru" || name == "lru" || name == "fifo" || name == "clock" || name == "mru";
}

unique_ptr<EvictionPolicy> CreateEvictionPolicy(const string &name, idx_t size) {
	if (name == "lru") {
		return make_uniq<LruPolicy>();
	}
	if (name == "fifo") {
		return make_uniq<FifoPolicy>();
	}
	if (name == "clock") {
		return make_uniq<ClockPolicy>();
	}
	if (name == "mru") {
		return make_uniq<MruPolicy>();
	}
	if (name == "slru") {
		return make_uniq<SizeAwareLruPolicy>();
	}
	if (name == "slru512") {
		return make_uniq<SizeAwareLruPolicy>(512 * 1024);
	}
	if (name == "slru1536") {
		return make_uniq<SizeAwareLruPolicy>(1536 * 1024);
	}
	if (name == "smru") {
		return make_uniq<SizeAwareMruPolicy>();
	}
	if (name == "slrui") {
		return make_uniq<SizeAwareLruInvPolicy>();
	}
	if (name == "smrui") {
		return make_uniq<SizeAwareMruInvPolicy>();
	}
	if (name == "s3fifo") {
		return make_uniq<S3FifoPolicy>(size);
	}
	if (name == "s3fifo_pro128") {
		return make_uniq<S3FifoProPolicy>(
		size,
		128 * 1024, // reference size: 4 MiB
		3                // maximum promotion threshold
		);
	}
	if (name == "s3fifo_both128") {
		return make_uniq<S3FifoBothPolicy>(
		size,
		128 * 1024, // reference size: 4 MiB
		6,               // maximum accumulated frequency
		3                // maximum size weight
		);
	}
	if (name == "s3fifo_pro512") {
		return make_uniq<S3FifoProPolicy>(
		size,
		512 * 1024, // reference size: 4 MiB
		3                // maximum promotion threshold
		);
	}
	if (name == "s3fifo_both512") {
		return make_uniq<S3FifoBothPolicy>(
		size,
		512 * 1024, // reference size: 4 MiB
		6,               // maximum accumulated frequency
		3                // maximum size weight
		);
	}
	if (name == "s3fifo_pro1024") {
		return make_uniq<S3FifoProPolicy>(
		size,
		1024 * 1024, // reference size: 4 MiB
		3                // maximum promotion threshold
		);
	}
	if (name == "s3fifo_both1024") {
		return make_uniq<S3FifoBothPolicy>(
		size,
		1024 * 1024,
		6,               // maximum accumulated frequency
		3                // maximum size weight
		);
	}
	if (name == "s3fifo_roun1024") {
		return make_uniq<S3FifoRounPolicy>(
		size,
		1024 * 1024
		);
	}
	if (name == "s3fifo_roun512") {
		return make_uniq<S3FifoRounPolicy>(
		size,
		512 * 1024
		);
	}
	if (name == "s3fifo_roun1536") {
		return make_uniq<S3FifoRounPolicy>(
		size,
		1536 * 1024
		);
	}
	throw InvalidInputException("Unknown eviction policy '%s'; expected one of slru, lru, fifo, clock, mru", name);
}

} // namespace duckdb
