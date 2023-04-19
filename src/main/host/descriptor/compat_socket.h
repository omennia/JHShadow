/*
 * The Shadow Simulator
 * See LICENSE for licensing information
 */

#ifndef SRC_MAIN_HOST_DESCRIPTOR_COMPAT_SOCKET_H_
#define SRC_MAIN_HOST_DESCRIPTOR_COMPAT_SOCKET_H_

typedef enum _CompatSocketTypes CompatSocketTypes;
typedef union _CompatSocketObject CompatSocketObject;
typedef struct _CompatSocket CompatSocket;

#include "main/bindings/c/bindings-opaque.h"
#include "main/host/descriptor/socket.h"
#include "main/utility/tagged_ptr.h"

enum _CompatSocketTypes {
    CST_NONE,
    CST_LEGACY_SOCKET,
    CST_INET_SOCKET,
};

union _CompatSocketObject {
    LegacySocket* as_legacy_socket;
    const InetSocket* as_inet_socket;
};

struct _CompatSocket {
    CompatSocketTypes type;
    CompatSocketObject object;
};

CompatSocket compatsocket_fromLegacySocket(LegacySocket* socket);
CompatSocket compatsocket_fromInetSocket(const InetSocket* socket);

/* reference counting */
CompatSocket compatsocket_refAs(const CompatSocket* socket);
void compatsocket_unref(const CompatSocket* socket);

/* handle to the socket object */
uintptr_t compatsocket_getCanonicalHandle(const CompatSocket* socket);

/* converting between a CompatSocket and a tagged pointer */
uintptr_t compatsocket_toTagged(const CompatSocket* socket);
CompatSocket compatsocket_fromTagged(uintptr_t ptr);

/* compatability wrappers */
const Packet* compatsocket_peekNextOutPacket(const CompatSocket* socket);
void compatsocket_pushInPacket(const CompatSocket* socket, const Host* host, Packet* packet);
Packet* compatsocket_pullOutPacket(const CompatSocket* socket, const Host* host);
void compatsocket_updatePacketHeader(const CompatSocket* socket, const Host* host, Packet* packet);

#endif /* SRC_MAIN_HOST_DESCRIPTOR_COMPAT_SOCKET_H_ */
