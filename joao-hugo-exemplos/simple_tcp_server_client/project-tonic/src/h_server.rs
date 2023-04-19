use config::monitor_client::MonitorClient;
use config::HelloRequest;
use std::env;
use std::io::{Error, Read, Write};
use std::net::{TcpListener, TcpStream};

pub mod config {
    tonic::include_proto!("config");
}

async fn start_server(address: String, monitor_address: &String) {
    let addr = format!("{}:{}", address, "8888");
    println!("A começar o servidor no endereço {}", addr);

    let listener = TcpListener::bind(addr).expect("Could not bind");

    for stream in listener.incoming() {
        match stream {
            Err(e) => println!("failed: {}", e),
            Ok(stream) => {
                println!("Conexão entre o servidor e o cliente estabelecida:\n\t {:?}\n", stream);
                if let Err(e) = handle_client(stream, monitor_address).await {
                    println!("{:?}", e);
                };
            }
        }
    }
    println!("Finished the whole process...");
}

async fn handle_client(mut stream: TcpStream, monitor_address: &String) -> Result<(), Error> {
    println!("Conexão a chegar de: {}", stream.peer_addr()?);
    if let Err(e) = connect_to_count(monitor_address).await {
        println!("Erro ao connectar com o monitor {:?}", e);
    };
    let mut buf = [0; 512];

    loop {
        let bytes_read = stream.read(&mut buf)?;
        let my_sentence = std::str::from_utf8(&buf[..bytes_read])
            .unwrap()
            .to_uppercase();
          
          println!("Vamos devolver esta mensagem: {}", my_sentence);
        // println!("Aqui vão os bytes read: {}", bytes_read);
        if bytes_read == 0 {
            return Ok(());
        }

        stream.write_all(my_sentence.as_bytes())?;
    }
}

async fn connect_to_count(monitor_address: &String) -> Result<(), Box<dyn std::error::Error>> {
  let addr = format!("http://{}:9999", monitor_address);
    println!("O address do monitor é: {}", addr);
    let mut client = MonitorClient::connect(addr).await?;
    let request = tonic::Request::new(HelloRequest {
        name: "Tonic".into(),
    });

    let _response = client.contact(request).await?;
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // default configuration
    if let Some(arg1) = env::args().nth(1) {
        let server_address = arg1.parse().unwrap();
        if let Some(arg2) = env::args().nth(2) {
            let monitor_address = arg2.parse().unwrap();
            start_server(server_address, &monitor_address).await;
        } else {
            println!("Temos de dizer o IP do servidor, seguido do IP do monitor");
        }
    } else {
        println!("Temos de dizer o IP do servidor, seguido do IP do monitor");
    }
    Ok(())
}