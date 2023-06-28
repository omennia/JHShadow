# import socket programming library
import socket
import sys
# import thread module
from _thread import *
from threading import Thread
from pylib import send_message_to_receiver


# thread function
def threaded(c):
    running = True
    while(running):
        try:
            # data received from client
            data = c.recv(1024)
            if len(data) == 0:
                break
            print("Received " + str(data), flush=True)
            # reverse the given string from client
            data = data[::-1]
            # send back reversed string to client
            c.send(data)
            # connection closed
        except Exception:
            continue
            #print(e)


def start_server(server_ip, server_port):
    s = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
    s.bind((server_ip, server_port))
    print("socket binded to port", server_port, flush=True)

    # put the socket into listening mode
    s.listen(6)
    print("socket is listening")

    # a forever loop until client wants to exit
    threads = []
    for i in range(5):
        # establish connection with client
        c, addr = s.accept()
        print('Connected to :', addr[0], ':', addr[1], flush=True)
        # Start a new thread and return its identifier
        t = Thread(target = threaded, args= (c,))
        threads.append(t)
        t.start()

    for t in threads:
        t.join()

    s.close()


if __name__ == '__main__':
    print("Starting server", flush=True)
    if len(sys.argv) < 2:
        print("Invalid number of arguments", flush=True)
        print("Usage: python server.py server_IP server_port", flush=True)
        exit(0)

    server_ip = sys.argv[1]
    server_port = int(sys.argv[2])
    print("Starting server", flush=True)
    print((server_ip, server_port))


    try:
      result = send_message_to_receiver(32, 12, 13, "Pyy2...".encode(), False)
      print(f'O resultado da chamada à função é: {result}')
    except Exception as e:
        print(e)

    start_server(server_ip, server_port)
    print("Ending server", flush=True)
