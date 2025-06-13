#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>
#include <sys/socket.h>
#include <sys/un.h>
#include <assert.h>
#include <fcntl.h>
#include <sys/types.h>
#include <sys/mman.h>
#include "shm.h"

#define QMP_SOCKET_SERVER_PATH "/tmp/qmp-socket-server"
#define QMP_SOCKET_CLIENT_PATH "/tmp/qmp-socket-client"
#define BUFFER_SIZE 1024

const char *qmp_command_del = "{ \"execute\": \"device_del \", \"arguments\": {\"nvidia_gpu\"} }\n";
const char *qmp_command_add = "{ \"execute\": \"device_add \", \"arguments\": {\"vfio-pci\",\"host=ca:00.0\",\"id=nvidia_gpu\"} }\n";

static long round_up(long n, long mult)
{
    return ((n + mult - 1) / mult) * mult;
}

void send_qmp_command(int sock, const char *command) {
    if (write(sock, command, strlen(command)) < 0) {
        perror("write");
        exit(EXIT_FAILURE);
    }
}

void read_qmp_response(int sock) {
    char buffer[BUFFER_SIZE];
    ssize_t bytes_read;

    bytes_read = read(sock, buffer, sizeof(buffer) - 1);
    if (bytes_read > 0) {
        buffer[bytes_read] = '\0';
        printf("Response:\n%s\n", buffer);
    } else {
        perror("read");
        exit(EXIT_FAILURE);
    }
}

struct Communication comm;

int init_shm(){
    int fd = open("/dev/shm/shm1", O_RDWR);
    assert(fd != -1);

    long pagesize = sysconf(_SC_PAGESIZE);
    long shm_size = round_up(sizeof(struct SharedMemory), pagesize);
    if (ftruncate(fd, shm_size) == -1) {
        perror("ftruncate");
        return -1;
    }

    comm.requestQueue = (SharedMemory*) mmap(0, shm_size, PROT_READ | PROT_WRITE, MAP_SHARED, fd, 0);
    assert(comm.requestQueue != NULL);
    assert(comm.requestQueue != MAP_FAILED);

    close(fd);

    return 0;
}

int main(void){
    init_shm();
    int sock_server, sock_client;
    struct sockaddr_un addr_server, addr_client;

    // Create a Unix domain socket
    if ((sock_server = socket(AF_UNIX, SOCK_STREAM, 0)) == -1) {
        perror("socket");
        exit(EXIT_FAILURE);
    }
    if ((sock_client = socket(AF_UNIX, SOCK_STREAM, 0)) == -1) {
        perror("socket");
        exit(EXIT_FAILURE);
    }

    // Set up the socket address structure
    memset(&addr_server, 0, sizeof(addr_server));
    addr_server.sun_family = AF_UNIX;
    strncpy(addr_server.sun_path, QMP_SOCKET_SERVER_PATH, sizeof(addr_server.sun_path) - 1);
    
    memset(&addr_client, 0, sizeof(addr_client));
    addr_client.sun_family = AF_UNIX;
    strncpy(addr_client.sun_path, QMP_SOCKET_CLIENT_PATH, sizeof(addr_client.sun_path) - 1);

    // Connect to QEMU's QMP socket
    if (connect(sock_server, (struct sockaddr *)&addr_server, sizeof(addr_server)) == -1) {
        perror("connect");
        exit(EXIT_FAILURE);
    }
    if (connect(sock_client, (struct sockaddr *)&addr_client, sizeof(addr_client)) == -1) {
        perror("connect");
        exit(EXIT_FAILURE);
    }

    // Read QMP greeting message
    read_qmp_response(sock_server);
    read_qmp_response(sock_client);

    // Enable QMP capabilities
    const char *qmp_capabilities = "{ \"execute\": \"qmp_capabilities\" }\n";
    send_qmp_command(sock_server, qmp_capabilities);
    read_qmp_response(sock_server);
    send_qmp_command(sock_client, qmp_capabilities);
    read_qmp_response(sock_client);
    
    while(true){
        if(sem_trywait(&comm.requestQueue->control.hypervisor_switch_notification) != 0){
            continue;
        }
        printf("Hypervisor received GPU request...\n");
        switch(comm.requestQueue->control.target){
            case HOST:
                // device_del the GPU from the LLMOS
                send_qmp_command(sock_server, qmp_command_del);
                read_qmp_response(sock_server);
                // device_add the GPU to client
                send_qmp_command(sock_client, qmp_command_add);
                read_qmp_response(sock_client);
                break;
            default:
                break;
        };
        sem_post(&comm.requestQueue->control.switch_complete);
    }
    close(sock_server);
    close(sock_client);
}