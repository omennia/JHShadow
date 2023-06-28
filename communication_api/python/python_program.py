from pylib import send_message_to_receiver

result = send_message_to_receiver(32, 12, 13, "hello world from python".encode(), False)
print(f'O resultado da chamada à função é: {result}')