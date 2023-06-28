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