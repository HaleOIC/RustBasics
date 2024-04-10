use std::env;
use std::num::ParseIntError;

struct TribonacciError(String);

fn main() {
    let args: Vec<String> = env::args().collect();
    let error_message = String::from("Please enter a valid size");

    let size = match args.get(1) {
        Some(s) => s.parse::<usize>(),
        None => Ok(10),
    };

    if let Err(e) = compute_tribonacci(size, error_message) {
        println!("Error: {}", e.0)
    }
}

/// Computes the tribonacci sequence of a given size
/// Prints the sequence, and its sum
fn compute_tribonacci(
    size: Result<usize, ParseIntError>,
    // The error message your function should return
    // inside the `TribonacciError` struct
    error_msg: String,
) -> Result<(), TribonacciError> {
    let size = match size {
        Ok(value) => { value }
        Err(_) => {
            return Err(TribonacciError(error_msg));
        }
    };
    let mut array: Vec<u128> = vec![1, 1, 1];
    
    // make sure calculate range
    if size < 3 || size > 145 {
        return Err(TribonacciError(String::from("Please enter a valid size!")));
    }

    // Compute the value of each 
    for index in 3..=size - 1 {
        let cur_value = array[index - 1] as u128 
            + array[index - 2] as u128 
            + array[index - 3] as u128;
        array.push(cur_value);
    }
    let overall_sum: u128 = array.clone().into_iter().sum();

    // print out useful message
    println!("Values: {:?}\n", array);
    println!("Sum: {}", overall_sum);

    return Ok(());
}
