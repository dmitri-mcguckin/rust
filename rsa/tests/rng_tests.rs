use rand::Rng;
use rsa::*;

#[test]
fn test() {
    for i in 0..1000 {
        println!("iterating test #{}", i);
        let (p, q) = genkey();
        let modulo: u64 = u32_u64(p) * u32_u64(q);
        let request = rand::thread_rng().gen_range(0, 10000);
        let cipher = encrypt(modulo, request);
        let response = decrypt((p, q), cipher);
        assert!(request == response);
    }
}
