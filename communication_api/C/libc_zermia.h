#ifndef LIBZERMIA_H
#define LIBZERMIA_H
#define MIN(x, y) (((x) < (y)) ? (x) : (y))

#include <stdbool.h>
#include <stdint.h>
#include <string.h>

typedef struct {
    uint32_t code;
    uint32_t ip_src;
    uint32_t ip_dest;
    uint8_t msg[32];
    bool return_status;
} Message;

bool send_zermia_message(Message message);

bool send_message(uint32_t code, uint32_t ip_src, uint32_t ip_dest, char *msg, bool return_status){
  Message message;

  message.code = code;
  message.ip_src = ip_src;
  message.ip_dest = ip_dest;
  memcpy(message.msg, msg, MIN(strlen(msg) + 1, 31));
  message.return_status = return_status;

  return send_zermia_message(message);
}


#endif
