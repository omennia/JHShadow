use ipc_rs::IpcError;
use ipc_rs::MessageQueue;
use serde::{Deserialize, Serialize};
// use std::collections::HashMap;

// Representa uma struct para a mensagem
#[repr(C)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Message {
    pub code: u32,
    pub ip_src: u32,
    pub ip_dest: u32,
    pub msg: [u8; 32],
    pub return_status: bool,
}


const SYSCALL_SENDTO: u32 = 44;
const SYSCALL_RCVFROM: u32 = 45;
const CORRUPT_DATA_PACKET: u32 = 88883;
static mut CNT: u32 = 0;
// static mut BREAKIT: bool = false;

fn print_message(message: Message) {
    println!("=============================================================");
    let ip_v4_src = std::net::IpAddr::V4(u32::from_be(message.ip_src).into());
    let ip_v4_dst = std::net::IpAddr::V4(u32::from_be(message.ip_dest).into());

    match std::str::from_utf8(&message.msg) {
        // Imprimir a struct
        Ok(descricao) => {
            print!(
                "Código da mensagem: {}\nPayload: {}\nSource: {}\nDest: {}\n",
                message.code, descricao, ip_v4_src, ip_v4_dst
            )
        }
        Err(error) => {
            eprintln!("Um erro a descodificar a mensagem. Error: {:?}", error);
            let result = String::from_utf8_lossy(&message.msg).into_owned();
            print!("Mas a mensagem é algo como:\nCódigo da mensagem: {}\nPayload: {}\nSource: {}\nDest: {}\n", message.code, result, ip_v4_src, ip_v4_dst);
        }
    }
    println!("=============================================================");
}


fn treat_call(message: &Message) -> i32 { // Usamos para dont_receive

  match message.code { // Neste momento está configurado para repetir a mensagem dois
                       // para o exemplo do relatório. Mudar estes parámetros replica as experiencias.
    SYSCALL_SENDTO => {
      println!("sendto");
      unsafe {
        CNT += 1;
      // //   // println!("Call to sendto num: {}", CNT);
        if CNT == 3 {
          // BREAKIT = true;
          return 6;
        }
      }
    }

    SYSCALL_RCVFROM => {
      println!("rcvfrom");
      // unsafe {
        // CNT += 1;
        // if BREAKIT {
        //   BREAKIT = false;
        //   return 3;
        // }
      // }
        // println!("Received a recvfrom");
    }


    CORRUPT_DATA_PACKET => { // pacote de dados
      println!("Data packet sent..");
      // println!("Corrupt packet????");
      // unsafe{
      //   if CNT == 3 || CNT == 4 {
      //     println!("Vamos retornar 5 agora...");
      //     return 5; // Queremos corromper
      //     }
      //   }
    }

    _ => {

    }
  }
  return 0;
}

fn main() -> Result<(), IpcError> {
    // let mut my_map: HashMap<u32, u32> = HashMap::new();
    let send_key = 1234;
    let response_key = 4321;

    let queue = MessageQueue::<Message>::new(send_key).create().init()?;     // Queue de entrada
    let response_queue = MessageQueue::<i32>::new(response_key).create().init()?;  // Queue de saída

    loop {
        let message = queue.recv().expect("Failed to receive a message");
        // let m_ret_type = message.code as i64;
        print_message(message.clone());

        let ret_val: i32 = treat_call(&message);
        // println!("O ret_val para a chamada {} é {}", message.code, ret_val);

        let _ = response_queue.send(ret_val, 1);
        
        if message.code == 9999 {
            println!("A terminar a simulação.. recebemos o código 9999");
            break;
        }
    }

    Ok(())
}
