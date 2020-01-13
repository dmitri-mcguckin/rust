use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    let rng = rand::thread_rng().gen_range(0,101);
    let mut guess_count = 10;

    loop {
        if guess_count == 0 {
            println!("Loser!");
            break;
        }

        let mut guess = String::new();
        println!("\nPick a number. [{} chances remaining]", guess_count);
        io::stdin().read_line(&mut guess).expect("Ambiguous IO error!");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("\nInvalid input!");
                continue;
            },
        };

        match guess.cmp(&rng) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("Winner!");
                break;
            }
        }

        guess_count -= 1;
    }
}
