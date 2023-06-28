
No C foi mais simples:

Comando para compilação do nosso programa:
```bash
gcc -o c_program c_program.c -L/home/hugocardante/JHShadow/build/src/target/release -l:libzermia_lib.so
```

Em que passamos o path para a biblioteca no target, e depois passamos -l:nome_da_biblioteca.


Para executar o nosso programa temos de dizer onde está a shared library:
```bash
export LD_LIBRARY_PATH=/home/hugocardante/JHShadow/build/src/target/release:$LD_LIBRARY_PATH
./c_program
```

_________________________________________________________________________________


Header file (libc_zermia.h):
```c
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
```

No programa.c:

```c
// Incluímos o header
#include "libc_zermia.h"

// E chamamos a função:
bool result = send_message(1, 2, 3, "hello world from C", true);
```