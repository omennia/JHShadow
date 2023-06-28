No rust só temos de adicionar o path da library no Cargo.toml:
```rust
[dependencies]
zermia_lib = { path = "/home/hugocardante/JHShadow/src/lib/zermia_lib" }
```

Depois no nosso programa Rust:
```rust

extern crate zermia_lib; // chamamos a extern crate
 

fn string_to_u8_32(string: &str) -> [u8; 32] { // Talvez haja maneiras mais fáceis para isto
	let mut array = [0; 32];
	let bytes = string.as_bytes();
	for (&x, p) in bytes.iter().zip(array.iter_mut()) {
		*p = x;
	}
	array
}

  
fn main() {
	let message = zermia_lib::message::Message {
		code: 2345,
		ip_src: 12345,
		ip_dest: 54321,
		msg: string_to_u8_32("hellow world from rust"),
		return_status: true,
	};
	
	let result = zermia_lib::send_zermia_message(message);
	println!("{}", result);
}
```