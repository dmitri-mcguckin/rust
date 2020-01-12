use std::io::BufRead;

fn modexp(x: i32, n: i32, m: i32) -> i32 {
   x + n + m
}

fn main() {
    let stdin = std::io::stdin(); // Standard in
    let mut c = 1;
    let mut args: [i32] = [0, 0, 0];

    for line in stdin.lock().lines() {
        let line = line.expect("ambiguous io error!");

        let val: i32 = line.parse().expect("NaN");

        println!("In: {}\tC: {}", val, c);
        c += 1;
        if(c > 3) { break; }
    }
}
