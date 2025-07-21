use std::io;
use std::io::stdin;

fn main() {
    println!("Hello, world! this is my own hash function , iam writing it just for fun");

    let mut password = String::new();
    io::stdin().read_line(&mut password).unwrap();

    let mut passByte = password.trim().as_bytes();
    let mut acc:u32 = 0;

    for byte in passByte
    {
        acc = acc^*byte as u32;
    }
    println!("{}",acc*31);

}

teh above code is to hash a password which can easily brute forces
