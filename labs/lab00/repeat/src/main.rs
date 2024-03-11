use std::io;


fn main() -> io::Result<()> {
    // define an mutable variable as buffer to sustain new input string
    let mut buffer = String::new();

    // declare a new stdin to read the input
    let stdin = io::stdin();

    // read first line by using stdin function and store contents into buffer
    stdin.read_line(&mut buffer)?;

    // print out acquired string into the terminal 
    print!("{}", buffer);

    // return back ok code
    Ok(())
}
