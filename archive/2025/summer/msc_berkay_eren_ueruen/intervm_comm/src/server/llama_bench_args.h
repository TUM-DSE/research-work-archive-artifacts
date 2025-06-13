#pragma once

#include <regex>
#include "llama.h"
#include "common.h"

template <class T>
static std::string join(const std::vector<T>& values, const std::string& delim) {
    std::ostringstream str;
    for (size_t i = 0; i < values.size(); i++) {
        str << values[i];
        if (i < values.size() - 1) {
            str << delim;
        }
    }
    return str.str();
}

// command line params
enum output_formats { NONE, CSV, JSON, JSONL, MARKDOWN };

static bool output_format_from_str(const std::string& s, output_formats& format) {
    if (s == "none") {
        format = NONE;
    } else if (s == "csv") {
        format = CSV;
    } else if (s == "json") {
        format = JSON;
    } else if (s == "jsonl") {
        format = JSONL;
    } else if (s == "md") {
        format = MARKDOWN;
    } else {
        return false;
    }
    return true;
}

struct cmd_params {
    std::vector<std::string> model;
    std::vector<int> n_prompt;
    std::vector<int> n_gen;
    std::vector<std::pair<int, int>> n_pg;
    std::vector<int> n_batch;
    std::vector<int> n_ubatch;
    std::vector<ggml_type> type_k;
    std::vector<ggml_type> type_v;
    std::vector<int> n_threads;
    std::vector<std::string> cpu_mask;
    std::vector<bool> cpu_strict;
    std::vector<int> poll;
    std::vector<int> n_gpu_layers;
    std::vector<std::string> rpc_servers;
    std::vector<llama_split_mode> split_mode;
    std::vector<int> main_gpu;
    std::vector<bool> no_kv_offload;
    std::vector<bool> flash_attn;
    std::vector<std::vector<float>> tensor_split;
    std::vector<bool> use_mmap;
    std::vector<bool> embeddings;
    ggml_numa_strategy numa;
    int reps;
    ggml_sched_priority prio;
    int delay;
    bool verbose;
    bool progress;
    output_formats output_format;
    output_formats output_format_stderr;
};

static const cmd_params cmd_params_defaults = {
    /* model                */ {"models/7B/ggml-model-q4_0.gguf"},
    /* n_prompt             */ {512},
    /* n_gen                */ {128},
    /* n_pg                 */ {},
    /* n_batch              */ {2048},
    /* n_ubatch             */ {512},
    /* type_k               */ {GGML_TYPE_F16},
    /* type_v               */ {GGML_TYPE_F16},
    /* n_threads            */ {cpu_get_num_math()},
    /* cpu_mask             */ {"0x0"},
    /* cpu_strict           */ {false},
    /* poll                 */ {50},
    /* n_gpu_layers         */ {99},
    /* rpc_servers          */ {""},
    /* split_mode           */ {LLAMA_SPLIT_MODE_LAYER},
    /* main_gpu             */ {0},
    /* no_kv_offload        */ {false},
    /* flash_attn           */ {false},
    /* tensor_split         */ {std::vector<float>(llama_max_devices(), 0.0f)},
    /* use_mmap             */ {true},
    /* embeddings           */ {false},
    /* numa                 */ GGML_NUMA_STRATEGY_DISABLED,
    /* reps                 */ 5,
    /* prio                 */ GGML_SCHED_PRIO_NORMAL,
    /* delay                */ 0,
    /* verbose              */ false,
    /* progress             */ false,
    /* output_format        */ MARKDOWN,
    /* output_format_stderr */ NONE,
};

static ggml_type ggml_type_from_name(const std::string& s) {
    if (s == "f16") {
        return GGML_TYPE_F16;
    }
    if (s == "q8_0") {
        return GGML_TYPE_Q8_0;
    }
    if (s == "q4_0") {
        return GGML_TYPE_Q4_0;
    }
    if (s == "q4_1") {
        return GGML_TYPE_Q4_1;
    }
    if (s == "q5_0") {
        return GGML_TYPE_Q5_0;
    }
    if (s == "q5_1") {
        return GGML_TYPE_Q5_1;
    }
    if (s == "iq4_nl") {
        return GGML_TYPE_IQ4_NL;
    }

    return GGML_TYPE_COUNT;
}

static cmd_params parse_cmd_params(int argc, char** argv) {
    cmd_params params;
    std::string arg;
    bool invalid_param = false;
    const std::string arg_prefix = "--";
    const char split_delim = ',';

    params.verbose = cmd_params_defaults.verbose;
    params.output_format = cmd_params_defaults.output_format;
    params.output_format_stderr = cmd_params_defaults.output_format_stderr;
    params.reps = cmd_params_defaults.reps;
    params.numa = cmd_params_defaults.numa;
    params.prio = cmd_params_defaults.prio;
    params.delay = cmd_params_defaults.delay;
    params.progress = cmd_params_defaults.progress;

    for (int i = 1; i < argc; i++) {
        arg = argv[i];
        if (arg.compare(0, arg_prefix.size(), arg_prefix) == 0) {
            std::replace(arg.begin(), arg.end(), '_', '-');
        }

        if (arg == "-h" || arg == "--help") {
            exit(0);
        } else if (arg == "-m" || arg == "--model") {
            if (++i >= argc) {
                invalid_param = true;
                break;
            }
            auto p = string_split<std::string>(argv[i], split_delim);
            params.model.insert(params.model.end(), p.begin(), p.end());
        } else if (arg == "-p" || arg == "--n-prompt") {
            if (++i >= argc) {
                invalid_param = true;
                break;
            }
            auto p = string_split<int>(argv[i], split_delim);
            params.n_prompt.insert(params.n_prompt.end(), p.begin(), p.end());
        } else if (arg == "-n" || arg == "--n-gen") {
            if (++i >= argc) {
                invalid_param = true;
                break;
            }
            auto p = string_split<int>(argv[i], split_delim);
            params.n_gen.insert(params.n_gen.end(), p.begin(), p.end());
        } else if (arg == "-pg") {
            if (++i >= argc) {
                invalid_param = true;
                break;
            }
            auto p = string_split<std::string>(argv[i], ',');
            if (p.size() != 2) {
                invalid_param = true;
                break;
            }
            params.n_pg.push_back({std::stoi(p[0]), std::stoi(p[1])});
        } else if (arg == "-b" || arg == "--batch-size") {
            if (++i >= argc) {
                invalid_param = true;
                break;
            }
            auto p = string_split<int>(argv[i], split_delim);
            params.n_batch.insert(params.n_batch.end(), p.begin(), p.end());
        } else if (arg == "-ub" || arg == "--ubatch-size") {
            if (++i >= argc) {
                invalid_param = true;
                break;
            }
            auto p = string_split<int>(argv[i], split_delim);
            params.n_ubatch.insert(params.n_ubatch.end(), p.begin(), p.end());
        } else if (arg == "-ctk" || arg == "--cache-type-k") {
            if (++i >= argc) {
                invalid_param = true;
                break;
            }
            auto p = string_split<std::string>(argv[i], split_delim);
            std::vector<ggml_type> types;
            for (const auto& t : p) {
                ggml_type gt = ggml_type_from_name(t);
                if (gt == GGML_TYPE_COUNT) {
                    invalid_param = true;
                    break;
                }
                types.push_back(gt);
            }
            if (invalid_param) {
                break;
            }
            params.type_k.insert(params.type_k.end(), types.begin(), types.end());
        } else if (arg == "-ctv" || arg == "--cache-type-v") {
            if (++i >= argc) {
                invalid_param = true;
                break;
            }
            auto p = string_split<std::string>(argv[i], split_delim);
            std::vector<ggml_type> types;
            for (const auto& t : p) {
                ggml_type gt = ggml_type_from_name(t);
                if (gt == GGML_TYPE_COUNT) {
                    invalid_param = true;
                    break;
                }
                types.push_back(gt);
            }
            if (invalid_param) {
                break;
            }
            params.type_v.insert(params.type_v.end(), types.begin(), types.end());
        } else if (arg == "-t" || arg == "--threads") {
            if (++i >= argc) {
                invalid_param = true;
                break;
            }
            auto p = string_split<int>(argv[i], split_delim);
            params.n_threads.insert(params.n_threads.end(), p.begin(), p.end());
        } else if (arg == "-C" || arg == "--cpu-mask") {
            if (++i >= argc) {
                invalid_param = true;
                break;
            }
            auto p = string_split<std::string>(argv[i], split_delim);
            params.cpu_mask.insert(params.cpu_mask.end(), p.begin(), p.end());
        } else if (arg == "--cpu-strict") {
            if (++i >= argc) {
                invalid_param = true;
                break;
            }
            auto p = string_split<bool>(argv[i], split_delim);
            params.cpu_strict.insert(params.cpu_strict.end(), p.begin(), p.end());
        } else if (arg == "--poll") {
            if (++i >= argc) {
                invalid_param = true;
                break;
            }
            auto p = string_split<int>(argv[i], split_delim);
            params.poll.insert(params.poll.end(), p.begin(), p.end());
        } else if (arg == "-ngl" || arg == "--n-gpu-layers") {
            if (++i >= argc) {
                invalid_param = true;
                break;
            }
            auto p = string_split<int>(argv[i], split_delim);
            params.n_gpu_layers.insert(params.n_gpu_layers.end(), p.begin(), p.end());
        } else if (llama_supports_rpc() && (arg == "-rpc" || arg == "--rpc")) {
            if (++i >= argc) {
                invalid_param = true;
                break;
            }
            params.rpc_servers.push_back(argv[i]);
        } else if (arg == "-sm" || arg == "--split-mode") {
            if (++i >= argc) {
                invalid_param = true;
                break;
            }
            auto p = string_split<std::string>(argv[i], split_delim);
            std::vector<llama_split_mode> modes;
            for (const auto& m : p) {
                llama_split_mode mode;
                if (m == "none") {
                    mode = LLAMA_SPLIT_MODE_NONE;
                } else if (m == "layer") {
                    mode = LLAMA_SPLIT_MODE_LAYER;
                } else if (m == "row") {
                    mode = LLAMA_SPLIT_MODE_ROW;
                } else {
                    invalid_param = true;
                    break;
                }
                modes.push_back(mode);
            }
            if (invalid_param) {
                break;
            }
            params.split_mode.insert(params.split_mode.end(), modes.begin(), modes.end());
        } else if (arg == "-mg" || arg == "--main-gpu") {
            if (++i >= argc) {
                invalid_param = true;
                break;
            }
            params.main_gpu = string_split<int>(argv[i], split_delim);
        } else if (arg == "-nkvo" || arg == "--no-kv-offload") {
            if (++i >= argc) {
                invalid_param = true;
                break;
            }
            auto p = string_split<bool>(argv[i], split_delim);
            params.no_kv_offload.insert(params.no_kv_offload.end(), p.begin(), p.end());
        } else if (arg == "--numa") {
            if (++i >= argc) {
                invalid_param = true;
                break;
            } else {
                std::string value(argv[i]);
                /**/ if (value == "distribute" || value == "") {
                    params.numa = GGML_NUMA_STRATEGY_DISTRIBUTE;
                } else if (value == "isolate") {
                    params.numa = GGML_NUMA_STRATEGY_ISOLATE;
                } else if (value == "numactl") {
                    params.numa = GGML_NUMA_STRATEGY_NUMACTL;
                } else {
                    invalid_param = true;
                    break;
                }
            }
        } else if (arg == "-fa" || arg == "--flash-attn") {
            if (++i >= argc) {
                invalid_param = true;
                break;
            }
            auto p = string_split<bool>(argv[i], split_delim);
            params.flash_attn.insert(params.flash_attn.end(), p.begin(), p.end());
        } else if (arg == "-mmp" || arg == "--mmap") {
            if (++i >= argc) {
                invalid_param = true;
                break;
            }
            auto p = string_split<bool>(argv[i], split_delim);
            params.use_mmap.insert(params.use_mmap.end(), p.begin(), p.end());
        } else if (arg == "-embd" || arg == "--embeddings") {
            if (++i >= argc) {
                invalid_param = true;
                break;
            }
            auto p = string_split<bool>(argv[i], split_delim);
            params.embeddings.insert(params.embeddings.end(), p.begin(), p.end());
        } else if (arg == "-ts" || arg == "--tensor-split") {
            if (++i >= argc) {
                invalid_param = true;
                break;
            }
            for (auto ts : string_split<std::string>(argv[i], split_delim)) {
                // split string by ; and /
                const std::regex regex{R"([;/]+)"};
                std::sregex_token_iterator it{ts.begin(), ts.end(), regex, -1};
                std::vector<std::string> split_arg{it, {}};
                GGML_ASSERT(split_arg.size() <= llama_max_devices());

                std::vector<float> tensor_split(llama_max_devices());
                for (size_t i = 0; i < llama_max_devices(); ++i) {
                    if (i < split_arg.size()) {
                        tensor_split[i] = std::stof(split_arg[i]);
                    } else {
                        tensor_split[i] = 0.0f;
                    }
                }
                params.tensor_split.push_back(tensor_split);
            }
        } else if (arg == "-r" || arg == "--repetitions") {
            if (++i >= argc) {
                invalid_param = true;
                break;
            }
            params.reps = std::stoi(argv[i]);
        } else if (arg == "--prio") {
            if (++i >= argc) {
                invalid_param = true;
                break;
            }
            params.prio = (enum ggml_sched_priority)std::stoi(argv[i]);
        } else if (arg == "--delay") {
            if (++i >= argc) {
                invalid_param = true;
                break;
            }
            params.delay = std::stoi(argv[i]);
        } else if (arg == "-o" || arg == "--output") {
            if (++i >= argc) {
                invalid_param = true;
                break;
            }
            invalid_param = !output_format_from_str(argv[i], params.output_format);
        } else if (arg == "-oe" || arg == "--output-err") {
            if (++i >= argc) {
                invalid_param = true;
                break;
            }
            invalid_param = !output_format_from_str(argv[i], params.output_format_stderr);
        } else if (arg == "-v" || arg == "--verbose") {
            params.verbose = true;
        } else if (arg == "--progress") {
            params.progress = true;
        } else {
            invalid_param = true;
            break;
        }
    }
    if (invalid_param) {
        fprintf(stderr, "error: invalid parameter for argument: %s\n", arg.c_str());

        exit(1);
    }

    // set defaults
    if (params.model.empty()) {
        params.model = cmd_params_defaults.model;
    }
    if (params.n_prompt.empty()) {
        params.n_prompt = cmd_params_defaults.n_prompt;
    }
    if (params.n_gen.empty()) {
        params.n_gen = cmd_params_defaults.n_gen;
    }
    if (params.n_pg.empty()) {
        params.n_pg = cmd_params_defaults.n_pg;
    }
    if (params.n_batch.empty()) {
        params.n_batch = cmd_params_defaults.n_batch;
    }
    if (params.n_ubatch.empty()) {
        params.n_ubatch = cmd_params_defaults.n_ubatch;
    }
    if (params.type_k.empty()) {
        params.type_k = cmd_params_defaults.type_k;
    }
    if (params.type_v.empty()) {
        params.type_v = cmd_params_defaults.type_v;
    }
    if (params.n_gpu_layers.empty()) {
        params.n_gpu_layers = cmd_params_defaults.n_gpu_layers;
    }
    if (params.rpc_servers.empty()) {
        params.rpc_servers = cmd_params_defaults.rpc_servers;
    }
    if (params.split_mode.empty()) {
        params.split_mode = cmd_params_defaults.split_mode;
    }
    if (params.main_gpu.empty()) {
        params.main_gpu = cmd_params_defaults.main_gpu;
    }
    if (params.no_kv_offload.empty()) {
        params.no_kv_offload = cmd_params_defaults.no_kv_offload;
    }
    if (params.flash_attn.empty()) {
        params.flash_attn = cmd_params_defaults.flash_attn;
    }
    if (params.tensor_split.empty()) {
        params.tensor_split = cmd_params_defaults.tensor_split;
    }
    if (params.use_mmap.empty()) {
        params.use_mmap = cmd_params_defaults.use_mmap;
    }
    if (params.embeddings.empty()) {
        params.embeddings = cmd_params_defaults.embeddings;
    }
    if (params.n_threads.empty()) {
        params.n_threads = cmd_params_defaults.n_threads;
    }
    if (params.cpu_mask.empty()) {
        params.cpu_mask = cmd_params_defaults.cpu_mask;
    }
    if (params.cpu_strict.empty()) {
        params.cpu_strict = cmd_params_defaults.cpu_strict;
    }
    if (params.poll.empty()) {
        params.poll = cmd_params_defaults.poll;
    }

    return params;
}

struct cmd_params_instance {
    std::string model;
    int n_prompt;
    int n_gen;
    int n_batch;
    int n_ubatch;
    ggml_type type_k;
    ggml_type type_v;
    int n_threads;
    std::string cpu_mask;
    bool cpu_strict;
    int poll;
    int n_gpu_layers;
    std::string rpc_servers;
    llama_split_mode split_mode;
    int main_gpu;
    bool no_kv_offload;
    bool flash_attn;
    std::vector<float> tensor_split;
    bool use_mmap;
    bool embeddings;

    llama_model_params to_llama_mparams() const {
        llama_model_params mparams = llama_model_default_params();

        mparams.n_gpu_layers = n_gpu_layers;
        if (!rpc_servers.empty()) {
            mparams.rpc_servers = rpc_servers.c_str();
        }
        mparams.split_mode = split_mode;
        mparams.main_gpu = main_gpu;
        mparams.tensor_split = tensor_split.data();
        mparams.use_mmap = use_mmap;

        return mparams;
    }

    bool equal_mparams(const cmd_params_instance& other) const {
        return model == other.model && n_gpu_layers == other.n_gpu_layers &&
               rpc_servers == other.rpc_servers && split_mode == other.split_mode &&
               main_gpu == other.main_gpu && use_mmap == other.use_mmap &&
               tensor_split == other.tensor_split;
    }

    llama_context_params to_llama_cparams() const {
        llama_context_params cparams = llama_context_default_params();

        cparams.n_ctx = n_prompt + n_gen;
        cparams.n_batch = n_batch;
        cparams.n_ubatch = n_ubatch;
        cparams.type_k = type_k;
        cparams.type_v = type_v;
        cparams.offload_kqv = !no_kv_offload;
        cparams.flash_attn = flash_attn;
        cparams.embeddings = embeddings;

        return cparams;
    }
};

static std::vector<cmd_params_instance> get_cmd_params_instances(const cmd_params& params) {
    std::vector<cmd_params_instance> instances;

    // this ordering minimizes the number of times that each model needs to be reloaded
    for (const auto& m : params.model)
        for (const auto& nl : params.n_gpu_layers)
            for (const auto& rpc : params.rpc_servers)
                for (const auto& sm : params.split_mode)
                    for (const auto& mg : params.main_gpu)
                        for (const auto& ts : params.tensor_split)
                            for (const auto& mmp : params.use_mmap)
                                for (const auto& embd : params.embeddings)
                                    for (const auto& nb : params.n_batch)
                                        for (const auto& nub : params.n_ubatch)
                                            for (const auto& tk : params.type_k)
                                                for (const auto& tv : params.type_v)
                                                    for (const auto& nkvo : params.no_kv_offload)
                                                        for (const auto& fa : params.flash_attn)
                                                            for (const auto& nt : params.n_threads)
                                                                for (const auto& cm :
                                                                     params.cpu_mask)
                                                                    for (const auto& cs :
                                                                         params.cpu_strict)
                                                                        for (const auto& pl :
                                                                             params.poll) {
                                                                            for (const auto&
                                                                                     n_prompt :
                                                                                 params.n_prompt) {
                                                                                if (n_prompt == 0) {
                                                                                    continue;
                                                                                }
                                                                                cmd_params_instance
                                                                                    instance = {
                                                                                        /* .model =
                                                                                         */
                                                                                        m,
                                                                                        /* .n_prompt
                                                                                           = */
                                                                                        n_prompt,
                                                                                        /* .n_gen =
                                                                                         */
                                                                                        0,
                                                                                        /* .n_batch
                                                                                           = */
                                                                                        nb,
                                                                                        /* .n_ubatch
                                                                                           = */
                                                                                        nub,
                                                                                        /* .type_k
                                                                                           = */
                                                                                        tk,
                                                                                        /* .type_v
                                                                                           = */
                                                                                        tv,
                                                                                        /* .n_threads
                                                                                           = */
                                                                                        nt,
                                                                                        /* .cpu_mask
                                                                                           = */
                                                                                        cm,
                                                                                        /* .cpu_strict
                                                                                           = */
                                                                                        cs,
                                                                                        /* .poll =
                                                                                         */
                                                                                        pl,
                                                                                        /* .n_gpu_layers
                                                                                           = */
                                                                                        nl,
                                                                                        /* .rpc_servers
                                                                                           = */
                                                                                        rpc,
                                                                                        /* .split_mode
                                                                                           = */
                                                                                        sm,
                                                                                        /* .main_gpu
                                                                                           = */
                                                                                        mg,
                                                                                        /* .no_kv_offload=
                                                                                         */
                                                                                        nkvo,
                                                                                        /* .flash_attn
                                                                                           = */
                                                                                        fa,
                                                                                        /* .tensor_split
                                                                                           = */
                                                                                        ts,
                                                                                        /* .use_mmap
                                                                                           = */
                                                                                        mmp,
                                                                                        /* .embeddings
                                                                                           = */
                                                                                        embd,
                                                                                    };
                                                                                instances.push_back(
                                                                                    instance);
                                                                            }

                                                                            for (const auto& n_gen :
                                                                                 params.n_gen) {
                                                                                if (n_gen == 0) {
                                                                                    continue;
                                                                                }
                                                                                cmd_params_instance
                                                                                    instance = {
                                                                                        /* .model =
                                                                                         */
                                                                                        m,
                                                                                        /* .n_prompt
                                                                                           = */
                                                                                        0,
                                                                                        /* .n_gen =
                                                                                         */
                                                                                        n_gen,
                                                                                        /* .n_batch
                                                                                           = */
                                                                                        nb,
                                                                                        /* .n_ubatch
                                                                                           = */
                                                                                        nub,
                                                                                        /* .type_k
                                                                                           = */
                                                                                        tk,
                                                                                        /* .type_v
                                                                                           = */
                                                                                        tv,
                                                                                        /* .n_threads
                                                                                           = */
                                                                                        nt,
                                                                                        /* .cpu_mask
                                                                                           = */
                                                                                        cm,
                                                                                        /* .cpu_strict
                                                                                           = */
                                                                                        cs,
                                                                                        /* .poll =
                                                                                         */
                                                                                        pl,
                                                                                        /* .n_gpu_layers
                                                                                           = */
                                                                                        nl,
                                                                                        /* .rpc_servers
                                                                                           = */
                                                                                        rpc,
                                                                                        /* .split_mode
                                                                                           = */
                                                                                        sm,
                                                                                        /* .main_gpu
                                                                                           = */
                                                                                        mg,
                                                                                        /* .no_kv_offload=
                                                                                         */
                                                                                        nkvo,
                                                                                        /* .flash_attn
                                                                                           = */
                                                                                        fa,
                                                                                        /* .tensor_split
                                                                                           = */
                                                                                        ts,
                                                                                        /* .use_mmap
                                                                                           = */
                                                                                        mmp,
                                                                                        /* .embeddings
                                                                                           = */
                                                                                        embd,
                                                                                    };
                                                                                instances.push_back(
                                                                                    instance);
                                                                            }

                                                                            for (const auto& n_pg :
                                                                                 params.n_pg) {
                                                                                if (n_pg.first ==
                                                                                        0 &&
                                                                                    n_pg.second ==
                                                                                        0) {
                                                                                    continue;
                                                                                }
                                                                                cmd_params_instance
                                                                                    instance = {
                                                                                        /* .model =
                                                                                         */
                                                                                        m,
                                                                                        /* .n_prompt
                                                                                           = */
                                                                                        n_pg.first,
                                                                                        /* .n_gen =
                                                                                         */
                                                                                        n_pg.second,
                                                                                        /* .n_batch
                                                                                           = */
                                                                                        nb,
                                                                                        /* .n_ubatch
                                                                                           = */
                                                                                        nub,
                                                                                        /* .type_k
                                                                                           = */
                                                                                        tk,
                                                                                        /* .type_v
                                                                                           = */
                                                                                        tv,
                                                                                        /* .n_threads
                                                                                           = */
                                                                                        nt,
                                                                                        /* .cpu_mask
                                                                                           = */
                                                                                        cm,
                                                                                        /* .cpu_strict
                                                                                           = */
                                                                                        cs,
                                                                                        /* .poll =
                                                                                         */
                                                                                        pl,
                                                                                        /* .n_gpu_layers
                                                                                           = */
                                                                                        nl,
                                                                                        /* .rpc_servers
                                                                                           = */
                                                                                        rpc,
                                                                                        /* .split_mode
                                                                                           = */
                                                                                        sm,
                                                                                        /* .main_gpu
                                                                                           = */
                                                                                        mg,
                                                                                        /* .no_kv_offload=
                                                                                         */
                                                                                        nkvo,
                                                                                        /* .flash_attn
                                                                                           = */
                                                                                        fa,
                                                                                        /* .tensor_split
                                                                                           = */
                                                                                        ts,
                                                                                        /* .use_mmap
                                                                                           = */
                                                                                        mmp,
                                                                                        /* .embeddings
                                                                                           = */
                                                                                        embd,
                                                                                    };
                                                                                instances.push_back(
                                                                                    instance);
                                                                            }
                                                                        }

    return instances;
}