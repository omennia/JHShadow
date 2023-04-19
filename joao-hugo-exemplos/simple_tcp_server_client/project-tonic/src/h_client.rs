// Client

use std::env;
use std::io::{Error, Read, Write};
use std::net::TcpStream;
// use std::{thread, time};

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
  let mut buf = [0; 512];
  let client_message = "shadow simulator";

  stream.write_all(client_message.as_bytes()).unwrap();
  println!("Sent message \"{}\", awaiting reply...", client_message);

  let _bytes_read = stream.read(&mut buf);
  let server_response = std::str::from_utf8(&buf).unwrap();
  println!("Server responded with: {}", server_response);
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
