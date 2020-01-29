use rand::Rng;
use rsa::*;

// Randomly generate a key, encrypt, then decrpyt
//  Then proove the message is the same 1000 times
#[test]
fn rand_1000_test() {
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
