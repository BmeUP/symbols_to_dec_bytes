use std::io;
fn main() {
    println!("{:?}", to_bytes());
}

fn to_bytes() -> Vec<u8>{
    let mut raw_string: String = String::new();

    println!("Enter characters ");
    io::stdin().read_line(&mut raw_string).expect("Ups...");

    let bytes_string:Vec<u8> = raw_string.into_bytes();
    
    bytes_string
}
