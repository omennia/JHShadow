//
// Created by joaosoares on 13-04-2023.
//

#ifndef ZERMIA_LIB_H
#define ZERMIA_LIB_H


#ifdef __cplusplus
extern "C" {
#endif

#include <netinet/in.h>

int my_rust_function(int a, int b);
void start_tracker(const char* hostname);
void end_tracker(const char* hostname);
void new_socket(const char* hostname);
void connect_to_peer(const char* hostname, in_addr_t ip, in_port_t port);
void send_data(const char* hostname);
void start_client(char *addr, char *port, char *msg);


#ifdef __cplusplus
}
#endif


#endif // ZERMIA_LIB_H
