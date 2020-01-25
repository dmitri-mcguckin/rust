use std::convert::TryFrom;
use std::process;
use std::{fs, fs::File};
use toy_rsa_lib::*;

const E: u32 = 65537;

pub fn u64_u32(x: u64) -> u32 {
    u32::try_from(x).unwrap()
}
pub fn u32_u64(x: u32) -> u64 {
    u64::try_from(x).unwrap()
}

pub fn write_key(filename: &str, key: u64) {
    // Open the key-file
    match File::create(filename) {
        Ok(f) => f,
        Err(e) => {
            println!(
                "There was a problem creating the file: {}\n\tError: {}",
                filename, e
            );
            process::exit(-1);
        }
    };

    // Write out data to key-file
    match fs::write(filename, key.to_string()) {
        Ok(f) => f,
        Err(e) => {
            println!(
                "There was a problem writing to the file: {}\n\tError: {}",
                filename, e
            );
            process::exit(-1);
        }
    };
}

pub fn read_key(filename: &str) -> u64 {
    // Open the key-file
    match File::open(filename) {
        Ok(f) => f,
        Err(e) => {
            println!(
                "There was a problem opening the file: {}\n\tError: {}",
                filename, e
            );
            process::exit(-1);
        }
    };

    // Read the data
    let data = fs::read_to_string(filename).unwrap();

    // Trim + convert to u64's and return as key
    data.trim().parse::<u64>().unwrap()
}

fn genprime() -> u32 {
    let mut x: u32 = rsa_prime();
    while x < E && gcd(u32_u64(x), u32_u64(E)) != 1 {
        x = rsa_prime();
    }
    x
}

pub fn genkey() -> (u32, u32) {
    // Randomly generate two large prime numbers bigger than E
    (genprime(), genprime()) // Return modulo
}

pub fn encrypt(key: u64, msg: u32) -> u64 {
    modexp(u32_u64(msg), u32_u64(E), key)
}

pub fn decrypt(key: (u32, u32), msg: u64) -> u32 {
    let (p, q) = key;
    let modulo = u32_u64(p) * u32_u64(q);
    let totient = u32_u64(p - 1) * u32_u64(q - 1);
    let secret = modinverse(u32_u64(E), totient);
    u64_u32(modexp(msg, secret, modulo))
}
