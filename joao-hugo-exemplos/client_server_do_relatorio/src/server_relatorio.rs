use std::io::{Error, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::env;



fn start_server(address: String) {
  let addr = format!("{}:{}", address, "8888");
  println!("A começar o servidor no endereço {}", addr);

  let listener = TcpListener::bind(addr).expect("Could not bind");

  for stream in listener.incoming() {
      match stream {
          Err(e) => println!("failed: {}", e),
          Ok(stream) => {
              println!(
                  "Conexão entre o servidor e o cliente estabelecida:\n\t {:?}\n",
                  stream
              );
              if let Err(e) = handle_client(stream) {
                  println!("Erro no ciclo de conexação [server]: {:?}", e);
                  // continue;
              };
          }
      }
  }
}



fn handle_client(mut stream: TcpStream) -> Result<(), Error> {
  println!("Conexão a chegar de: {}", stream.peer_addr()?);
  let mut buf = [0; 512];

  loop {
      let bytes_read = stream.read(&mut buf)?;
      let my_sentence = std::str::from_utf8(&buf[..bytes_read])
          .unwrap()
          .to_uppercase();
        
      if bytes_read == 0 {
          return Ok(());
      }
      println!("Vamos devolver esta mensagem: {}", my_sentence);
      // println!("Aqui vão os bytes read: {}", bytes_read);

      stream.write_all(my_sentence.as_bytes())?;
  }
}



fn main() {
    if let Some(arg1) = env::args().nth(1) {
        let server_address = arg1.parse().unwrap();
        start_server(server_address);
    } else {
        println!("Temos de dizer o IP do servidor");
    }
}
