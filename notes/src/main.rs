use std::io::BufRead;

fn main() {
    let x = 1; // Local variables

    let stdin = std::io::stdin(); // Standard input buffer
    let mut sum = 0;

    for line in stdin.lock().lines() {
        // Logic block
        let line = line.expect("io error");
        let val: i64 = line.parse().expect(" invalid number");
        sum += val;
    }

    println!("{}", sum);
}
