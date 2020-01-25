use std::env;
use std::process;

static MAX: u64 = 4_294_967_296;

fn usage(e: String) -> ! {
    eprintln!("{}\n\nusage: modexp <base> <exponent> <modulo>", e);
    process::exit(0);
}

pub fn modexp(_base: u64, _exponent: u64, _modulo: u64) -> u64 {
    if _base == 0 {
        return 0;
    }
    if _exponent == 0 {
        return 1;
    }

    if _modulo == 0 {
        usage(
            "error: Modulo cannot be zero!\n\tThis would cause a divide-by-zero operation!"
                .to_string(),
        );
    }
    if _base >= MAX || _exponent >= MAX || _modulo >= MAX {
        usage("error: Input is too large!\n\tLargest allowed number is 2^32!".to_string());
    }

    let mut res = modexp(_base, _exponent / 2, _modulo);
    res = (res * res) % _modulo;

    if _exponent % 2 != 0 {
        res = (res * _base) % _modulo;
    }
    res
}

fn str_to_u64(user_input: &str) -> u64 {
    let user_input: u64 = match user_input.trim().parse() {
        Ok(num) => num,
        Err(e) => usage(e.to_string()),
    };

    user_input
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 4 {
        usage("error: Incorrect number of arguments!".to_string());
    }
    let base = str_to_u64(&args[1]);
    let exponent = str_to_u64(&args[2]);
    let modulo = str_to_u64(&args[3]);

    let res = modexp(base, exponent, modulo);
    println!("Result: {}", res);
}

#[test]
fn test_1() {
    let x = modexp(2, 20, 17);
    assert!(x == 16);
}

#[test]
fn test_2() {
    let x = modexp(2, 25, 7);
    assert!(x == 2);
}

#[test]
fn test_3() {
    let x = modexp(3, 17, 3);
    assert!(x == 0);
}

#[test]
#[should_panic]
fn test_4() {
    let ref arg = "a".to_string();
    let _x = str_to_u64(&arg);
}

#[test]
#[should_panic]
fn test_5() {
    let ref arg = "-2".to_string();
    let _x = str_to_u64(&arg);
}
