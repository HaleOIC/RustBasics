use std::env;
use std::io;

fn main() {
    let pattern_string = env::args()
        .nth(1)
        .expect("missing required command-line argument: <pattern>");

    let pattern = &pattern_string;

    loop {
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(n) => {
                if input.contains(pattern) {
                    println!("{}", input.trim());
                }
                if n == 0 {
                    break 0;
                }
            }
            Err(_) => {
                break 0;
            },
        }
    };

}
