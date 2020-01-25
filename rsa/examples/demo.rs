use rsa::*;
use rand::Rng;

fn main() {
	let (p, q) = genkey();
	println!("Private Key: p = 0x{:x} q = 0x{:x}", p, q);

	let modulo: u64 = u32_u64(p) * u32_u64(q);
	println!("Public Key: p * q = 0x{:x}", modulo);

    let request = rand::thread_rng().gen_range(0,10000);
	let cipher = encrypt(modulo, request);
	let response = decrypt((p, q), cipher);
	println!("Message: {}\nEncrypted: 0x{:x}\nDecrypted: {}", request, cipher, response);
}
