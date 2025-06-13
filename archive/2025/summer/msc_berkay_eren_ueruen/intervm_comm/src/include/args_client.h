#include <argp.h>

#include <iostream>
#include <string>

#define ARGUMENT_SHM_SHORT 0x1

// Structure to hold parsed arguments
struct Arguments {
    int id;
    int prio;
    int lora_number = 0;  // defaults to no lora (0)
    int repeats = -1;     // defaults zo infinite
    int sleep_time = 3;
    int clean = 0;
    int shutdown = 0;
    float throughput = 999;
    int active_throughput = 0;
    std::string shm_location;
};

// Program documentation
static char doc[] = "Client-side argument parser using argp.h";

// Options
static struct argp_option options[] = {
    {"id", 'i', "ID", 0, "User ID (Required)"},
    {"prio", 'p', "PRIO", 0, "Priority level (Default = 0)"},
    {"lora_number", 'l', "NUM", 0, "Number of LoRA (Default = 0 / No LoRA)"},
    {"repeat", 'r', "REP", 0, "Number of requests to send (Default = -1 / Infinite)"},
    {"sleep", 's', "SLP", 0, "Seconds to sleep between requests (Default = 3). 0 means no sleep"},
    {"throughput", 'd', "THORUGHPUT", 0, "Throughput limit"},
    {"active-throughput", 'a', 0, 0, "Creates a thread, listening the user input for new throughput limits"},
    {"shutdown", 'w', 0, 0, "Send shutdown request to the server"},
    {"shared-mem", ARGUMENT_SHM_SHORT, "SHM_LOCATION", 0, "Specify shm locations"},
    {0}};

// Argument parser function
static error_t parse_opt(int key, char *arg, struct argp_state *state) {
    Arguments *arguments = static_cast<Arguments *>(state->input);

    switch (key) {
        case 'i':
            arguments->id = std::stoi(arg);
            break;
        case 'p':
            arguments->prio = std::stoi(arg);
            break;
        case 'l':
            arguments->lora_number = std::stoi(arg);
            break;
        case 'r':
            arguments->repeats = std::stoi(arg);
            break;
        case 's':
            arguments->sleep_time = std::stoi(arg);
            break;
        case 'd':
            arguments->throughput = std::stof(arg);
            break;
        case 'a':
            arguments->active_throughput = 1;
            break;
        case 'w':
            arguments->shutdown = 1;
            arguments->prio = 0;  // highest prio
            arguments->repeats = 1;
            break;
        case ARGUMENT_SHM_SHORT:
            arguments->shm_location = arg;
            break;
        case ARGP_KEY_END:
            if (arguments->id == 0 && arguments->shutdown == 0) {
                argp_usage(state);  // Print usage and exit if required args are missing
            }
            break;
        default:
            return ARGP_ERR_UNKNOWN;
    }
    return 0;
}

// Argp parser setup
static struct argp argp = {options, parse_opt, nullptr, doc};
