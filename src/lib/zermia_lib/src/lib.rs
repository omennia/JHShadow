// use std::sync::Mutex;
use std::{io::prelude::*, net::TcpStream};
use std::sync::atomic::{AtomicU32, Ordering};
use std::os::raw::c_char; // FFI safety I guess
use std::ffi::CStr;
use ipc_rs::{MessageQueue, IpcError};
pub mod message;

#[no_mangle]
fn send_message_queue(message: message::Message) -> Result<i32, IpcError> {
  let send_key = 1234;
  let response_key = 4321;
  // let m_type = message.code as i64;

  let queue = MessageQueue::<message::Message>::new(send_key)
      .create()
      .init()?;

  let queue_for_response = MessageQueue::<i32>::new(response_key)
    .create()
    .init()?;
      

  queue.send(message, 42)
      .expect("failed to send a message");

  let should_send: i32 = queue_for_response.recv()?;

  Ok(should_send)
}


#[no_mangle]
pub extern "C" fn send_zermia_message(message: message::Message) -> i32 {
  let result = send_message_queue(message);

  match result{
    Ok(should_send) => return should_send,
    Err(_e) => return 0,
  }
}

#[no_mangle]
pub extern "C" fn new_message() -> message::Message {
    message::Message {
        code: 0,
        ip_src: 0,
        ip_dest: 0,
        msg: [0; 32],
        return_status: false,
    }
}


/* Versão compatível com java */
use jni::JNIEnv;
use jni::objects::JClass;
use jni::sys::{jboolean, jint, jbyteArray};

#[no_mangle]
pub extern "system" fn Java_RustInterface_sendMessage(
    env: JNIEnv,
    _class: JClass,
    code: jint,
    ip_src: jint,
    ip_dest: jint,
    msg: jbyteArray,
    return_status: jboolean,
) -> jint {

    let byte_vec = env.convert_byte_array(msg).expect("failed to convert message");
    let mut byte_arr = [0u8; 32];
    for (i, byte) in byte_vec.into_iter().enumerate() {
        byte_arr[i] = byte;
    }

    let message = message::Message {
        code: code as u32,
        ip_src: ip_src as u32,
        ip_dest: ip_dest as u32,
        msg: byte_arr,
        return_status: return_status == jni::sys::JNI_TRUE,
    };

    match send_message_queue(message) {
        Ok(should_send) => should_send,
        Err(_e) => 0,
    }
}

/* Versão compatível com java */


















// Algumas funções que usamos para explorar..

static mut N:i32 = 0;
static TRACKER_COUNTER: AtomicU32 = AtomicU32::new(0);
static SOCKET_COUNTER: AtomicU32 = AtomicU32::new(0);


// Mock function.. it seems to be working
#[no_mangle]
pub extern "C" fn my_rust_function(a: i32, b: i32) -> i32 {
    a + b
}

#[no_mangle]
pub extern "C" fn start_tracker(hostname: *const c_char) {
    println!("[Lib Zermia] start tracker: {}", unsafe { CStr::from_ptr(hostname).to_string_lossy().into_owned()});
    unsafe {
        N += 1;
        println!("N: {}", N);
    }
    let crt = TRACKER_COUNTER.load(Ordering::Relaxed)+1;
    TRACKER_COUNTER.store(crt, Ordering::Relaxed);
    println!("TRACKER_COUNTER: {}", crt);
}

#[no_mangle]
pub extern "C" fn end_tracker(hostname: *const c_char) {
    println!("[Lib Zermia] end tracker: {}", unsafe { CStr::from_ptr(hostname).to_string_lossy().into_owned()});
}

#[no_mangle]
pub extern "C" fn new_socket(hostname: *const c_char) {
    println!("[Lib Zermia] new socket: {}", unsafe { CStr::from_ptr(hostname).to_string_lossy().into_owned()});
    unsafe {
        N += 1;
        println!("N: {}", N);
    }
    let crt = SOCKET_COUNTER.load(Ordering::Relaxed)+1;
    SOCKET_COUNTER.store(crt, Ordering::Relaxed);
    println!("SOCKET_COUNTER: {}", crt);
}

#[no_mangle]
pub extern "C" fn send_data(hostname: *const c_char) {
    println!("[Lib Zermia] send data: {}", unsafe { CStr::from_ptr(hostname).to_string_lossy().into_owned()});
    unsafe {
        N += 1;
        println!("N: {}", N);
    }
    let crt = SOCKET_COUNTER.load(Ordering::Relaxed)+1;
    SOCKET_COUNTER.store(crt, Ordering::Relaxed);
    println!("SOCKET_COUNTER: {}", crt);
}

#[no_mangle]
pub extern "C" fn start_client(addr: *const c_char, prt: *const c_char, msg: *const c_char) {
    let address = unsafe { CStr::from_ptr(addr).to_string_lossy().into_owned() };
    let port = unsafe { CStr::from_ptr(prt).to_string_lossy().into_owned() };
    let addr = format!("{}:{}", address, port);
    match TcpStream::connect(addr) {
        Ok(stream) => {
            println!("Successfully connected to server in port {}", port);
            handle_response(stream, msg);
        }
        Err(e) => {
            println!("Failed to connect: {}", e);
        }
    }
    println!("Terminated.");
}

// Does not need to be in the my_rust_lib.h
#[no_mangle]
fn handle_response(mut stream: TcpStream, msg: *const c_char) {
    let mut buf = [0; 512];
    let client_message = unsafe { CStr::from_ptr(msg).to_string_lossy().into_owned() };

    stream.write_all(client_message.as_bytes()).unwrap();
    println!("Sent message \"{}\", awaiting reply...", client_message);

    let _bytes_read = stream.read(&mut buf);
    let server_response = std::str::from_utf8(&buf).unwrap();
    println!("Server responded with: {}", server_response);
}