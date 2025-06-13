#pragma once

#include "defines.h"
#include "llama.h"
#include "shm.h"

class ClientRequest {
private:
    // Raw request lying in the shared memory
    Request *request;

public:
    std::string prompt;
    int n_chars_to_gen;
    int id;
    int prio;
    enum Lora lora;
    float throughput = 999;

    // Following are used to keep the llama.cpp context
    std::vector<llama_token> prompt_tokens;
    llama_batch batch;
    llama_context *ctx;
    llama_sampler *smpl;
    int new_token_id;
    int n_generated_char = 0;
    int n_past_tokens = 0;
    uint8_t *state_save = NULL;

    // KV Cache
    enum Policy policy = general_policy;  // Apply the default policy at start
    uint8_t *kv_cache_backup = NULL;
    size_t kv_cache_size = 0;

    bool initialized = false;  // Each new request must be initilized with a new context
    bool done = false;         // We remove the done requests at the main loop
    bool require_context_switch =
        false;  // True if the request is newly scheduled after a different request

    // For debugging purposes
    int scheduler_counter = 0;
    llama_kv_cache_view kv_view;
    int n_past_tokens_mod = 0;
    bool limit_hit = false;

    ClientRequest(Request *request, std::string prompt, int n_chars_to_gen, int id, int prio,
                  enum Lora lora)
        : request(request),
          prompt(prompt),
          n_chars_to_gen(n_chars_to_gen),
          id(id),
          prio(prio),
          lora(lora) {
    }

    Request *get_raw_request() {
        return request;
    }
    int get_current_length() {
        return strlen(request->text);
    };

    float get_throughput() {
        if (pthread_mutex_trylock(&request->throughput_mutex) == 0) {
            throughput = request->throughput;
            pthread_mutex_unlock(&request->throughput_mutex);
        }

        return throughput;
    }

    friend std::ostream &operator<<(std::ostream &os, const ClientRequest &obj) {
        os << "Printing request...\n"
           << "\t- id: " << obj.id << "\n"
           << "\t- prio: " << obj.prio << "\n"
           << "\t- lora: " << obj.lora << "\n"
           << "\t- # of times this request has been scheduled: " << obj.scheduler_counter
           << "\n"
           //   << "\t- # of requests from the client: " << client_request_counter[obj.id] <<
           //   std::endl;
           << std::endl;
        return os;
    }

    void print_kv_cache() {
        std::cout << "Used cells: " << kv_view.used_cells << "\\" << kv_view.n_cells << std::endl;
    }

    void apply_lora() {
        if (lora != NO_LORA) {
            if ((size_t)lora >= lora_adapters.size() + 1) {
                std::cout << "Wrong lora index" << std::endl;
                // We can continue without applying any lora here
                // To detect the issue easily we exit for now
                exit(0);
            } else {
                llama_lora_adapter_set(ctx, lora_adapters.at(lora - 1), 1);
            }
        }
    }

    void init_from_clean_kv() {
        std::cout << "\tLoading context without kv cache..." << std::endl;
        // initialize the context
        llama_context_params ctx_params = llama_context_default_params();
        // n_ctx is the context size
        ctx_params.n_ctx = CTX_SIZE;
        // n_batch is the maximum number of tokens that can be processed in a
        // single call to llama_decode ctx_params.n_batch = n_prompt;
        ctx_params.n_batch = BATCH_SIZE;
        // enable performance counters
        ctx_params.no_perf = false;
        ctx_params.offload_kqv = true;

        if (!use_llama_bench_args) {
            ctx = llama_new_context_with_model(model, ctx_params);
        } else {
            ctx = llama_new_context_with_model(model, llama_bench_args.to_llama_cparams());
        }

        if (ctx == NULL) {
            fprintf(stderr, "%s: error: failed to create the llama_context\n", __func__);
            exit(1);
        }

        // initialize the sampler

        auto sparams = llama_sampler_chain_default_params();
        sparams.no_perf = false;
        smpl = llama_sampler_chain_init(sparams);

        llama_sampler_chain_add(smpl, llama_sampler_init_greedy());

        n_past_tokens = 0;

        // prepare the batch
        batch = llama_batch_init(prompt_tokens.size(), 0, 1);
        for (size_t i = 0; i < prompt_tokens.size(); i++) {
            common_batch_add(batch, prompt_tokens[i], i, {0}, false);
        }
        batch.logits[batch.n_tokens - 1] = true;  // generate next token
    }

    void init_from_saved_kv() {
        std::cout << "\tLoading kv cache..." << std::endl;
        // initialize the context
        llama_context_params ctx_params = llama_context_default_params();
        // n_ctx is the context size
        ctx_params.n_ctx = CTX_SIZE;
        // n_batch is the maximum number of tokens that can be processed in a
        // single call to llama_decode ctx_params.n_batch = n_prompt;
        ctx_params.n_batch = BATCH_SIZE;
        // enable performance counters
        ctx_params.no_perf = false;
        ctx_params.offload_kqv = true;

        if (!use_llama_bench_args) {
            ctx = llama_new_context_with_model(model, ctx_params);
        } else {
            ctx = llama_new_context_with_model(model, llama_bench_args.to_llama_cparams());
        }

        if (ctx == NULL) {
            fprintf(stderr, "%s: error: failed to create the llama_context\n", __func__);
            exit(1);
        }

        // initialize the sampler

        auto sparams = llama_sampler_chain_default_params();
        sparams.no_perf = false;
        smpl = llama_sampler_chain_init(sparams);

        llama_sampler_chain_add(smpl, llama_sampler_init_greedy());

        llama_state_seq_set_data(ctx, kv_cache_backup, kv_cache_size, 0);

        // set_state_data does not restore the batch, therefore we need to reinit it (since we freed
        // it) and add the last sampled token
        batch = llama_batch_init(1, 0, 1);
        common_batch_clear(batch);
        common_batch_add(batch, new_token_id, n_past_tokens, {0}, true);
        batch.logits[batch.n_tokens - 1] = true;
    }

    void init_ctx_from_save_state() {
        std::cout << "\tLoading saved context..." << std::endl;
        // initialize the context

        llama_context_params ctx_params = llama_context_default_params();
        // n_ctx is the context size
        ctx_params.n_ctx = CTX_SIZE;
        // n_batch is the maximum number of tokens that can be processed in a
        // single call to llama_decode ctx_params.n_batch = n_prompt;
        ctx_params.n_batch = BATCH_SIZE;
        // enable performance counters
        ctx_params.no_perf = false;

        if (!use_llama_bench_args) {
            ctx = llama_new_context_with_model(model, ctx_params);
        } else {
            ctx = llama_new_context_with_model(model, llama_bench_args.to_llama_cparams());
        }

        if (ctx == NULL) {
            fprintf(stderr, "%s: error: failed to create the llama_context\n", __func__);
            exit(1);
        }

        // initialize the sampler

        auto sparams = llama_sampler_chain_default_params();
        sparams.no_perf = false;
        smpl = llama_sampler_chain_init(sparams);

        llama_sampler_chain_add(smpl, llama_sampler_init_greedy());

        llama_set_state_data(ctx, state_save);

        // set_state_data does not restore the batch, therefore we need to reinit it (since we freed
        // it) and add the last sampled token
        batch = llama_batch_init(1, 0, 1);
        common_batch_clear(batch);
        common_batch_add(batch, new_token_id, n_past_tokens, {0}, true);
        batch.logits[batch.n_tokens - 1] = true;
    }

    void init_llama_context() {
        // tokenize the prompt

        // find the number of tokens in the prompt
        const int n_prompt =
            -llama_tokenize(model, prompt.c_str(), prompt.size(), NULL, 0, true, true);

        // allocate space for the tokens and tokenize the prompt
        prompt_tokens.resize(n_prompt);
        if (llama_tokenize(model, prompt.c_str(), prompt.size(), prompt_tokens.data(),
                           prompt_tokens.size(), true, true) < 0) {
            fprintf(stderr, "%s: error: failed to tokenize the prompt\n", __func__);
            exit(1);
        }

        // initialize the context

        llama_context_params ctx_params = llama_context_default_params();
        // n_ctx is the context size
        ctx_params.n_ctx = CTX_SIZE;
        // n_batch is the maximum number of tokens that can be processed in a
        // single call to llama_decode ctx_params.n_batch = n_prompt;
        ctx_params.n_batch = BATCH_SIZE;
        // enable performance counters
        ctx_params.no_perf = false;

        if (!use_llama_bench_args) {
            ctx = llama_new_context_with_model(model, ctx_params);
        } else {
            ctx = llama_new_context_with_model(model, llama_bench_args.to_llama_cparams());
        }

        if (ctx == NULL) {
            fprintf(stderr, "%s: error: failed to create the llama_context\n", __func__);
            exit(1);
        }

        kv_view = llama_kv_cache_view_init(ctx, 1);

        // initialize the sampler

        auto sparams = llama_sampler_chain_default_params();
        sparams.no_perf = false;
        smpl = llama_sampler_chain_init(sparams);

        llama_sampler_chain_add(smpl, llama_sampler_init_greedy());

        // print the prompt token-by-token

        std::cout << "Request text:\n\t";
        for (auto id : prompt_tokens) {
            char buf[128];
            int n = llama_token_to_piece(model, id, buf, sizeof(buf), 0, true);
            if (n < 0) {
                fprintf(stderr, "%s: error: failed to convert token to piece\n", __func__);
                exit(1);
            }
            std::string s(buf, n);
            std::cout << s;
        }
        std::cout << std::endl;

        // prepare the batch
        batch = llama_batch_init(prompt_tokens.size(), 0, 1);
        for (size_t i = 0; i < prompt_tokens.size(); i++) {
            common_batch_add(batch, prompt_tokens[i], i, {0}, true);
        }
        batch.logits[batch.n_tokens - 1] = true;  // generate next token
    }

    void auto_set_kv_policy() {
        llama_kv_cache_view_update(ctx, &kv_view);
        if (policy != Policy::save_load_full && kv_view.used_cells >= KV_CACHE_POLICY_LIMIT) {
            policy = Policy::save_load_full;
            std::cout << "Changing KV cache policy to save/load" << std::endl;
        }
    }
};
