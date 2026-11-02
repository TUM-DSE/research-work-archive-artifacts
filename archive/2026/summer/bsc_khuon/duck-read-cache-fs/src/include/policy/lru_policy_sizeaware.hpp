#pragma once

#include <algorithm>
#include <list>
#include <unordered_map>

#include "policy/eviction_policy.hpp"

namespace duckdb {

class SizeAwareLruPolicy final : public EvictionPolicy {
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
explicit SizeAwareLruPolicy(idx_t reference_size = 1024 * 1024)
	: reference_block_size(reference_size) {
}


void OnHit(const InMemCacheBlock &key) override {
auto it = index.find(key);
if (it == index.end()) {
return;
}

	// Most recently used blocks are kept at the front.
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

bool PopVictim(InMemCacheBlock &victim) override {
	while (!order.empty()) {
		auto entry_it = std::prev(order.end());
		auto &entry = *entry_it;

		const auto max_rounds = GetMaxRounds(entry.block.blk_size);

		// Give the block another round through the LRU list.
		if (entry.counter < max_rounds) {
			entry.counter++;
			order.splice(order.begin(), order, entry_it);
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
	return "size_aware_lru";
}

private:
idx_t GetMaxRounds(idx_t size) const {
static constexpr idx_t MAX_ROUNDS = 10;

	if (size == 0) {
		return MAX_ROUNDS;
	}

	return std::min<idx_t>(reference_block_size/ size, MAX_ROUNDS);
}

private:
EntryList order;
idx_t reference_block_size;

std::unordered_map<InMemCacheBlock, EntryIterator, InMemCacheBlockHash, InMemCacheBlockEqual> index;

};

} // namespace duckdb
