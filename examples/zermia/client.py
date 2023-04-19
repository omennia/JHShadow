# Import socket module
import socket
import sys
import datetime

def start_client(server_ip, server_port):
    s = socket.socket(socket.AF_INET, socket.SOCK_STREAM)

    # connect to server on local computer
    s.connect((server_ip,server_port))

    date = datetime.datetime.now()
    message = "My first message " + str(date)

    # message sent to server
    s.send(message.encode('ascii'))

    # message received from server
    data = s.recv(1024)

    # print the received message
    # here it would be a reverse of sent message
    print('Received from the server :',str(data.decode('ascii')))

    date = datetime.datetime.now()
    message = "My second message " + str(date)

    # message sent to server
    s.send(message.encode('ascii'))

    # message received from server
    data = s.recv(1024)

    # print the received message
    # here it would be a reverse of sent message
    print('Received from the server :',str(data.decode('ascii')))

    s.close()


if __name__ == '__main__':
    print("Starting client")
    if len(sys.argv) < 2:
        print("Invalid number of arguments")
        print("Usage: python client.py server_IP server_port")
        exit(0)

    server_ip = sys.argv[1]
    server_port = int(sys.argv[2])
    print((server_ip, server_port))

    start_client(server_ip, server_port)
    print("Ending client")
