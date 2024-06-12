use std::env;
use std::fs;

fn main() -> Result<(), String> {
    let args: Vec<String> = env::args().collect(); // return back a dynamic array

    // Introduce Usage msg for hints
    if args.len() != 3 {
        return Err(String::from(
            "Usage: cargo run [searchString] [example-filename]",
        ));
    }

    let query = args.get(1).unwrap();
    let filename = args.get(2).unwrap();
    println!("query string: {:?}, filename: {:?}", query, filename);

    // read content from file
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    println!("With text: \n{}", contents);

    Ok(())
}
