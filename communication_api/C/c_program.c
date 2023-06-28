#include "libc_zermia.h"
#include <stdio.h>

int main() {
  bool result = send_message(1, 2, 3, "hello world from C", true);
  if(result){
    printf("Recebemos true");
  }
  else{
    printf("Recebemos false");
  }
}