use libc::{
    ftruncate, mmap, /* munmap, */ shm_open, /* shm_unlink, */ MAP_FAILED, MAP_SHARED,
    PROT_READ, PROT_WRITE,
};
use std::os::raw::c_char;
use std::{ffi::CString, mem::size_of};
use std::env;

const SHARED_MEM_NAME: &str = "/joao-hugo-shmem";
const MEM_SIZE: usize = 4096;

fn main() {
    let c_shared_mem_name = CString::new(SHARED_MEM_NAME).unwrap();
    let mem_size = size_of::<i32>() + MEM_SIZE + size_of::<bool>();

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

    if let Some(arg1) = env::args().nth(1) {
        let host_name: String = arg1.parse().unwrap();

        let counter_ptr = ptr as *mut i32;
        let message_ptr = unsafe { ptr.offset(size_of::<i32>() as isize) } as *mut c_char;
        let ready_ptr =
            unsafe { ptr.offset(size_of::<i32>() as isize + MEM_SIZE as isize) } as *mut bool;

        let mut counter = 0;

        loop {
            if counter == 20 {
                break;
            }
            unsafe {
                // Write a new message to shared memory
                let new_message = std::format!("[{}] enviou o counter: {}", host_name, counter);
                let c_str_new_message = CString::new(new_message).unwrap();
                std::ptr::copy_nonoverlapping(
                    c_str_new_message.as_ptr(),
                    message_ptr,
                    c_str_new_message.as_bytes_with_nul().len(),
                );
                *counter_ptr = counter;

                *ready_ptr = false;
                while !*ready_ptr { /* Esperar para avançar para a próxima mensagem */ }

                println!("[Dentro da simulação] O counter está a {}", counter);
            }
            counter += 1;
        }
    } else {
        println!("Must pass the host-name as an argument");
    }
}
