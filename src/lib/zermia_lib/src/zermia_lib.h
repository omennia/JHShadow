//
// Created by joaosoares on 13-04-2023.
//

#ifndef ZERMIA_LIB_H
#define ZERMIA_LIB_H


#ifdef __cplusplus
extern "C" {
#endif

#include <netinet/in.h>
#include <stdbool.h>

typedef struct {
    uint32_t code;
    uint32_t ip_src;
    uint32_t ip_dest;
    uint8_t msg[512];
} Message;



int my_rust_function(int a, int b);
void start_tracker(const char* hostname);
void end_tracker(const char* hostname);
void new_socket(const char* hostname);
void connect_to_peer(const char* hostname, in_addr_t ip, in_port_t port);
void send_data(const char* hostname);
void start_client(char *addr, char *port, char *msg);

bool send_zermia_message(Message msg);
Message new_message();


#ifdef __cplusplus
}
#endif


#endif // ZERMIA_LIB_H
