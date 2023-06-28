# Import socket module
import socket
import sys
import datetime

from ctypes import *
from pylib import send_message_to_receiver

# so_file = "/home/shadow-starter/.local/lib/libzermia_lib.so"
# my_functions = CDLL(so_file)

# print(type(my_functions))
# print(my_functions.my_rust_function(10, 20))


def connect_to_server(server_ip, server_port):
    s = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
    # connect to server on local computer
    s.connect((server_ip,server_port))
    return s


def send_message(s, message):
        date = datetime.datetime.now()
        message = str(message) + ": " + str(date)
        # message sent to server
        s.send(message.encode('ascii'))
        # message received from server
        data = s.recv(1024)
        # print the received message
        # here it would be a reverse of sent message
        print('Received from the server :', str(data.decode('ascii')), flush=True)

if __name__ == '__main__':
    print("Starting client", flush=True)
    if len(sys.argv) < 2:
        print("Invalid number of arguments", flush=True)
        print("Usage: python client.py server_IP server_port", flush=True)
        exit(0)

    server_ip = sys.argv[1]
    server_port = int(sys.argv[2])
    print("Connecting to server", flush=True)
    print((server_ip, server_port))
    s = connect_to_server(server_ip, server_port)

    try:
      result = send_message_to_receiver(98356, 0, 0, "PyMonitor".encode(), False)
      print(f'O resultado da chamada à função é: {result}')
    except Exception as e:
        print(e)

    try:
        send_message(s, "My first message")
    except Exception as e:
        print(e)

    try:
        send_message(s, "My second message")
    except Exception as e:
        print(e)

    try:
        send_message(s, "My third message")
    except Exception as e:
        print(e)

    try:
        send_message(s, "My fourth message")
    except Exception as e:
        print(e)

    print("Disconnecting from server", flush=True)
    s.close()

    print("Ending client", flush=True)
