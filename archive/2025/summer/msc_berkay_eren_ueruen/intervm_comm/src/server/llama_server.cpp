#include <argp.h>
#include <assert.h>
#include <fcntl.h>
#include <string.h>
#include <sys/mman.h>
#include <unistd.h>

#include <algorithm>
#include <array>
#include <atomic>
#include <chrono>
#include <cstddef>
#include <cstdio>
#include <fstream>
#include <iostream>
#include <list>
#include <thread>
#include <unordered_map>
#include <vector>

#include "args_server.h"
#include "client_request.h"
#include "common.h"
#include "defines.h"
#include "llama.h"
#include "llama_bench_args.h"
#include "misc.h"
#include "shm.h"

#ifdef DEBUG_FLAG
#define LOG_MSG(...) printf(__VA_ARGS__) // Or simply LOG_MSG(msg) printf(msg)
#define LOG_FLUSH(...) printf(__VA_ARGS__)
#else
#define LOG_MSG(...)                     // Or LOG_MSG(msg)
#define LOG_FLUSH(...)
#endif

int run = 1;
unsigned int n_threads = 1;

struct Communication comm;

llama_model *model;
std::unordered_map<int, llama_lora_adapter *> lora_adapters;

int last_processed_request_id = -1;
int last_processed_request_prio = -1;
bool context_change_required;

int curr_priority;
std::atomic<int> highest_avail_prio(
    NO_PRIORITY);  // will receinve a proper priority on fist scheduled request

enum Policy general_policy;
bool auto_policy = false;

struct ggml_threadpool *threadpool;
struct ggml_threadpool *threadpool_batch;

bool use_llama_bench_args = false;
cmd_params_instance llama_bench_args;
unsigned int server_total_nb_tokens_generated;
std::chrono::milliseconds total_context_switch_duration;
unsigned int nb_context_switches = 0;
std::chrono::milliseconds total_context_creation_duration;
unsigned int nb_context_creations = 0;

// Holds number of requests by their id (for debugging)
std::unordered_map<int, int> client_request_counter;

int round_robin_next[NB_PRIORITIES] = {0};

struct request_entry {
    int id;
    ClientRequest request;
};

std::array<std::list<request_entry>, NB_PRIORITIES> open_reqs;
std::array<std::mutex, NB_PRIORITIES> open_reqs_mutex;

int prepare_llama_cpp(struct Arguments arguments) {
    // initialize the model
    llama_model_params model_params = llama_model_default_params();
    model_params.n_gpu_layers = 999;

    if (!use_llama_bench_args) {
        model = llama_load_model_from_file(arguments.model_path.c_str(), model_params);
    } else {
        std::cout << "Using llama-bench arguments, however we ignore model specification\n"
                  << std::endl;
        model = llama_load_model_from_file(arguments.model_path.c_str(),
                                           llama_bench_args.to_llama_mparams());
    }

    if (model == NULL) {
        fprintf(stderr, "%s: error: unable to load model\n", __func__);
        return 1;
    }

    // Load lora adapters, they are loaded but not applied.
    int index = 0;
    for (const auto &lora_path : arguments.lora_paths) {
        struct llama_lora_adapter *lora = llama_lora_adapter_init(model, lora_path.c_str());
        lora_adapters.insert({index++, lora});
    }

    return 0;
}

int clean_llama_cpp(void) {
    llama_free_model(model);
    return 0;
}

int request_processor(ClientRequest &request, int tokens_to_gen, float throughput_limit) {
    throughput_limit = MIN(throughput_limit, request.get_throughput());
    // init context if it has not already
    if (request.require_context_switch && request.initialized) {
        LOG_MSG(stdout, "Loading context to serve a different request...");
        LOG_FLUSH(stdout);

        auto t1 = std::chrono::high_resolution_clock::now();
        switch (request.policy) {
            case Policy::save_load_full:
                request.init_ctx_from_save_state();
                break;
            case Policy::save_load_kv:
                request.init_from_saved_kv();
                break;
            case Policy::recompute:
                request.init_from_clean_kv();
                break;
            default:
                break;
        };

        auto t2 = std::chrono::high_resolution_clock::now();
        auto duration = duration_cast<std::chrono::milliseconds>(t2 - t1);
        LOG_MSG(stdout, "%s: Context switch in %ld ms\n", __func__, duration.count());

        total_context_switch_duration += duration;
        nb_context_switches++;
    } else if (!request.initialized) {
        LOG_MSG(stdout, "Creating new context to serve a different request...");
        LOG_FLUSH(stdout);

        auto t1 = std::chrono::high_resolution_clock::now();
        request.init_llama_context();
        request.initialized = true;

        auto t2 = std::chrono::high_resolution_clock::now();

        auto duration = duration_cast<std::chrono::milliseconds>(t2 - t1);
        LOG_MSG(stdout, "%s: Context creation in %ld ms\n", __func__, duration.count());
        LOG_FLUSH(stdout);

        total_context_creation_duration += duration;
        nb_context_creations++;
    }

    // Apply requests lora, does nothing if no lora specified
    request.apply_lora();

    llama_kv_cache_view_update(request.ctx, &request.kv_view);
    LOG_MSG(stdout, "KV Cache right after the switch:\n\t");
#ifdef DEBUG_FLAG
    request.print_kv_cache();
#endif
    std::cout << std::endl;
    request.require_context_switch = false;

    LOG_MSG(stdout, "[TEO_DEBUG]: Llama context initialized\n");

    const auto t_main_start = ggml_time_us();
    int n_decode = 0;
    int n_generated_tokens = 0;

    auto t_per_token_start = ggml_time_us();
    auto t_per_token_end = t_per_token_start;

    llama_attach_threadpool(request.ctx, threadpool, threadpool_batch);
    llama_set_n_threads(request.ctx, n_threads, n_threads);

    while (n_generated_tokens < tokens_to_gen && run) {
        // evaluate the current batch with the transformer model
        int result_decode = llama_decode(request.ctx, request.batch);
        if (result_decode != 0) {
            if (result_decode == 1) {
                std::cout << "KV cache full! Stopping request..." << std::endl;
                request.done = true;
                request.initialized = false;
                return 0;
            }
            fprintf(stderr, "%s : failed to eval, return code %d\n", __func__, 1);
            exit(1);
        }

        request.n_past_tokens += request.batch.n_tokens;
        request.n_past_tokens_mod += request.batch.n_tokens;
        n_decode += request.batch.n_tokens;

        server_total_nb_tokens_generated++;  // For metrics

        // sample the next token
        {
            request.new_token_id = llama_sampler_sample(request.smpl, request.ctx, -1);
            Request *raw_request = request.get_raw_request();

            // is it an end of generation?
            if (llama_token_is_eog(model, request.new_token_id)) {
                request.done = true;
                request.initialized = false;
                std::cout << "[Info] End of generation charachter created, we generated less chars "
                             "than the "
                             "client requested\n"
                          << "Request has been fully processed!" << std::endl;
                break;
            }

            char buf[128];
            int n = llama_token_to_piece(model, request.new_token_id, buf, sizeof(buf), 0, true);
            if (n < 0) {
                fprintf(stderr, "%s: error: failed to convert token to piece\n", __func__);
                return 1;
            }

            // Write to shm
            int cur_length = request.get_current_length();
            for (int a = 0; a < n && request.n_generated_char + a <= request.n_chars_to_gen; a++) {
                raw_request->text[cur_length + a] = buf[a];
            }

            std::string s(buf, n);
            LOG_MSG(stdout, "Generated in this slot:\n  %s", s.c_str());
            LOG_FLUSH(stdout);

            request.n_generated_char += n;

            if (request.n_generated_char >= request.n_chars_to_gen) {
                request.done = true;
                request.initialized = false;
                std::cout << "Request has been fully processed!" << std::endl;
            }

            // TODO: Do these make sense after batch is done or should we move those up?
            // prepare the next batch with the sampled token
            common_batch_clear(request.batch);
            common_batch_add(request.batch, request.new_token_id, request.n_past_tokens, {0}, true);

            // Save generated tokens so that we can recover after a KV cache flush
            request.prompt_tokens.push_back(request.new_token_id);

#ifdef DEBUG_FLAG
            // Update kv_cache view (debug)
            llama_kv_cache_view_update(request.ctx, &request.kv_view);
            request.print_kv_cache();
#endif
        }

        t_per_token_end = ggml_time_us();
        float token_throughput = 1 / ((t_per_token_end - t_per_token_start) / 1000000.0f);
        LOG_MSG(stdout, "Token throughput: %f\n", token_throughput);
        if (token_throughput > throughput_limit) {
            auto t_main_end_new = (1 * 1000000.0 / throughput_limit) + t_per_token_start;
            int sleep_time_us = t_main_end_new - ggml_time_us();
            LOG_MSG(stdout, "Sleeping for %d us\n", sleep_time_us);
            usleep(sleep_time_us);
        }
        t_per_token_end = ggml_time_us();
        token_throughput = 1 / ((t_per_token_end - t_per_token_start) / 1000000.0f);
        LOG_MSG(stdout, "New Token throughput: %f\n", token_throughput);
        t_per_token_start = t_per_token_end;

        n_generated_tokens++;
        if (request.n_generated_char >= request.n_chars_to_gen) {
            break;
        }

        // lower numbers are higher priority
        if (highest_avail_prio.load() < curr_priority) {
            printf("Higher priority request detected! Preempting current one!");
            break;
        }
    }

    LOG_MSG(stdout, "\n");
    LOG_MSG(stdout, "n_decode: %d\n", n_decode);
    auto t_main_end = ggml_time_us();
    float throughput = n_decode / ((t_main_end - t_main_start) / 1000000.0f);

    LOG_MSG(stdout, "Throughput: %f\n", throughput);
    LOG_MSG(stderr, "%s: decoded %d tokens in %.2f s, speed: %.2f t/s\n", __func__, n_decode,
            (t_main_end - t_main_start) / 1000000.0f, throughput);

    // Temp stop point for benchmarks
    if (request.limit_hit == false && request.n_past_tokens_mod > KV_CACHE_LIMIT) {
        request.limit_hit = true;
        request.n_past_tokens_mod = 0;
        std::cout << "KV_CACHE_LIMIT hit, sleeping for 5 seconds" << std::endl;
        // sleep(5);
    }

    if (auto_policy) {
        request.auto_set_kv_policy();
    }

#ifdef DEBUG_FLAG
    llama_perf_sampler_print(request.smpl);
    llama_perf_context_print(request.ctx);
    fprintf(stderr, "\n");
#endif
    return 0;
}

int handle_request(ClientRequest &request, int tokens_to_gen, float throughput_limit) {
    request_processor(request, tokens_to_gen, throughput_limit);
    LOG_MSG(stdout, "[TEO_DEBUG]: Finished processing request\n");
    return 0;
}

void scan_control(void) {
    while (1) {
        // Get the current semaphore value
        if (sem_getvalue(&comm.requestQueue->run, &run) == -1) {
            perror("sem_getvalue failed");
            exit(EXIT_FAILURE);
        }

        if (run == 0) {
            break;
        }
    }
}

void scan_requests(void) {
    // This is not ideal and should be 'sem_wait' instead.
    // However, sem_wait does not seem to work when the semaphore is
    // inside of a shared memory, used by two QEMU VMs.
    while (sem_trywait(&comm.requestQueue->active_reqs) != 0) {
        if (run == 0) {
            return;
        }
    }

    for (int i = 0; i < MAX_REQUESTS; i++) {
        if (sem_trywait(&comm.requestQueue->requests[i].serverNotifier) == 0) {
            ClientRequest request{&comm.requestQueue->requests[i],
                                  std::string{comm.requestQueue->requests[i].text},
                                  comm.requestQueue->requests[i].n_chars_to_gen,
                                  comm.requestQueue->requests[i].id,
                                  comm.requestQueue->requests[i].prio,
                                  comm.requestQueue->requests[i].lora};
            open_reqs_mutex[request.prio].lock();
            open_reqs[request.prio].emplace_back(request.id, request);

            // lower numbers have higher priority
            if (request.prio < highest_avail_prio.load())
                highest_avail_prio.store(request.prio);

            open_reqs_mutex[request.prio].unlock();

            // Update the number of request we received from this specific
            // client
            if (client_request_counter.count(request.id)) {
                client_request_counter[request.id]++;
            } else {
                client_request_counter[request.id] = 1;
            }
        }
    }
}

void remove_completed_reqs(void) {
    for (int i = 0; i < NB_PRIORITIES; i++) {
        int j = 0;
        open_reqs_mutex[i].lock();
        for (auto req_iterator = open_reqs[i].begin(); req_iterator != open_reqs[i].end();) {
            if (req_iterator->request.done) {
                auto request = req_iterator->request;
                // free the resources
                llama_synchronize(request.ctx);
                llama_kv_cache_clear(request.ctx);
                llama_kv_cache_view_update(request.ctx, &request.kv_view);
                llama_batch_free(request.batch);
                llama_sampler_free(request.smpl);
                llama_free(request.ctx);
                if (request.state_save) {
                    free(request.state_save);
                    request.state_save = NULL;
                }
                sem_post(&request.get_raw_request()->clientNotifier);

                // remove the element and keep round robin index consistent
                if (j < round_robin_next[i])
                    round_robin_next[i]--;
                req_iterator = open_reqs[i].erase(req_iterator);
            } else {
                ++req_iterator;
            }
        }
        open_reqs_mutex[i].unlock();
    }
}

// Assumes there are requests in the queue
ClientRequest &schedule_next(void) {
    // assert(reqs_available() == true);
    size_t i = 0;
    for (; i < open_reqs.size(); i++) {
        open_reqs_mutex[i].lock();
        if (!open_reqs[i].empty()) {
            open_reqs_mutex[i].unlock();
            break;
        }
        open_reqs_mutex[i].unlock();
    }

    if (i >= open_reqs.size()) {
        throw std::out_of_range("No element could be found");
    }

    open_reqs_mutex[i].lock();

    auto req_it = open_reqs[i].begin();
    std::advance(req_it, round_robin_next[i]);
    ClientRequest &req = req_it->request;

    if (last_processed_request_id != -1 && last_processed_request_id != req.id) {
        req.require_context_switch = true;
        // find the last request
        auto it =
            std::find_if(open_reqs[last_processed_request_prio].begin(),
                         open_reqs[last_processed_request_prio].end(),
                         [](const request_entry &r) { return r.id == last_processed_request_id; });
        // if found, suspend this request. It will get cleaned by the scanner thread.
        if (it != open_reqs[last_processed_request_prio].end()) {
            std::cout << "Freeing context to serve a different request..." << std::endl;
            auto t1 = std::chrono::high_resolution_clock::now();
            switch (it->request.policy) {
                case Policy::save_load_full: {
                    // Save the state
                    std::cout << "\tSaving context before free..." << std::endl;

                    const size_t ctx_size = llama_get_state_size(it->request.ctx);
                    it->request.state_save = (uint8_t *)malloc(ctx_size);
                    llama_state_get_data(it->request.ctx, it->request.state_save, ctx_size);
                    break;
                }
                case Policy::save_load_kv: {
                    // Save the kv_cache
                    std::cout << "\tSaving kv cache before free..." << std::endl;

                    it->request.kv_cache_size = llama_state_seq_get_size(it->request.ctx, 0);
                    it->request.kv_cache_backup = (uint8_t *)malloc(it->request.kv_cache_size);
                    llama_state_seq_get_data(it->request.ctx, it->request.kv_cache_backup,
                                             it->request.kv_cache_size, 0);
                    break;
                }
                default:
                    break;
            };

            llama_kv_cache_clear(it->request.ctx);
            llama_kv_cache_view_update(it->request.ctx, &it->request.kv_view);
            // save load state does not cover the batch
            llama_batch_free(it->request.batch);
            llama_sampler_free(it->request.smpl);
            llama_free(it->request.ctx);

            auto t2 = std::chrono::high_resolution_clock::now();

            auto duration = duration_cast<std::chrono::milliseconds>(t2 - t1);
            LOG_MSG(stdout, "%s: Context save in %ld ms\n", __func__, duration.count());
        }
    }

    last_processed_request_id = req.id;
    last_processed_request_prio = req.prio;

    round_robin_next[i]++;

    req_it = open_reqs[i].begin();
    std::advance(req_it, round_robin_next[i]);
    if (req_it == open_reqs[i].end()) {
        round_robin_next[i] = 0;
    }

    curr_priority = req.prio;
    // see if I want to lock more
    // to avoid overriding a higher priority value that came in the meanwhile
    highest_avail_prio.store(req.prio);

    open_reqs_mutex[i].unlock();

    if (!req.done)
        return req;
    else
        throw std::out_of_range("No element could be found");
}

bool reqs_available(void) {
    for (int i = 0; i < NB_PRIORITIES; i++) {
        open_reqs_mutex[i].lock();
        if (!open_reqs[i].empty()) {
            LOG_MSG(stdout, "[TEO_DEBUG] There is a new request!\n");
            open_reqs_mutex[i].unlock();
            return true;
        }
        open_reqs_mutex[i].unlock();
    }
    return false;
}

void update_req_list() {
    while (run) {
        scan_requests();
    }
}

int main(int argc, char **argv) {
    // init_shm(true, comm);

    Arguments arguments;
    arguments.state_save = false;
    arguments.throughput_limit = 1000;
    arguments.tokens_to_gen = 10;

    // Parse arguments
    argp_parse(&argp, argc, argv, 0, 0, &arguments);

    auto_policy = arguments.auto_policy;
    n_threads = arguments.n_threads;

    if (!arguments.sig_file.empty()) {
        std::ofstream signal_file(arguments.sig_file.c_str());
        signal_file << "signal" << std::endl;
        signal_file.close();
    }

    // Parse llama-bench style arguments if exists
    if (arguments.llama_bench_args) {
        cmd_params params = cmd_params_defaults;
        std::vector<cmd_params_instance> params_instances = get_cmd_params_instances(params);
        std::cout << "Using llama-bench arguments for model and context creation...\n" << std::endl;
        use_llama_bench_args = true;
        llama_bench_args = params_instances[0];
    }

    // Set the default policy for all requests.
    if (arguments.state_save) {
        general_policy = Policy::save_load_full;
    } else if (arguments.kv_save) {
        general_policy = Policy::save_load_kv;
    } else {
        general_policy = Policy::recompute;
    }

    init_shm(true, comm, arguments.shm_location);
    ggml_time_init();

    struct ggml_threadpool_params tpp_batch = ggml_threadpool_params_default(n_threads);
    struct ggml_threadpool_params tpp = ggml_threadpool_params_default(n_threads);

    set_process_priority(GGML_SCHED_PRIO_NORMAL);

    threadpool_batch = NULL;
    if (!ggml_threadpool_params_match(&tpp, &tpp_batch)) {
        threadpool_batch = ggml_threadpool_new(&tpp_batch);
        if (!threadpool_batch) {
            return 1;
        }

        // Start the non-batch threadpool in the paused state
        tpp.paused = true;
    }

    threadpool = ggml_threadpool_new(&tpp);
    if (!threadpool) {
        return 1;
    }

    prepare_llama_cpp(arguments);
    // OPTIMIZE: No need for 2 passes to check if empty and to schedule next

    std::thread req_list_updater{update_req_list};
    std::thread control_scanner{scan_control};

    printf("Server accepting requests now!\n");
    const auto t_main_start = ggml_time_us();
    while (run) {
        if(arguments.token_limit < server_total_nb_tokens_generated){
            break;
        }
        remove_completed_reqs();
        if (!reqs_available()) {
            continue;
        }
        LOG_MSG(stdout, "[TEO_DEBUG]: Get next request to be processes\n");
        ClientRequest &request = schedule_next();
        LOG_MSG(stdout, "[TEO_DEBUG]: Selected request %d that wants to generate %d characters\n",
               request.id, request.n_chars_to_gen);
        handle_request(request, arguments.tokens_to_gen, arguments.throughput_limit);
    }
    const auto t_main_end = ggml_time_us();
    std::cout << "Shutdown request received, shutting down the server...";

    if (!arguments.performance_metrics_file.empty()) {
        float throughput =
            server_total_nb_tokens_generated / ((t_main_end - t_main_start) / 1000000.0f);

        std::chrono::duration<long int, std::ratio<1, 1000>> avg_ctx_creation_duration{0};
        if (nb_context_creations != 0) {
            avg_ctx_creation_duration = total_context_creation_duration / nb_context_creations;
        }

        std::chrono::duration<long int, std::ratio<1, 1000>> avg_ctx_switch_duration{0};
        if (nb_context_switches != 0) {
            avg_ctx_switch_duration = total_context_switch_duration / nb_context_switches;
        }

        std::ofstream metric_file(arguments.performance_metrics_file.c_str());
        metric_file << "Tokens generated: " << server_total_nb_tokens_generated << "\n"
                    << "Time spent: " << (t_main_end - t_main_start) / 1000000.0f << "\n"
                    << "Throughput (t/s): " << throughput << "\n"
                    << "Time spent on context creation: " << total_context_creation_duration << "\n"
                    << "Number of context creations: " << nb_context_creations << "\n"
                    << "Average context creation time: " << avg_ctx_creation_duration << "\n"
                    << "Time spent on context switches: " << total_context_switch_duration << "\n"
                    << "Number of context switches: " << nb_context_switches << "\n"
                    << "Average context switch time: " << avg_ctx_switch_duration << std::endl;
        metric_file.close();
    }

    if (!arguments.sig_file.empty()) {
        std::remove(arguments.sig_file.c_str());
    }

    req_list_updater.join();
    control_scanner.join();

    ggml_threadpool_free(threadpool);
    ggml_threadpool_free(threadpool_batch);

    clean_llama_cpp();
    // free(result);
    return 0;
}
