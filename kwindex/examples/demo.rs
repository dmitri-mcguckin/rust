use kwindex::*;
use std::env::*;
use std::process::*;

fn usage(msg: &str) {
    println!("{} \
             \nUsage: cargo run --example demo \"<message>\" <match word [optional]> \
             \n\tmessage: the message to parse into keywords \
             \n\tmatch word: search the message for this key-word and return number of occurences", msg);
    exit(-1);
}

fn main() {
    let args: Vec<String> = args().collect();

    match args.len() {
        1 => usage("Too few arguments!"),
        2 => {
            let text = &args[1].to_string();
            let mut index = KWIndex::new();
            index = index.extend_from_text(text);
            println!("Empty: {}\nLength: {}", index.is_empty(), index.len());
        }
        3 => {
            let text = &args[1].to_string();
            let word_match = &args[2].to_string();
            let mut index = KWIndex::new();
            index = index.extend_from_text(text);
            println!("Empty: {}\nLength: {}\nMatch Count: {}", index.is_empty(), index.len(), index.count_matches(word_match));
        }
        _ => usage("Too many arguments!"),
    }
}
