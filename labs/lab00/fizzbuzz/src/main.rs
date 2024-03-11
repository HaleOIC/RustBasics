fn main() {
    for i in 1..=100 {
        if i % 3 == 0 {
            if i % 5 == 0 {
                println!("FizzBuzz");
                continue;
            }
            println!("Fizz");
        } else if i % 5 == 0 {
            println!("Buzz");
        } else {
            println!("{i}");
        }
    }
}
