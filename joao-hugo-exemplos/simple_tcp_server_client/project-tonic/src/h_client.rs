// Client
extern crate zermia_lib;
use std::env;
// use std::ffi::CString;
use std::io::{Error, Read, Write};
use std::net::TcpStream;
use std::sync::Mutex;
use std::cmp;
// use std::{thread, time};
use zermia_lib::message::Message;
use zermia_lib::send_zermia_message;

fn start_client(address: String) -> Result<(), Error> {
    let addr = format!("{}:{}", address, "8888");
    match TcpStream::connect(addr) {
        Ok(stream) => {
            println!("Successfully connected to server in {}:8888", address);
            if let Err(e) = handle_response(stream) {
                println!("Error {:?} has occurred.", e);
            };
        }
        Err(e) => {
            println!("Failed to connect: {}", e);
        }
    }
    println!("Terminated.");
    Ok(())
}

fn handle_response(mut stream: TcpStream) -> Result<(), Error> {
    let mut cnt = 1;

    while cnt < 10 {
        let mut buf = [0; 512];
        let client_message = format!("{} mensagem num {}", "shadow simulator", cnt);

        stream.write_all(client_message.as_bytes()).unwrap();

        // ESTOU A PENSAR NESTE BLOCO COMO SE FOSSE UM ASPETO
        {
            let mut msg = Message {
                code: 1023412, // 1 para aplication
                ip_src:94,
                ip_dest: 223,
                return_status: false,
                msg: [0; 32], // Initialize the byte array with zeros
            };

            let text_bytes = client_message.as_bytes();
            msg.msg[..cmp::min(32, text_bytes.len())].copy_from_slice(&text_bytes[0..cmp::min(32, text_bytes.len())]);
            let res = send_zermia_message(msg);
            

        //     if !res {
        //       println!("Some error has occurred!!!");
        //     }
        }

        println!("Sent message \"{}\", awaiting reply...", client_message);

        let _bytes_read = stream.read(&mut buf);
        let server_response = std::str::from_utf8(&buf).unwrap();
        println!("Server responded with: {}", server_response);

        cnt += 1;
    }

    Ok(())
}

fn main() { 
    if let Some(arg1) = env::args().nth(1) {
        let address = arg1.parse().unwrap();
        println!("Starting a client...");
        match start_client(address) {
            Ok(_) => println!("OK!"),
            _ => println!("ERRRO"),
        }
    } else {
        println!("Cannot run sever, must provide a port number");
    }
}
