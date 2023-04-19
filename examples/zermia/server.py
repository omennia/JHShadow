# import socket programming library
import socket
import sys
# import thread module
from _thread import *
import threading

print_lock = threading.Lock()

# thread function
def threaded(c):
    for i in range(2):

        # data received from client
        data = c.recv(1024)
        print("Received " + str(data))
        # reverse the given string from client
        data = data[::-1]

        # send back reversed string to client
        c.send(data)

    # connection closed
    c.close()


def start_server(server_ip, server_port):
    s = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
    s.bind((server_ip, server_port))
    print("socket binded to port", server_port)

    # put the socket into listening mode
    s.listen(5)
    print("socket is listening")

    # a forever loop until client wants to exit
    for i in range(3):
        # establish connection with client
        c, addr = s.accept()

        print('Connected to :', addr[0], ':', addr[1])

        # Start a new thread and return its identifier
        start_new_thread(threaded, (c,))
    s.close()


if __name__ == '__main__':
    print("Starting server")
    if len(sys.argv) < 2:
        print("Invalid number of arguments")
        print("Usage: python server.py server_IP server_port")
        exit(0)

    server_ip = sys.argv[1]
    server_port = int(sys.argv[2])

    print((server_ip, server_port))

    start_server(server_ip, server_port)
    print("Ending server")
