#pragma once

#include <algorithm>
#include <list>
#include <unordered_map>

#include "policy/eviction_policy.hpp"

namespace duckdb {

// Size-Aware Most Recently Used.
//
// Similar to MRU, but blocks receive additional rounds depending on their
// size. When the most recently used block is selected for eviction, it is
// given another round through the list if its counter has not yet reached its
// size-dependent threshold. Smaller blocks therefore receive more chances to
// remain in the cache than larger blocks.
class SizeAwareMruInvPolicy final : public EvictionPolicy {
private:
struct Entry {
InMemCacheBlock block;

	// Number of times this block has reached the eviction position.
	idx_t counter = 0;

	explicit Entry(const InMemCacheBlock &block_p) : block(block_p) {
	}
};

using EntryList = std::list<Entry>;
using EntryIterator = EntryList::iterator;

public:
void OnHit(const InMemCacheBlock &key) override {
auto it = index.find(key);
if (it == index.end()) {
return;
}

	// Most recently used block at the front.
	order.splice(order.begin(), order, it->second);

	// A new access gives the block a fresh set of eviction rounds.
	it->second->counter = 0;
}

void OnInsert(const InMemCacheBlock &key, idx_t size, double cost_us) override {
	auto it = index.find(key);
	if (it != index.end()) {
		order.splice(order.begin(), order, it->second);
		it->second->counter = 0;
		return;
	}

	order.emplace_front(key);
	index[key] = order.begin();
}

void OnErase(const InMemCacheBlock &key) override {
	auto it = index.find(key);
	if (it == index.end()) {
		return;
	}

	order.erase(it->second);
	index.erase(it);
}

// MRU: examine the front, which contains the most recently used block.
bool PopVictim(InMemCacheBlock &victim) override {
	while (!order.empty()) {
		auto entry_it = order.begin();
		auto &entry = *entry_it;

		const auto max_rounds = GetMaxRounds(entry.block.blk_size);

		// Give the block another round if it has not yet used all of its
		// size-dependent eviction rounds.
		if (entry.counter < max_rounds) {
			entry.counter++;

			// Move it away from the MRU position. Since this is MRU, move
			// it to the back so the next most recently used block is
			// considered next.
			order.splice(order.end(), order, entry_it);
			continue;
		}

		// The block has used all of its rounds and is evicted.
		victim = entry.block;
		index.erase(victim);
		order.erase(entry_it);
		return true;
	}

	return false;
}

void Clear() override {
	order.clear();
	index.clear();
}

string GetName() const override {
	return "size_aware_mru_inv";
}

private:
idx_t GetMaxRounds(idx_t size) const {
static constexpr idx_t REFERENCE_BLOCK_SIZE = 1024ULL * 1024ULL;
static constexpr idx_t MAX_ROUNDS = 10;

	if (size == 0) {
		return MAX_ROUNDS;
	}

	return std::min<idx_t>(size / REFERENCE_BLOCK_SIZE, MAX_ROUNDS);
}

private:
EntryList order;

std::unordered_map<InMemCacheBlock, EntryIterator, InMemCacheBlockHash, InMemCacheBlockEqual> index;

};

} // namespace duckdb
