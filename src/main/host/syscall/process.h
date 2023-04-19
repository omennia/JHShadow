/*
 * The Shadow Simulator
 * See LICENSE for licensing information
 */

#ifndef MAIN_HOST_SYSCALL_PROCESS_H
#define MAIN_HOST_SYSCALL_PROCESS_H

#include "main/host/syscall/protected.h"

SYSCALL_HANDLER(execve);
SYSCALL_HANDLER(prctl);
SYSCALL_HANDLER(prlimit);
SYSCALL_HANDLER(prlimit64);

#endif
