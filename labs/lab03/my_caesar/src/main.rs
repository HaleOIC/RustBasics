use std::char;
use std::env;
use std::io::{stdin, BufRead};

const DEFAULT_SHIFT: i32 = 5;

fn main() {
    let shift_by: i32 = env::args()
        .nth(1)
        .and_then(|arg| arg.parse().ok())
        .unwrap_or(DEFAULT_SHIFT);

    for line in stdin().lock().lines() {
        let shifted = shift(shift_by, line.expect("no input line"));

        println!("Shifted ascii by {shift_by} is: {shifted}");
    }
}

fn shift(offset: i32, input: String) -> String {
    let mut result = String::new();

    for c in input.chars() {
        let code = c as u32;
        let shifted_char = if code >= 65 && code <= 90 {
            char::from_u32((code - 65 + offset as u32) % 26 + 65).unwrap()
        } else if code >= 97 && code <= 122 {
            char::from_u32((code - 97 + offset as u32) % 26 + 97).unwrap()
        } else {
            c
        };
        result.push(shifted_char);
    }
    result
}
