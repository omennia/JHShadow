A nossa biblioteca em python é:

```py
from ctypes import cdll, Structure, c_uint, c_ubyte, c_bool, byref

class Message(Structure):
	_fields_ = [
		('code', c_uint),
		('ip_src', c_uint),
		('ip_dest', c_uint),
		('msg', c_ubyte * 32),
		('return_status', c_bool)
	]

zermia_lib = cdll.LoadLibrary('/home/hugocardante/JHShadow/build/src/target/release/libzermia_lib.so')

def send_message_to_receiver(code: int, ip_src: int, ip_dest: int, msg_bytes: bytes, return_status: bool):

	zermia_lib = cdll.LoadLibrary('/home/hugocardante/JHShadow/build/src/target/release/libzermia_lib.so')
	
	new_message = zermia_lib.new_message
	new_message.restype = Message
	
	zermia_lib.send_zermia_message.argtypes = [Message]
	zermia_lib.send_zermia_message.restype = c_bool
	
	  
	msg = new_message()
	msg.code = code
	msg.ip_src = ip_src
	msg.ip_dest = ip_dest
	msg.msg = (c_ubyte * 32)(*msg_bytes)
	msg.return_status = return_status
	
	return zermia_lib.send_zermia_message(msg)
```

Para termos esta biblioteca foi necessário compilar a biblioteca de rust, dinamicamente, e é necessário dar ao python o path para o nosso zermia_lib.so.

Este ficheiro, depois de compilar o shadow, fica no build/src/target/release/libzermia_lib.so



Depois no nosso programa em python:
```py
from pylib import send_message_to_receiver

result = send_message_to_receiver(32, 12, 13, "hello world from python".encode(), False)
print(f'O resultado da chamada à função é: {result}')
```