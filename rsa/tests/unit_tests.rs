use rsa::*;

#[test]
fn unit_test_1() {
    let x = 5u32;
    let y = u32_u64(x);
    assert_eq!(y, 5u64);
}

#[test]
fn unit_test_2() {
    let x = 5u64;
    let y = u64_u32(x);
    assert_eq!(y, 5u32);
}

#[test]
fn unit_test_3() {
    let (x, y) = rsa::genkey();
    assert!(x % 2 != 0);
    assert!(y % 2 != 0);
}

#[test]
fn unit_test_4() {
    let (x, y) = rsa::genkey();
    let msg = 1358u32;
    let modulo: u64 = u32_u64(x) * u32_u64(y);
    let cipher = encrypt(modulo, msg);
    assert!(cipher != u32_u64(msg));
}

#[test]
fn unit_test_5() {
    let (x, y) = rsa::genkey();
    let msg = 1358u32;
    let modulo: u64 = u32_u64(x) * u32_u64(y);
    let cipher = encrypt(modulo, msg);
    let res = decrypt((x, y), cipher);
    assert_eq!(msg, res);
}
