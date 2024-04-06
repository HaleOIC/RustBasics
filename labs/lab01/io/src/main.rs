use std::io::{self, Write};

fn main() {
    print!("What is your name? ");
    io::stdout().flush().unwrap();

    let mut name = String::new();
    io::stdin().read_line(&mut name).unwrap();

    name = name.trim().to_string();

    if name.is_empty() {
        println!("No name entered :(, goodbye.");
    } else {
        println!("Hello, {}, nice to meet you!", name);
    }
}