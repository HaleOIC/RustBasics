////////// DO NOT CHANGE BELOW HERE /////////
fn print_result(num: i32) {
    println!("The result is {num}");
}
////////// DO NOT CHANGE ABOVE HERE /////////

macro_rules! math {
    ($var1:literal plus $var2:literal) => {
        $var1 + $var2
    };
    (square $var:literal) => {
        $var * $var
    }
}

////////// DO NOT CHANGE BELOW HERE /////////

fn main() {
    print_result(math!(3 plus 5));
    print_result(math!(square 2));
}
