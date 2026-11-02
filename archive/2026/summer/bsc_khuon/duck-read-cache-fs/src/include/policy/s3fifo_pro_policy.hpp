// Size-aware S3-FIFO eviction policy.
//
// S3-FIFO uses three FIFO queues:
//   S: small probationary queue for newly inserted blocks
//   M: main queue for blocks that have demonstrated sufficient reuse
//   G: ghost queue for blocks recently evicted from S
//
// This variant makes promotion from S to M size aware. A block must receive
// more hits in S when it is larger:
//
//   promotion_threshold(size) =
//       clamp(ceil(size / reference_size), 1, max_promotion_threshold)
//
// Thus, larger blocks need stronger evidence of reuse before they are allowed
// to occupy space in M.

#pragma once

#include <cstdint>
#include <list>
#include <unordered_map>

#include "policy/eviction_policy.hpp"

namespace duckdb {

class S3FifoProPolicy final : public EvictionPolicy {
public:
	explicit S3FifoProPolicy(idx_t cache_size, idx_t reference_size,
	                      uint8_t max_promotion_threshold = 3)
	    : small_capacity(cache_size / 10), main_capacity(cache_size - small_capacity),
	      reference_size(reference_size), max_promotion_threshold(max_promotion_threshold) {
		D_ASSERT(reference_size > 0);
		D_ASSERT(max_promotion_threshold > 0);
	}

	void OnHit(const InMemCacheBlock &key) override {
		auto it = index.find(key);
		if (it == index.end()) {
			return;
		}

		// Count accesses while the block is in the cache. The counter is capped
		// because values above the maximum possible promotion threshold provide
		// no additional benefit.
		if (it->second.frequency < max_promotion_threshold) {
			it->second.frequency++;
		}
	}

	void OnInsert(const InMemCacheBlock &key, idx_t size, double cost_us) override {
		(void)cost_us;

		if (index.find(key) != index.end()) {
			return;
		}

		Entry entry;
		entry.size = size;
		entry.frequency = 0;

		auto ghost_it = ghost_index.find(key);
		if (ghost_it != ghost_index.end()) {
			// A ghost hit means that this block was previously evicted from S
			// and has now been requested again. It has therefore demonstrated
			// reuse across cache residency periods and is admitted directly to M.
			ghost_order.erase(ghost_it->second);
			ghost_index.erase(ghost_it);

			entry.queue = Queue::MAIN;
			main_order.push_front(key);
			entry.position = main_order.begin();
			main_size += size;
		} else {
			// New blocks always enter S first.
			entry.queue = Queue::SMALL;
			small_order.push_front(key);
			entry.position = small_order.begin();
			small_size += size;
		}

		index.emplace(key, entry);
	}

	void OnErase(const InMemCacheBlock &key) override {
		auto it = index.find(key);
		if (it == index.end()) {
			return;
		}

		const auto &entry = it->second;

		if (entry.queue == Queue::SMALL) {
			small_size -= entry.size;
			small_order.erase(entry.position);
		} else {
			main_size -= entry.size;
			main_order.erase(entry.position);
		}

		index.erase(it);
	}

	bool PopVictim(InMemCacheBlock &victim) override {
		// S is the probationary part of the cache. Prefer processing it when it
		// exceeds its assigned capacity.
		if (small_size > small_capacity) {
			if (PopFromSmall(victim)) {
				return true;
			}
		}

		// Keep M within its assigned capacity.
		if (main_size > main_capacity) {
			if (PopFromMain(victim)) {
				return true;
			}
		}

		// If neither partition exceeds its capacity, S3-FIFO first considers
		// the probationary queue.
		if (!small_order.empty()) {
			return PopFromSmall(victim);
		}

		if (!main_order.empty()) {
			return PopFromMain(victim);
		}

		return false;
	}

	void Clear() override {
		small_order.clear();
		main_order.clear();
		ghost_order.clear();

		index.clear();
		ghost_index.clear();

		small_size = 0;
		main_size = 0;
	}

	string GetName() const override {
		return "s3fifo_pro";
	}

private:
	enum class Queue : uint8_t {
		SMALL,
		MAIN
	};

	struct Entry {
		Queue queue;
		uint8_t frequency;
		idx_t size;
		std::list<InMemCacheBlock>::iterator position;
	};

	uint8_t GetPromotionThreshold(idx_t size) const {
		// Ceiling division:
		//
		//   ceil(size / reference_size)
		//
		// At least one hit is always required for promotion, and the threshold
		// is capped to keep the required evidence bounded.
		const idx_t weight = (size + reference_size - 1) / reference_size;
		const idx_t threshold = MinValue<idx_t>(
		    MaxValue<idx_t>(1, weight), static_cast<idx_t>(max_promotion_threshold));

		return static_cast<uint8_t>(threshold);
	}

	bool PopFromSmall(InMemCacheBlock &victim) {
		while (!small_order.empty()) {
			// FIFO: inspect the oldest block.
			const auto key = small_order.back();

			auto it = index.find(key);
			D_ASSERT(it != index.end());

			auto &entry = it->second;

			const auto promotion_threshold = GetPromotionThreshold(entry.size);

			if (entry.frequency >= promotion_threshold) {
				// The block has accumulated enough accesses relative to its size
				// and is therefore promoted to M.
				small_size -= entry.size;
				small_order.pop_back();

				main_order.push_front(key);
				entry.queue = Queue::MAIN;
				entry.position = main_order.begin();
				main_size += entry.size;

				continue;
			}

			// The block did not demonstrate enough reuse for its size. Evict it
			// and remember it in G.
			victim = key;

			small_size -= entry.size;
			small_order.pop_back();
			index.erase(it);

			AddToGhost(key);
			return true;
		}

		return false;
	}

	bool PopFromMain(InMemCacheBlock &victim) {
		while (!main_order.empty()) {
			// FIFO: inspect the oldest block.
			const auto key = main_order.back();

			auto it = index.find(key);
			D_ASSERT(it != index.end());

			auto &entry = it->second;

			if (entry.frequency > 0) {
				// Standard S3-FIFO second-chance behaviour in M: decrement the
				// frequency and give the block another pass through the queue.
				entry.frequency--;

				main_order.pop_back();
				main_order.push_front(key);
				entry.position = main_order.begin();

				continue;
			}

			victim = key;

			main_size -= entry.size;
			main_order.pop_back();
			index.erase(it);

			return true;
		}

		return false;
	}

	void AddToGhost(const InMemCacheBlock &key) {
		ghost_order.push_front(key);
		ghost_index[key] = ghost_order.begin();

		// The original policy bounds G relative to M. Since G stores only
		// metadata, this implementation uses the current number of entries in M.
		const idx_t ghost_capacity = main_order.size();

		while (ghost_order.size() > ghost_capacity) {
			const auto oldest = ghost_order.back();
			ghost_index.erase(oldest);
			ghost_order.pop_back();
		}
	}

private:
	const idx_t small_capacity;
	const idx_t main_capacity;

	// Size used to normalize block sizes when calculating promotion thresholds.
	const idx_t reference_size;

	// Maximum number of hits that can be required for promotion.
	const uint8_t max_promotion_threshold;

	idx_t small_size = 0;
	idx_t main_size = 0;

	std::list<InMemCacheBlock> small_order;
	std::list<InMemCacheBlock> main_order;

	std::unordered_map<InMemCacheBlock, Entry, InMemCacheBlockHash, InMemCacheBlockEqual> index;

	std::list<InMemCacheBlock> ghost_order;
	std::unordered_map<InMemCacheBlock, std::list<InMemCacheBlock>::iterator, InMemCacheBlockHash,
	                   InMemCacheBlockEqual>
	    ghost_index;
};

} // namespace duckdb
