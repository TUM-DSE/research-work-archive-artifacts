#include <assert.h>
#include <fcntl.h>
#include <string.h>
#include <sys/mman.h>
#include <unistd.h>

#include "args_client.h"
#include "shm.h"

#define MIN(a, b) (((a) < (b)) ? (a) : (b))
#define MAX(a, b) (((a) > (b)) ? (a) : (b))

int id;

struct Communication comm;

void shutdown_server() {
    while (sem_trywait(&comm.requestQueue->run) == -1) {
    };
}

void write_to_shm(struct SharedMemory* shm, const char* text, int n_chars_to_gen, int index,
                  int prio, enum Lora lora, float throughput) {
    // TODO: this zero set is a safety net
    // currently it protects the memory if a client dies without cleaning the memory
    // A better way would be register a cleaner signal handler.
    for (int i = 0; i < MAX_TEXT; i++) {
        shm->requests[index].text[i] = 0;
    }

    printf("Writing to shm index: [%d]\n"
           "-> Current string in cell [%d] (should be empty): %s\n"
           "-> Writing to cell...\n",
           index, index, shm->requests[index].text);

    for (int i = 0; i < (int)strlen(text); i++) {
        shm->requests[index].text[i] = text[i];
    }

    printf("-> Resulting string in cell: %s\n", shm->requests[index].text);

    shm->requests[index].n_chars_to_gen = n_chars_to_gen;
    shm->requests[index].id = id;
    shm->requests[index].prio = prio;
    shm->requests[index].lora = lora;
    shm->requests[index].throughput = throughput;
}

void print_from_shm(struct SharedMemory* shm, int str_len, int n_gen, int free_request_slot_num) {
    printf("\n---------- Answer ----------\n");
    bool server_done = false;
    for (int i = str_len; server_done == false && i < str_len + n_gen; i++) {
        while (shm->requests[free_request_slot_num].text[i] == '\0') {
            // Server might have generated an end of gen char before we reached the number of chars
            // we want to generate and expect. So we check if the server is done
            if (sem_trywait(&comm.requestQueue->requests[free_request_slot_num].clientNotifier) ==
                0) {
                server_done = true;
                break;
            }
        }
        printf("%c", shm->requests[free_request_slot_num].text[i]);
        fflush(stdout);
    }
    printf("\n------- End of Answer -------\n");

    // If the server is not done, we wait for it.
    if (server_done == false) {
        // sem_wait seems to cause unexpected SIGABRT (The futex facility returned an unexpected
        // error code) This is also true on server side, thus the busy waiting
        while (sem_trywait(&comm.requestQueue->requests[free_request_slot_num].clientNotifier) ==
               -1) {
        };
    }

    printf("\nRequest has been fully answered!\n");
    fflush(stdout);
}

int find_free_shm_request() {
    for (int i = 0; i < MAX_REQUESTS; i++) {
        if (pthread_mutex_trylock(&comm.requestQueue->requests[i].mutex) == 0) {
            printf("=============================\n"
                   "Free slot found: %d\n",
                   i);
            fflush(stdout);
            return i;
        }
    }
    return -1;
}

void free_request(int num) {
    printf("Freeing cell: %d\n", num);
    printf("-> Current string in cell [%d]: %s\n", num, comm.requestQueue->requests[num].text);
    for (int i = 0; i < MAX_TEXT; i++) {
        comm.requestQueue->requests[num].text[i] = 0;
    }
    printf("-> After removal cell [%d] content (should be empty): %s\n", num,
           comm.requestQueue->requests[num].text);

    pthread_mutex_unlock(&comm.requestQueue->requests[num].mutex);
}

const char* prompts[] = {"Building a website",
                         "The sky",
                         "Albert Einstein was",
                         "First rule of",
                         "Reversible computing is",
                         "In summer, trees are",
                         "Sunken cost of",
                         "In their first album, Pink Floyd",
                         "Preparing for a presentation could be",
                         "To prepare a good iskender"};

const char* prompts2[] = {
    "Best kebap",
};

volatile int free_request_slot_num;

void* throughput_input(void*) {
    while (1) {
        float new_throughput;
        scanf("%f", &new_throughput);

        while (pthread_mutex_trylock(
                   &comm.requestQueue->requests[free_request_slot_num].throughput_mutex) != 0)
            ;

        comm.requestQueue->requests[free_request_slot_num].throughput = new_throughput;
        pthread_mutex_unlock(&comm.requestQueue->requests[free_request_slot_num].throughput_mutex);
    }
}

int main(int argc, char** argv) {
    Arguments arguments = {0, 0, 0};  // Default values

    // Parse arguments
    argp_parse(&argp, argc, argv, 0, 0, &arguments);

    init_shm(false, comm, arguments.shm_location);

    // Sanitize args
    int prio = 0;
    if (arguments.prio >= NB_PRIORITIES) {
        prio = NB_PRIORITIES - 1;
        printf("Specified prio is higer than the max, using the max (%d)\n", prio);
    } else if (arguments.prio < 0) {
        prio = 0;
        printf("Specified prio is lower than 0, using 0\n");
    } else {
        prio = arguments.prio;
    }

    if (arguments.shutdown) {
        printf("Sending shutdown request to server...");
        shutdown_server();
        return 0;
    }

    pthread_t t_user_input;  // Declare a thread
    if (arguments.active_throughput) {
        pthread_create(&t_user_input, NULL, throughput_input, NULL);  // Create the thread
    }

    id = arguments.id;
    enum Lora lora = (enum Lora)arguments.lora_number;

    int loop_forever = arguments.repeats == -1 ? 1 : 0;

    while (loop_forever || arguments.repeats > 0) {
        free_request_slot_num = find_free_shm_request();
        if (free_request_slot_num == -1) {
            continue;
        }

        arguments.repeats--;

        const char* prompt = prompts[id % 10];
        const int n_chars_to_gen = 2048;

        printf("Sending the following request:\n"
               "-> %s\n",
               prompt);
        fflush(stdout);

        write_to_shm(comm.requestQueue, prompt, n_chars_to_gen, free_request_slot_num, prio, lora,
                     arguments.throughput);

        // Notify the server that there is a new request
        sem_post(&comm.requestQueue->requests[free_request_slot_num].serverNotifier);
        sem_post(&comm.requestQueue->active_reqs);

        // This will block until the server notifies that the request is completed
        print_from_shm(comm.requestQueue, strlen(prompt), n_chars_to_gen, free_request_slot_num);

        free_request(free_request_slot_num);

        if (arguments.sleep_time > 0) {
            sleep(arguments.sleep_time);
        }
    }

    if (arguments.active_throughput) {
        pthread_exit(&t_user_input);
    }

    return 0;
}
