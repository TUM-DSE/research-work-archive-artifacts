#pragma once

#include <pthread.h>
#include <semaphore.h>

#include <string>

#define SHM_LOCATION "/sys/bus/pci/devices/0000:00:12.0/resource2"
#define SHM_SIZE (32 * 1024 * 1024)
#define MAX_REQUESTS 512
#define MAX_TEXT 8196
#define NB_PRIORITIES 3

enum Lora { NO_LORA, SQL, FOOD, SIZE };

struct Request {
    pthread_mutex_t mutex;
    pthread_mutex_t throughput_mutex;
    sem_t clientNotifier;
    sem_t serverNotifier;
    int id;
    int n_chars_to_gen;
    char text[MAX_TEXT];
    int prio;
    enum Lora lora;
    float throughput;
};

struct SharedMemory {
    struct Request requests[MAX_REQUESTS];
    sem_t active_reqs;
    sem_t run;  // server will run while this is larger than 0
};

struct Communication {
    struct SharedMemory* requestQueue;
};

int init_shm(bool create, Communication& comm, std::string optional_shm_location);
int clean_shm(struct SharedMemory* shm);
