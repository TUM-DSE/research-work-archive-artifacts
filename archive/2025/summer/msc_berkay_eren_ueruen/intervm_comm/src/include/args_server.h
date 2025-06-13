#include <argp.h>

#include <iostream>
#include <string>
#include <vector>
#include <limits.h>
#define ARGUMENT_SHM_SHORT 0x1
#define ARGUMENT_THREAD 0x2

struct Arguments {
    std::string model_path;               // Required model path
    std::vector<std::string> lora_paths;  // Optional multiple LoRA paths
    bool state_save = false;
    bool kv_save = false;
    bool auto_policy = false;
    bool llama_bench_args;
    float throughput_limit;
    int tokens_to_gen;
    unsigned int token_limit = UINT_MAX;
    unsigned int n_threads = 1;
    std::string performance_metrics_file;
    std::string sig_file;
    std::string shm_location;
};

// Program documentation
static char doc[] = "LLM-OS Server";

// Options
static struct argp_option options[] = {
    {"model", 'm', "FILE", 0, "Path to the model file (Required)"},
    {"lora", 'l', "FILE", 0, "Path to a LoRA file (Can be specified multiple times)"},
    {"perf-metrics", 'p', "FILE", 0, "Path to a performance metrics file"},
    {"state-save", 's', 0, 0, "Enable state save and load between context switches"},
    {"kv-save", 'k', 0, 0, "Enable kv cache save and load between context switches"},
    {"auto-policy", 'a', 0, 0, "Choose the kv cache policy automatically"},
    {"llama-bench-args", 'b', 0, 0, "Use llama-bench defaults"},
    {"throughput-limit", 't', "LIMIT", 0, "Upper thoughput limit for the server (tokens/s)"},
    {"n_threads", ARGUMENT_THREAD, "N_THREADS", 0, "Number of threads for inference"},
    {"tokens-to-gen", 'g', "NB_TOKENS", 0,
     "Number of tokens to generate before engaging the round robin mechanism and performing a "
     "context switch"},
    {"token_limit", 'z', "NB_TOKENS", 0,
     "Number of total tokens to generate before quiting"},
    {"signal-file", 'f', "FILE", 0,
     "Create a temporary file at the start and delete it at the end"},
    {"shared-mem", ARGUMENT_SHM_SHORT, "SHM_LOCATION", 0, "Specify shm locations"},
    {0}};

// Argument parser function
static error_t parse_opt(int key, char *arg, struct argp_state *state) {
    Arguments *arguments = static_cast<Arguments *>(state->input);

    switch (key) {
        case 'm':
            arguments->model_path = arg;
            break;
        case 'l':
            arguments->lora_paths.push_back(arg);
            break;
        case 's':
            arguments->state_save = true;
            break;
        case 'k':
            arguments->kv_save = true;
            break;
        case 'b':
            arguments->llama_bench_args = true;
            break;
        case 'a':
            arguments->auto_policy = true;
            break;
        case 't':
            arguments->throughput_limit = atof(arg);
            break;
        case 'g':
            arguments->tokens_to_gen = atoi(arg);
            break;
        case 'p':
            arguments->performance_metrics_file = arg;
            break;
        case 'f':
            arguments->sig_file = arg;
            break;
        case 'z':
            arguments->token_limit = atoi(arg);
            break;
        case ARGUMENT_SHM_SHORT:
            arguments->shm_location = arg;
            break;
        case ARGUMENT_THREAD:
            arguments->n_threads = atoi(arg);
            break;
        case ARGP_KEY_END:
            if (arguments->model_path.empty()) {
                argp_usage(state);  // Print usage and exit if model_path is missing
            }
            break;
        default:
            return ARGP_ERR_UNKNOWN;
    }
    return 0;
}

// Argp parser setup
static struct argp argp = {options, parse_opt, nullptr, doc};
