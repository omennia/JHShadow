use std::io::{Error, Read, Write};
use std::net::TcpStream;
use std::env;

fn start_client(address: String) -> Result<(), Error> {
    let addr = format!("{}:{}", address, "8888");
    match TcpStream::connect(addr) {
        Ok(stream) => {
            println!("Conectado ao servidor");
            if let Err(e) = send_messages(stream) {
                println!("Erro: {:?}.", e);
            };
        }
        Err(e) => {
            println!("Não conseguimos conectar: {}", e);
        }
    }
    Ok(())
}

fn send_messages(mut stream: TcpStream) -> Result<(), Error> {
    let mut cnt = 1;

    while cnt <= 3 {
        let mut buf = [0; 512];
        let client_message = format!("mensagem {}", cnt);

        stream.write_all(client_message.as_bytes()).unwrap();

        println!("Enviamos: \"{}\"", client_message);

        let _bytes_read = stream.read(&mut buf);
        let server_response = std::str::from_utf8(&buf).unwrap();
        println!("O servidor respondeu: {}", server_response);

        cnt += 1;
    }
    Ok(())
}

fn main() {
    if let Some(arg1) = env::args().nth(1) {
        let address = arg1.parse().unwrap();
        println!("A começar o client...");
        match start_client(address) {
            Ok(_) => println!("OK!"),
            _ => println!("ERRO!"),
        }
    } else {
        println!("Precisamos especificar um endereço");
    }
}
