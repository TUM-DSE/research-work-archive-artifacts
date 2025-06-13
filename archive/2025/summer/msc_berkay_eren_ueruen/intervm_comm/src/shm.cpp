#include "shm.h"

#include <assert.h>
#include <fcntl.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/mman.h>
#include <unistd.h>

#include "misc.h"

int init_shm(bool create, Communication &comm, std::string optional_shm_location) {
    int fd;
    if (!optional_shm_location.empty()) {
        fd = open(optional_shm_location.c_str(), O_RDWR);
        assert(fd != -1);
    } else {
        fd = open(SHM_LOCATION, O_RDWR);
        assert(fd != -1);
    }

    long pagesize = sysconf(_SC_PAGESIZE);
    long shm_size = round_up(sizeof(struct SharedMemory), pagesize);
    if (ftruncate(fd, shm_size) == -1) {
        perror("ftruncate");
        return -1;
    }

    comm.requestQueue =
        (SharedMemory *)mmap(0, shm_size, PROT_READ | PROT_WRITE, MAP_SHARED, fd, 0);
    assert(comm.requestQueue != NULL);
    assert(comm.requestQueue != MAP_FAILED);

    close(fd);

    if (!create) {
        // only connect to shared memory, do not set up semaphores and mutextes
        return 0;
    }

    memset(comm.requestQueue, 0, shm_size);

    // Init mutexes
    pthread_mutexattr_t attr;
    pthread_mutexattr_init(&attr);
    pthread_mutexattr_setpshared(&attr, PTHREAD_PROCESS_SHARED);

    sem_destroy(&comm.requestQueue->active_reqs);
    if (sem_init(&comm.requestQueue->active_reqs, 1, 0) != 0) {
        perror("sem_init failed");
        exit(EXIT_FAILURE);
    }

    sem_destroy(&comm.requestQueue->run);
    if (sem_init(&comm.requestQueue->run, 1, 1) != 0) {  // init to 1 since server will start
        perror("sem_init failed");
        exit(EXIT_FAILURE);
    }

    for (int i = 0; i < MAX_REQUESTS; i++) {
        pthread_mutex_init(&comm.requestQueue->requests[i].mutex, &attr);
        sem_destroy(&comm.requestQueue->requests[i].clientNotifier);
        if (sem_init(&comm.requestQueue->requests[i].clientNotifier, 1, 0) != 0) {
            perror("sem_init failed");
            exit(EXIT_FAILURE);
        }
        sem_destroy(&comm.requestQueue->requests[i].serverNotifier);
        if (sem_init(&comm.requestQueue->requests[i].serverNotifier, 1, 0) != 0) {
            perror("sem_init failed");
            exit(EXIT_FAILURE);
        }
    }

    return 0;
}

int clean_shm(struct SharedMemory *shm) {
    munmap(shm, SHM_SIZE);
    return 0;
}
