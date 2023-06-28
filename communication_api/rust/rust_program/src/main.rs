extern crate zermia_lib;


fn string_to_u8_32(string: &str) -> [u8; 32] {
  let mut array = [0; 32];
  let bytes = string.as_bytes();
  
  for (&x, p) in bytes.iter().zip(array.iter_mut()) {
      *p = x;
  }
  array
}


fn main() {
    let message = zermia_lib::message::Message { 
        // fill the message structure here
        code: 2345,
        ip_src: 12345,
        ip_dest: 54321,
        msg: string_to_u8_32("hellow world from rust"),
        return_status: true,
    };
    let result = zermia_lib::send_zermia_message(message);
    println!("{}", result);
}