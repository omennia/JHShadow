use libc::{mmap, shm_open, /* shm_unlink,  */ MAP_FAILED, MAP_SHARED, PROT_READ, PROT_WRITE};
use std::os::raw::c_char;
use std::{ffi::CString, mem::size_of};

const SHARED_MEM_NAME: &str = "/joao-hugo-shmem";
const MEM_SIZE: usize = 4096;

fn main() {
    let c_shared_mem_name = CString::new(SHARED_MEM_NAME).unwrap();
    let mem_size = size_of::<i32>() + MEM_SIZE + size_of::<bool>();

    let shm_fd = unsafe {
        shm_open(
            c_shared_mem_name.as_ptr(),
            libc::O_RDWR,
            libc::S_IRUSR | libc::S_IWUSR,
        )
    };
    if shm_fd == -1 {
        panic!("Failed to open shared memory");
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

    println!("(Sempre que alguém escrever para esta região de memória partilhada em /dev/shm/joao-hugo-shmem o monitor vai imprimir a mensagem escrita)");
    println!("O monitor iniciou...");
    let counter_ptr = ptr as *const i32;
    let message_ptr = unsafe { ptr.offset(size_of::<i32>() as isize) } as *const c_char;
    let ready_ptr: *mut bool =
        unsafe { ptr.offset(size_of::<i32>() as isize + MEM_SIZE as isize) } as *mut bool;

    loop {
        unsafe {
            // Esperar por uma mensagem nova
            while *ready_ptr {
                std::thread::sleep(std::time::Duration::from_millis(100));
            }

            // Imprimir a mensagem da shared memory
            let message = std::ffi::CStr::from_ptr(message_ptr).to_string_lossy();
            let counter = *counter_ptr;
            println!(
                "[Fora da simulação] Mensagem recebida: {}, counter: {}",
                message, counter
            );

            // Pomos a true para a próxima mensagem poder ser enviada
            *ready_ptr = true;
        }
    }

    /* Não precisamos disto, assim que o programa terminar acho que a memoria é unlinked */
    // let res = unsafe { shm_unlink(c_shared_mem_name.as_ptr()) };
    // if res == -1 {
    //     panic!("Failed to unlink shared memory");
    // }
}
