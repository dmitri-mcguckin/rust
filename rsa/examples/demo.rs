use rsa::*;
use std::env::*;
use std::process;

fn main() {
    // Gather command-line arguments
    let args: Vec<String> = args().collect();
    if args.len() != 2 {
        println!("Usage: cargo run --example demo <message>");
        process::exit(-1);
    }

    let (p, q) = genkey();
    println!("Private Key: p = 0x{:x} q = 0x{:x}", p, q);

    let modulo: u64 = u32_u64(p) * u32_u64(q);
    println!("Public Key: p * q = 0x{:x}", modulo);

    let request = *&args[1].trim().parse::<u32>().unwrap();
    let cipher = encrypt(modulo, request);
    let response = decrypt((p, q), cipher);
    println!(
        "Message: {}\nEncrypted: 0x{:x}\nDecrypted: {}",
        request, cipher, response
    );
}
