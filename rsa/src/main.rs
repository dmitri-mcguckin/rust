use modexp as m;

pub fn genkey() -> (u32, u32) {

	(0, 0)
}

pub fn encrypt(key: u64, msg: u32) -> u64 {

	0
}

pub fn decrypt(key: (u32, u32)) -> u32 {

	0
}

fn gcd(a: u64, b: u64) -> u64 {

	1
}

fn lcm(a: u64, b: u64) -> u64 {
	(a * b / (gcd(a, b)))
}

fn main() {
	let a: u64 = 20;
	let b: u64 = 30;
	let res = lcm(a, b);
	println!("lcm({}, {}): {}", a, b, res);
}
