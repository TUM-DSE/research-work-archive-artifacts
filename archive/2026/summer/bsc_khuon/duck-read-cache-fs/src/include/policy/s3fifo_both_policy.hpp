// Size-aware S3-FIFO eviction policy.
//
// This policy extends S3-FIFO with two size-aware mechanisms:
//
// 1. Size-aware promotion from S to M:
//    Larger blocks require more hits before they are promoted from the
//    probationary queue S to the main queue M.
//
// 2. Size-aware second chances in M:
//    Larger blocks consume more frequency credit when they reach the FIFO
//    eviction position. Therefore, they need proportionally more accesses to
//    receive the same amount of protection as smaller blocks.
//
// Both mechanisms use:
//
//   weight(size) = clamp(ceil(size / reference_size), 1, max_size_weight)
//
// The cache is divided into:
//   S = 10% of cache capacity
//   M = 90% of cache capacity
//
// G is a ghost FIFO queue containing metadata for blocks evicted from S.

#pragma once

#include <cstdint>
#include <list>
#include <unordered_map>

#include "policy/eviction_policy.hpp"

namespace duckdb {

class S3FifoBothPolicy final : public EvictionPolicy {
public:
	explicit S3FifoBothPolicy(idx_t cache_size, idx_t reference_size, uint8_t max_frequency = 3,
	                      uint8_t max_size_weight = 3)
	    : small_capacity(cache_size / 10), main_capacity(cache_size - small_capacity),
	      reference_size(reference_size), max_frequency(max_frequency), max_size_weight(max_size_weight) {
		D_ASSERT(reference_size > 0);
		D_ASSERT(max_frequency > 0);
		D_ASSERT(max_size_weight > 0);
	}

	void OnHit(const InMemCacheBlock &key) override {
		auto it = index.find(key);
		if (it == index.end()) {
			return;
		}

		// Hits add one unit of reuse credit. The counter is capped.
		if (it->second.frequency < max_frequency) {
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
			// A block found in G has demonstrated reuse after previously being
			// evicted from S. Admit it directly to M.
			ghost_order.erase(ghost_it->second);
			ghost_index.erase(ghost_it);

			entry.queue = Queue::MAIN;
			main_order.push_front(key);
			entry.position = main_order.begin();
			main_size += size;
		} else {
			// New blocks enter the probationary queue.
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
		// First enforce the capacity of S.
		if (small_size > small_capacity && PopFromSmall(victim)) {
			return true;
		}

		// Then enforce the capacity of M.
		if (main_size > main_capacity && PopFromMain(victim)) {
			return true;
		}

		// If neither partition is currently over capacity, prefer S because it
		// contains probationary blocks that have not yet demonstrated enough
		// reuse to justify occupying M.
		if (!small_order.empty() && PopFromSmall(victim)) {
			return true;
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
		return "s3fifo_both";
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

	uint8_t GetSizeWeight(idx_t size) const {
		// Ceiling division:
		//
		//   ceil(size / reference_size)
		//
		// The result is capped so that very large blocks do not require an
		// unbounded number of accesses.
		const idx_t weight = (size + reference_size - 1) / reference_size;
		const idx_t capped_weight =
		    MinValue<idx_t>(MaxValue<idx_t>(1, weight), static_cast<idx_t>(max_size_weight));

		return static_cast<uint8_t>(capped_weight);
	}

	uint8_t GetPromotionThreshold(idx_t size) const {
		// Mechanism 1:
		// Larger blocks require more reuse before promotion from S to M.
		return GetSizeWeight(size);
	}

	uint8_t GetMainEvictionCost(idx_t size) const {
		// Mechanism 2:
		// Larger blocks consume more reuse credit when they reach the tail of M.
		return GetSizeWeight(size);
	}

	bool PopFromSmall(InMemCacheBlock &victim) {
		while (!small_order.empty()) {
			// FIFO: the oldest block is at the back.
			const auto key = small_order.back();

			auto it = index.find(key);
			D_ASSERT(it != index.end());

			auto &entry = it->second;

			if (entry.frequency >= GetPromotionThreshold(entry.size)) {
				// Mechanism 1:
				// The block has accumulated enough accesses relative to its size
				// and is promoted to M.
				small_size -= entry.size;
				small_order.pop_back();

				main_order.push_front(key);
				entry.queue = Queue::MAIN;
				entry.position = main_order.begin();
				main_size += entry.size;

				continue;
			}

			// Insufficient reuse for the amount of cache space consumed.
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
				// Mechanism 2:
				// A block's accumulated reuse credit is consumed according to
				// its size. Larger blocks therefore lose protection faster.
				const auto eviction_cost = GetMainEvictionCost(entry.size);

				if (entry.frequency > eviction_cost) {
					entry.frequency -= eviction_cost;

					main_order.pop_back();
					main_order.push_front(key);
					entry.position = main_order.begin();

					continue;
				}
			}

			// Either the block had no remaining credit, or its credit was not
			// sufficient to pay for another pass through M.
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

		// G stores metadata only. Bound it by the number of entries currently
		// stored in M, following the original S3-FIFO design.
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

	// Normalization size for the size-aware mechanisms.
	const idx_t reference_size;

	// Maximum number of reuse credits accumulated by a block.
	const uint8_t max_frequency;

	// Maximum size-based promotion threshold and eviction cost.
	const uint8_t max_size_weight;

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
