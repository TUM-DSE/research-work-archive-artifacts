// t.c
#include <stdio.h>
#include <stdlib.h>
#include <unistd.h>
#include <fcntl.h>
#include <sys/mman.h>
#include <assert.h>
#include <curl/curl.h>
#include <string.h>
#include <sys/mman.h>
#include "shm.h"

#define SHM_SIZE (32 * 1024 * 1024)

#define MIN(a,b) (((a)<(b))?(a):(b))
#define MAX(a,b) (((a)>(b))?(a):(b))

struct Communication comm;

static size_t
WriteMemoryCallback(void *contents, size_t size, size_t nmemb, void *userp)
{
  size_t realsize = size * nmemb;
  struct Request *result = (struct Request *)userp;
 
  memcpy(result->text, contents, MIN(realsize, 512));
 
  return realsize;
}

static long round_up(long n, long mult)
{
    return ((n + mult - 1) / mult) * mult;
}

int init_shm(){
    int fd = open("/sys/bus/pci/devices/0000:00:10.0/resource2", O_RDWR);
    assert(fd != -1);
    
    long pagesize = sysconf(_SC_PAGESIZE);
    long shm_size = round_up(sizeof(struct SharedMemory), pagesize);
    if (ftruncate(fd, shm_size) == -1) {
        perror("ftruncate");
        return -1;
    }

    comm.requestQueue = mmap(0, shm_size, PROT_READ | PROT_WRITE, MAP_SHARED, fd, 0);
    assert(comm.requestQueue != NULL);
    assert(comm.requestQueue != MAP_FAILED);

    sem_destroy(&comm.requestQueue->semaphore);
    close(fd);
    
    memset(comm.requestQueue, 0, shm_size);
    if (sem_init(&comm.requestQueue->semaphore, 1, 0) != 0) {
        perror("sem_init failed");
        exit(EXIT_FAILURE);
    }
    
    fd = open("/sys/bus/pci/devices/0000:00:11.0/resource2", O_RDWR);
    assert(fd != -1);
    
    pagesize = sysconf(_SC_PAGESIZE);
    shm_size = round_up(sizeof(struct SharedMemory), pagesize);
    if (ftruncate(fd, shm_size) == -1) {
        perror("ftruncate");
        return -1;
    }

    comm.resultQueue = mmap(0, shm_size, PROT_READ | PROT_WRITE, MAP_SHARED, fd, 0);
    assert(comm.resultQueue != NULL);
    assert(comm.resultQueue != MAP_FAILED);

    sem_destroy(&comm.resultQueue->semaphore);
    close(fd);
    
    memset(comm.resultQueue, 0, shm_size);
    if (sem_init(&comm.resultQueue->semaphore, 1, 0) != 0) {
        perror("sem_init failed");
        exit(EXIT_FAILURE);
    }
    return 0;
}

int make_post_call(struct Request* request, struct Request* result){
    CURL *curl;
    CURLcode res;

    /* In Windows, this inits the Winsock stuff */
    curl_global_init(CURL_GLOBAL_ALL);

    /* get a curl handle */
    curl = curl_easy_init();
    if(curl) {
        /* First set the URL that is about to receive our POST. This URL can
         just as well be an https:// URL if that is what should receive the
         data. */
        curl_easy_setopt(curl, CURLOPT_URL, "http://localhost:8080/completion");
        // Set the POST request method
        curl_easy_setopt(curl, CURLOPT_POST, 1L);

        // Set the HTTP headers
        struct curl_slist *headers = NULL;
        headers = curl_slist_append(headers, "Content-Type: application/json");
        curl_easy_setopt(curl, CURLOPT_HTTPHEADER, headers);

        // Set the POST data
	char *data = malloc(1024 * sizeof(char));
	sprintf(data, "{\"prompt\": \"%s\",\"n_predict\": %d}", request->text, 10);
        curl_easy_setopt(curl, CURLOPT_POSTFIELDS, data);

        /* send all data to this function  */
        curl_easy_setopt(curl, CURLOPT_WRITEFUNCTION, WriteMemoryCallback);
 
        /* we pass our 'chunk' struct to the callback function */
        curl_easy_setopt(curl, CURLOPT_WRITEDATA, (void *)result);

        /* Perform the request, res gets the return code */
        res = curl_easy_perform(curl);
        /* Check for errors */
        if(res != CURLE_OK)
          fprintf(stderr, "curl_easy_perform() failed: %s\n",
                curl_easy_strerror(res));

        /* always cleanup */
	curl_slist_free_all(headers);
	free(data);
        curl_easy_cleanup(curl);
    }
    curl_global_cleanup();
    return 0;
}

int handle_request(struct Request* request, struct Request* result){
    printf("%s\n", request->text);
    fflush(stdout);
    
    make_post_call(request, result);
    //request->done = 1;
    return 0;
}

int write_result(struct Request* result){
    memcpy(comm.resultQueue->requests[comm.requestQueue->counter].text, result->text, 512);
    sem_post(&comm.resultQueue->semaphore);
    return 0;
}

int clean_shm(struct SharedMemory* shm){
    munmap(shm, SHM_SIZE);
    return 0;
}

int main(int argc, char **argv) {
    init_shm();

    struct Request* result = malloc(sizeof(struct Request));
    while(1){
        // Wait client writes
        if(sem_trywait(&comm.requestQueue->semaphore) == 0){
            printf("Recieved new request:\n\t%s\n", comm.requestQueue->requests[comm.requestQueue->counter].text);
            struct Request *request = &comm.requestQueue->requests[comm.requestQueue->counter];
	    handle_request(request, result);
	    write_result(result);
	}	
    }

    free(result);
    return 0;
}

