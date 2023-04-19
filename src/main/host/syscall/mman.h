/*
 * The Shadow Simulator
 * See LICENSE for licensing information
 */

#ifndef SRC_MAIN_HOST_SYSCALL_MMAN_H_
#define SRC_MAIN_HOST_SYSCALL_MMAN_H_

#include "main/host/syscall/protected.h"

SYSCALL_HANDLER(brk);
SYSCALL_HANDLER(mmap);
SYSCALL_HANDLER(mprotect);
SYSCALL_HANDLER(mremap);
SYSCALL_HANDLER(munmap);

#endif /* SRC_MAIN_HOST_SYSCALL_MMAN_H_ */
