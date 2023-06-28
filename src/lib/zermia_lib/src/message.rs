use serde::{Serialize, Deserialize};

#[repr(C)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Message {
    pub code: u32,
    pub ip_src: u32,
    pub ip_dest: u32,
    pub msg: [u8; 32],
    pub return_status: bool,
}