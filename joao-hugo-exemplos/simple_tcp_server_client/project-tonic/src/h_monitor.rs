use std::env;

use tonic::{transport::Server, Request, Response, Status};

use config::monitor_server::{Monitor, MonitorServer};
use config::{HelloReply, HelloRequest};
use std::net::{Ipv4Addr, SocketAddrV4, TcpStream};
use std::io::{Write, Read};


pub mod config {
    tonic::include_proto!("config");
}

static mut COUNT: u64 = 0;

#[derive(Debug, Default)]
pub struct MyMonitor {}

#[tonic::async_trait]
impl Monitor for MyMonitor {
    async fn contact(&self, request: Request<HelloRequest>, ) -> Result<Response<HelloReply>, Status> {
        // println!("Got a request: {:?}", request);
        unsafe {
            COUNT += 1;
            println!("Número de clientes que se connectaram até agora: {}", COUNT);
        }

        let reply = config::HelloReply {
            message: format!("Hello {}!", request.into_inner().name),
        };
        

        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

  let socket_addr = SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 9999); // change the IP address and port to the destination host
  let mut stream = TcpStream::connect(socket_addr).expect("connection failed");
  stream.write_all(b"Hello, world!").expect("write failed");
  let mut buf = [0; 1024];
  let n = stream.read(&mut buf).expect("read failed");
  println!("Received {} bytes: {:?}", n, &buf[..n]);


    if let Some(arg1) = env::args().nth(1) {
        let address: String = arg1.parse().unwrap();
        let addr = format!("{}:9999", address).parse()?;
        println!("A iniciar o monitor no endereço {}", addr);
        let monitor = MyMonitor::default();

        Server::builder()
            .add_service(MonitorServer::new(monitor))
            .serve(addr)
            .await?;

    } else {
        println!("Para correr o servidor, passamos o IP como argumento");
    }
    Ok(())
}
