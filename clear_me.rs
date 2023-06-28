use colored::*;
use libc::shm_open;
use libc::{ftruncate, mmap, MAP_FAILED, MAP_SHARED, PROT_READ, PROT_WRITE};
use std::collections::HashMap;
use std::{ffi::CString, mem::size_of};
use zermia_lib::message;
use zermia_lib::SHARED_MUTEX;

const SHARED_MEM_NAME: &str = "/joao-hugo-shmem";


fn print_message(message: &message::Message, my_map: &mut HashMap<(u32, u32), u32>){
  let ip_v4_src = std::net::IpAddr::V4(u32::from_be(message.ip_src).into());
  let ip_v4_dst = std::net::IpAddr::V4(u32::from_be(message.ip_dest).into());

  if let Some(value) = my_map.get(&(message.ip_src, message.ip_dest)) {
        println!(
            "Registamos outro pacote de dados entre {} e {}. Contagem: {}",
            ip_v4_src, ip_v4_dst, *value
        );
    }

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
}


fn treat_data_packets(message: &message::Message, my_map: &mut HashMap<(u32, u32), u32>) -> bool {
    
    if message.code == 3 /* 28 */ {
      *my_map.entry((message.ip_src, message.ip_dest)).or_insert(0) += 1;
    }

    print_message(&message, my_map);

    if let Some(value) = my_map.get(&(message.ip_src, message.ip_dest)) {
        if *value == 40 {
          if message.code == 18 {
            println!("{}", "Trying not to send or something... network_interface.c".yellow());
          }
          else if message.code == 28 || message.code == 23 {
            // *my_map.entry((message.ip_src, message.ip_dest)).or_insert(0) += 1;
            println!("{}", "Trying not to send or something.. tcp.c".red().bold());
          }
          // else if message.code == 2345{
          //  println!("{} value: {}", "Does this really drop the message? ;-;".red(), *value); 
          //  *my_map.entry((message.ip_src, message.ip_dest)).or_insert(0) += 1;
          //  if let Some(value) = my_map.get(&(message.ip_src, message.ip_dest)) {
          //   println!("{} value: {}", "Does this really drop the message? ;-;".red(), *value); 
          //  }
          //  else {
          //   println!("Error...");
          //  }
          // }
          else if message.code == 2222 {
           println!("{}", "going to return the cyan train".cyan()); 
          }
          else if message.code == 79323 {
            println!("{}", "learning to drop from codel queue".red().bold());
          }
          else if message.code == 11111 {
            println!("{}", "desperate try, can't think of anything else possible...".yellow().bold());
          }
          else if message.code == 111112 {
            println!("{}", "desperate try, can't think of anything else possible...".red().bold());
          }
          else if message.code == 11113 {
            println!("{}", "desperate try, can't think of anything else possible...".purple().bold());
          }
          else if message.code == 111114 {
            println!("{}", "desperate try, can't think of anything else possible...".purple().bold());
          }
          else if message.code == 77777 {
            println!("{}", "desperate try, can't think of anything else possible...".blue().bold());
          }
          else if message.code == 77778 {
            println!("{}", "desperate try, can't think of anything else possible...".blue().bold());
          }
          else if message.code == 999323 {
            println!("{}", "trying to drop here...".red().bold());
          }
          else if message.code == 101010101 {
            println!("{}", "a mensagem do sucesso".purple().bold());
            println!("{}", "a mensagem do sucesso".purple().bold());
            println!("{}", "a mensagem do sucesso".purple().bold());
            println!("{}", "a mensagem do sucesso".purple().bold());
            println!("{}", "a mensagem do sucesso".purple().bold());
            println!("{}", "a mensagem do sucesso".purple().bold());
            println!("{}", "a mensagem do sucesso".purple().bold());
            println!("{}", "a mensagem do sucesso".purple().bold());
            println!("{}", "a mensagem do sucesso".purple().bold());
            println!("{}", "a mensagem do sucesso".purple().bold());
            println!("{}", "a mensagem do sucesso".purple().bold());
            println!("{}", "a mensagem do sucesso".purple().bold());
            println!("{}", "a mensagem do sucesso".purple().bold());
            println!("{}", "a mensagem do sucesso".purple().bold());
            println!("{}", "a mensagem do sucesso".purple().bold());
            println!("{}", "a mensagem do sucesso".purple().bold());
            println!("{}", "a mensagem do sucesso".purple().bold());
            println!("{}", "a mensagem do sucesso".purple().bold());
            println!("{}", "a mensagem do sucesso".purple().bold());
          }

          else{
            println!("{}", "^^^^^^ Vamos alterar o payload deste ^^^^^^".red());
          }
          return false;

          
        } else {
          println!("{}", "^^^^^^ Supostamente vamos enviar este ^^^^^^".green());
          return true;
        }
    } else {
        println!(
            "{}",
            "^^^^^^ Este está set a true, vamos enviar.. ^^^^^^".green()
        );
        return true;
    }
}

// fn treat_control_packets() {
//     println!("É suposto ser um packet de controlo.. então só imprimimos esta mensagem");
// }

fn main() {
    let mut cnt_N = 0;

    let mut my_map: HashMap<(u32, u32), u32> = HashMap::new();
    let c_shared_mem_name = CString::new(SHARED_MEM_NAME).unwrap();
    let mem_size =
        size_of::<i32>() + size_of::<message::Message>() + size_of::<bool>() + size_of::<bool>();

    // Inicializar
    let shm_fd = unsafe {
        shm_open(
            c_shared_mem_name.as_ptr(),
            libc::O_RDWR | libc::O_CREAT,
            libc::S_IRUSR | libc::S_IWUSR,
        )
    };
    if shm_fd == -1 {
        panic!("Failed to open shared memory");
    }

    let res = unsafe { ftruncate(shm_fd, mem_size as i64) };
    if res == -1 {
        panic!("Failed to set size of shared memory");
    }

    let ptr = unsafe {
        mmap(
            std::ptr::null_mut(),
            mem_size,
            PROT_READ | PROT_WRITE,
            MAP_SHARED,
            shm_fd,
            0,
        )
    };
    if ptr == MAP_FAILED {
        panic!("Failed to map memory");
    }

    let should_continue_ptr =
        unsafe { ptr.offset(size_of::<i32>() as isize + size_of::<message::Message>() as isize) }
            as *mut bool;
    println!("(Sempre que alguém escrever para esta região de memória partilhada em /dev/shm/joao-hugo-shmem o monitor vai imprimir a mensagem escrita)");
    println!("O monitor iniciou...");

    let message_ptr = unsafe { ptr.offset(size_of::<i32>() as isize) } as *const message::Message;
    let counter_ptr = ptr as *mut i32;
    let mut last_counter = unsafe { *counter_ptr };
    let check_state = unsafe { ptr.offset((mem_size - size_of::<bool>()) as isize) } as *mut bool;

    loop {
        let _guard = SHARED_MUTEX.lock().unwrap();

        if last_counter != unsafe { *counter_ptr } {
            let message = unsafe { message_ptr.read() };

            match message.code {
                9999 => {
                    /* End the simulation */
                    println!("Recebemos o código 9999");
                    println!("A simulação acabou");
                    unsafe {
                        *check_state = true;
                    }
                    break;
                }
              }
            unsafe {
                *check_state = true;
            }
            last_counter = unsafe { *counter_ptr };
        }
        drop(_guard);
    }
}
