fn main() {
    // This varible 'x' is unmutable
    let x = 42;
    my_function(None);
    my_function(Some(x));
}

// Optional can avoid null pointer dereferences and explicit null checks.
fn my_function(x: Option<i32>) {
    match x {
        Some(val) => {
            println!("x has value {}", val);
        }
        None => {
            println!("no value for x");
        }
    }
}