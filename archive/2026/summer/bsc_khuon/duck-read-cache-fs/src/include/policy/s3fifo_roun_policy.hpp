#pragma once

#include <list>
#include <unordered_map>

#include "policy/eviction_policy.hpp"

namespace duckdb {

class S3FifoRounPolicy final : public EvictionPolicy {
public:
    explicit S3FifoRounPolicy(idx_t cache_size,
                              idx_t reference_size,
                              uint8_t max_frequency = 6,
                              uint8_t promotion_threshold = 1,
                              uint8_t max_eviction_cost = 3)
        : small_capacity(cache_size / 10),
          main_capacity(cache_size - small_capacity),
          reference_size(reference_size),
          max_frequency(max_frequency),
          promotion_threshold(promotion_threshold),
          max_eviction_cost(max_eviction_cost) {

        ghost_capacity = main_capacity / reference_size;

        if (ghost_capacity == 0) {
            ghost_capacity = 1;
        }
    }

    void OnHit(const InMemCacheBlock &key) override {
        auto small_it = small_index.find(key);
        if (small_it != small_index.end()) {
            if (small_it->second->frequency < max_frequency) {
                small_it->second->frequency++;
            }
            return;
        }

        auto main_it = main_index.find(key);
        if (main_it != main_index.end()) {
            if (main_it->second->frequency < max_frequency) {
                main_it->second->frequency++;
            }
            return;
        }
    }

    void OnInsert(const InMemCacheBlock &key, idx_t size, double cost_us) override {
        // Already cached.
        if (small_index.find(key) != small_index.end() ||
            main_index.find(key) != main_index.end()) {
            return;
        }

        // Ghost hit: admit directly to the main queue.
        auto ghost_it = ghost_index.find(key);
        if (ghost_it != ghost_index.end()) {
            ghost_order.erase(ghost_it->second);
            ghost_index.erase(ghost_it);

            MainEntry entry{key, size, 0};

            main_order.push_front(entry);
            main_index[key] = main_order.begin();
            return;
        }

        // Normal S3-FIFO admission into the small queue.
        SmallEntry entry{key, size, 0};

        small_order.push_front(entry);
        small_index[key] = small_order.begin();
    }

    void OnErase(const InMemCacheBlock &key) override {
        auto small_it = small_index.find(key);
        if (small_it != small_index.end()) {
            small_order.erase(small_it->second);
            small_index.erase(small_it);
            return;
        }

        auto main_it = main_index.find(key);
        if (main_it != main_index.end()) {
            main_order.erase(main_it->second);
            main_index.erase(main_it);
            return;
        }

        auto ghost_it = ghost_index.find(key);
        if (ghost_it != ghost_index.end()) {
            ghost_order.erase(ghost_it->second);
            ghost_index.erase(ghost_it);
        }
    }

    bool PopVictim(InMemCacheBlock &victim) override {
        // First process the small queue.
        while (!small_order.empty()) {
            auto it = std::prev(small_order.end());
            auto &entry = *it;

            // Normal, size-independent promotion.
            if (entry.frequency >= promotion_threshold) {
                MainEntry main_entry{
                    entry.key,
                    entry.size,
                    entry.frequency
                };

                small_index.erase(entry.key);
                small_order.erase(it);

                main_order.push_front(main_entry);
                main_index[main_entry.key] = main_order.begin();

                continue;
            }

            // Evict from small queue and remember in ghost queue.
            victim = entry.key;

            AddToGhost(entry.key);

            small_index.erase(entry.key);
            small_order.erase(it);

            return true;
        }

        // Then process the main queue.
        while (!main_order.empty()) {
            auto it = std::prev(main_order.end());
            auto &entry = *it;

            if (entry.frequency > 0) {
                const uint8_t eviction_cost =
                    GetEvictionCost(entry.size);

                // Consume frequency credit according to block size.
                if (entry.frequency > eviction_cost) {
                    entry.frequency -= eviction_cost;

                    // Reinsert at the front to give it another chance.
                    MainEntry moved{
                        entry.key,
                        entry.size,
                        entry.frequency
                    };

                    main_index.erase(entry.key);
                    main_order.erase(it);

                    main_order.push_front(moved);
                    main_index[moved.key] = main_order.begin();

                    continue;
                }
            }

            // No frequency credit remains: evict.
            victim = entry.key;

            main_index.erase(entry.key);
            main_order.erase(it);

            return true;
        }

        return false;
    }

    void Clear() override {
        small_order.clear();
        small_index.clear();

        main_order.clear();
        main_index.clear();

        ghost_order.clear();
        ghost_index.clear();
    }

    string GetName() const override {
        return "s3_fifo_size_main";
    }

private:
    struct SmallEntry {
        InMemCacheBlock key;
        idx_t size;
        uint8_t frequency;
    };

    struct MainEntry {
        InMemCacheBlock key;
        idx_t size;
        uint8_t frequency;
    };

    void AddToGhost(const InMemCacheBlock &key) {
        // Avoid duplicate ghost entries.
        auto existing = ghost_index.find(key);
        if (existing != ghost_index.end()) {
            ghost_order.erase(existing->second);
            ghost_index.erase(existing);
        }

        ghost_order.push_front(key);
        ghost_index[key] = ghost_order.begin();

        while (ghost_order.size() > ghost_capacity) {
            auto it = std::prev(ghost_order.end());

            ghost_index.erase(*it);
            ghost_order.erase(it);
        }
    }

    uint8_t GetEvictionCost(idx_t size) const {
        if (reference_size == 0) {
            return 1;
        }

        // ceil(size / reference_size)
        idx_t weight =
            (size + reference_size - 1) / reference_size;

        if (weight < 1) {
            weight = 1;
        }

        if (weight > max_eviction_cost) {
            weight = max_eviction_cost;
        }

        return static_cast<uint8_t>(weight);
    }

private:
    idx_t small_capacity;
    idx_t main_capacity;
    idx_t ghost_capacity;

    idx_t reference_size;

    uint8_t max_frequency;
    uint8_t promotion_threshold;
    uint8_t max_eviction_cost;

    std::list<SmallEntry> small_order;
    std::unordered_map<
        InMemCacheBlock,
        std::list<SmallEntry>::iterator,
        InMemCacheBlockHash,
        InMemCacheBlockEqual>
        small_index;

    std::list<MainEntry> main_order;
    std::unordered_map<
        InMemCacheBlock,
        std::list<MainEntry>::iterator,
        InMemCacheBlockHash,
        InMemCacheBlockEqual>
        main_index;

    std::list<InMemCacheBlock> ghost_order;
    std::unordered_map<
        InMemCacheBlock,
        std::list<InMemCacheBlock>::iterator,
        InMemCacheBlockHash,
        InMemCacheBlockEqual>
        ghost_index;
};

} // namespace duckdb
