// S3-FIFO eviction policy.
//
// The policy uses three FIFO queues:
//   S: small probationary queue for newly inserted blocks
//   M: main queue for blocks that have demonstrated reuse
//   G: ghost queue for blocks recently evicted from S
//
// S and M are partitioned by cache capacity:
//   S = 10% of the cache
//   M = 90% of the cache
//
// G stores metadata only and has the same entry capacity as M.
//
// Queue order is FIFO: newly inserted/reinserted entries are placed at the
// front, while candidates are considered from the back.
//
// Cache hits never modify queue order. Instead, they increment a capped
// frequency counter. This avoids the list updates required by LRU.

#pragma once

#include <cstdint>
#include <list>
#include <unordered_map>

#include "policy/eviction_policy.hpp"

namespace duckdb {

class S3FifoPolicy final : public EvictionPolicy {
public:
	explicit S3FifoPolicy(idx_t cache_size)
	    : small_capacity(cache_size / 10), main_capacity(cache_size - small_capacity) {
	}

	void OnHit(const InMemCacheBlock &key) override {
		auto it = index.find(key);
		if (it == index.end()) {
			return;
		}

		// S3-FIFO uses a capped frequency counter with a maximum value of 3.
		if (it->second.frequency < MAX_FREQUENCY) {
			it->second.frequency++;
		}
	}

	void OnInsert(const InMemCacheBlock &key, idx_t size, double cost_us) override {
		(void)cost_us;

		// Ignore duplicate inserts.
		if (index.find(key) != index.end()) {
			return;
		}

		Entry entry;
		entry.size = size;
		entry.frequency = 0;

		auto ghost_it = ghost_index.find(key);
		if (ghost_it != ghost_index.end()) {
			// A block that was previously evicted from S has demonstrated
			// reuse. Remove it from G and admit it directly into M.
			ghost_order.erase(ghost_it->second);
			ghost_index.erase(ghost_it);

			entry.queue = Queue::MAIN;
			main_order.push_front(key);
			entry.position = main_order.begin();
			main_size += size;
		} else {
			// New blocks enter the small probationary queue.
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
		// Keep processing S while it exceeds its capacity. This may promote
		// several reused blocks to M before finding an actual victim.
		if (small_size > small_capacity) {
			if (PopFromSmall(victim)) {
				return true;
			}
		}

		// Keep M within its capacity.
		if (main_size > main_capacity) {
			if (PopFromMain(victim)) {
				return true;
			}
		}

		// If neither partition is over capacity, choose an eviction candidate
		// from S first. This maintains S as the probationary part of the cache.
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
		return "s3fifo";
	}

private:
	static constexpr uint8_t MAX_FREQUENCY = 3;

	enum class Queue : uint8_t {
		SMALL,
		MAIN
	};

	struct Entry {
		Queue queue;
		uint8_t frequency;
		idx_t size;

		// The iterator belongs to either small_order or main_order,
		// depending on queue.
		std::list<InMemCacheBlock>::iterator position;
	};

	bool PopFromSmall(InMemCacheBlock &victim) {
		while (!small_order.empty()) {
			// FIFO: inspect the oldest block.
			const auto key = small_order.back();

			auto it = index.find(key);
			D_ASSERT(it != index.end());

			auto &entry = it->second;

			if (entry.frequency > 0) {
				// The block was accessed while in S. Promote it to M.
				small_size -= entry.size;
				small_order.pop_back();

				main_order.push_front(key);
				entry.queue = Queue::MAIN;
				entry.position = main_order.begin();
				main_size += entry.size;

				continue;
			}

			// A block without reuse is a one-hit wonder. It is evicted and
			// remembered in G.
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
				// Give a frequently accessed block another pass through M.
				entry.frequency--;

				main_order.pop_back();
				main_order.push_front(key);
				entry.position = main_order.begin();

				continue;
			}

			// No accesses since the previous pass through M.
			victim = key;

			main_size -= entry.size;
			main_order.pop_back();
			index.erase(it);

			return true;
		}

		return false;
	}

	void AddToGhost(const InMemCacheBlock &key) {
		// G stores metadata only. Its entry capacity matches the number of
		// blocks currently admitted to M.
		ghost_order.push_front(key);
		ghost_index[key] = ghost_order.begin();

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

	idx_t small_size = 0;
	idx_t main_size = 0;

	// S and M contain cached blocks.
	std::list<InMemCacheBlock> small_order;
	std::list<InMemCacheBlock> main_order;

	std::unordered_map<InMemCacheBlock, Entry, InMemCacheBlockHash, InMemCacheBlockEqual> index;

	// G contains only metadata for blocks previously evicted from S.
	std::list<InMemCacheBlock> ghost_order;
	std::unordered_map<InMemCacheBlock, std::list<InMemCacheBlock>::iterator, InMemCacheBlockHash,
	                   InMemCacheBlockEqual>
	    ghost_index;
};

} // namespace duckdb
