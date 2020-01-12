use std::io;

fn modexp(x: i32, n: i32, m: i32) -> i32 {
   x + n + m
}

fn main() {
    let mut arg1 = String::new();
    let mut arg2 = String::new();
    let mut arg3 = String::new();

    let mut x: i32 = arg1.unwrap();
    let mut n: i32 = arg1.unwrap();
    let mut m: i32 = arg1.unwrap();

    io::stdin().read_line(&mut x).expect("Could not read line!");
    io::stdin().read_line(&mut n).expect("Could not read line!");
    io::stdin().read_line(&mut m).expect("Could not read line!");

    println!("result: {}", modexp(x, n, m));
}
