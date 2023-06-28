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

    # print the received messag, flush=Truee
    # here it would be a reverse of sent message
    print('Received from the server :',str(data.decode('ascii')), flush=True)

    date = datetime.datetime.now()
    message = "My second message " + str(date)

    # message sent to server
    s.send(message.encode('ascii'))

    # message received from server
    data = s.recv(1024)

    # print the received messag, flush=Truee
    # here it would be a reverse of sent message
    print('Received from the server :',str(data.decode('ascii')), flush=True)



    date = datetime.datetime.now()
    message = "My third message " + str(date)

    # message sent to server
    s.send(message.encode('ascii'))

    # message received from server
    data = s.recv(1024)

    # print the received messag, flush=Truee
    # here it would be a reverse of sent message
    print('Received from the server :',str(data.decode('ascii')), flush=True)

    s.close()


if __name__ == '__main__':
    print("Starting client", flush=True)
    if len(sys.argv) < 2:
        print("Invalid number of arguments", flush=True)
        print("Usage: python client.py server_IP server_port", flush=True)
        exit(0)

    server_ip = sys.argv[1]
    server_port = int(sys.argv[2])
    print((server_ip, server_port), flush=True)

    start_client(server_ip, server_port)
    print("Ending client", flush=True)
