use std::{io::prelude::*, net::TcpStream};
use std::sync::atomic::{AtomicU32, Ordering};
use std::os::raw::{c_char/* , c_int */}; // FFI safety I guess
use std::ffi::CStr;

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