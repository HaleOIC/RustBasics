use core::num;

/* Define constant like C */
const FOO: i32 = 42;

/* Define struct data Type */
struct Student {
    zid: String,
    age: u32,
    wam: Option<f64>,
}

enum Automobile {
    Car { make: String, model: String, doors: u32 },
    Motorbike { make: String, model: String , horsepower: u32 },
    Train { capacity: u32 },
    Bus { capacity: u32 },  
}


fn tuple_func(mytuple: (i32, bool, char)) {
    /* Tuple.index is good choice */
    println!(
        "The i32 is {}, the bool is {}, the char is {}",
        mytuple.0, mytuple.1, mytuple.2
    );

    /* Unwrap for each field is also useful */
    let (x, y, z) = mytuple;
    println!(
        "The i32 is {}, the bool is {}, the char is {}",
        x, y, z
    );
}

fn array_func(x: [i32; 3]) {
    let [val1, val2, ..] = x;
    println!("val1: {val1}, val2: {val2}"); 
}

type Student1 = (String, u32, Option<u64>);
fn foo(automobile: Automobile) {
    match automobile {
        Automobile::Car { make, model, doors } => {
            println!("It's a car! mode by {make} -- it is the {model} with {doors} doors");
        },
        Automobile::Motorbike { make, model, horsepower } => {

        },
        Automobile::Train { capacity } => {

        },
        Automobile::Bus { capacity } => {

        },
    }
}

fn main() {
    /*
        let mut x = 42;
        which can make x = x * 2 available
     */
    let x = FOO;
    let y = 64;
    let z = 123;

    /* rebinding value and `type` */
    // let x = 100;

    {
        // This is a new scope.
        let x = x * 2;
        println!("x = {x}, y = {y}, z = {z}");
    }
    println!("x = {x}, y = {y}, z = {z}");
    
    /* Data Types:
        i32, u32 
            u32 can never be negative, it preserves 
            some additional information for that.
        i8, u8
        i16, u16
        i64, u64
        i128, u128      
        f32, f64(double)
        bool, char (single character)

        || && 
    */

    /* Tuple: can combine different types */
    let tuple = (1, true, '2');

    let student = Student {
        zid: String::from("Hale"),
        age: 100,
        wam: Some(100.0)
    };
    /* Any feild can not be changed due to immutablity 
        It is really pervasive, all thing is imutable. */
    //  student.age = 1;

    let cur_tuple = (16, false, '$');
    tuple_func(cur_tuple);

    /* Pattern matching    */
    let Student { zid, age, wam } = student;

    let my_array: [i32; 3] = [1, 2, 3];
    array_func(my_array);

    if_expression(true);
    if_expression(false);

    break_expression();
    looprelated();

    ownership();


}


/**
* Consider the following codes
    * 
    * let x = if flag {
    *      42;
    *      println!("The flag is true!");
    * } else {
    *      100;
    *      println!("The flag is false!");
    * }
    * Nothing will happen, so the type of x is `()``
*/

/* If expression can have blocks, and it only cares about last `expression` */
/* Statement = expresion + ";" */
fn if_expression(flag: bool) -> i32 {
    let x = if flag {
        println!("The flag was true!");
        42
    } else {
        println!("The flag was false!");
        100
    };
    println!("{x}");
    println!("{}", if x == 42 { "hello" } else { "goodbyte" });
    x   // it works like `return x`
}

/* Return value early */
fn log_2(number: f32) -> Option<f32> {
    let logrithm: f32 = if number == 0.0 {
        return None;
    } else {
        number.log2()
    };
    
    return Some(logrithm);
}

/* Indicate break X is also expression */
fn break_expression() {
    let mut x = 0;
    let y = loop {
        if x > 10 {
            break 42
        }
        x += 1;
        println!("Hello Students!");
    };
    println!("{y}");
}

fn looprelated() {
    let x = [1, 2, 3, 4, 5];
    for element in x {
        println!("{element}");
    }
}


fn take_result(x: Result<i32, String>) {
    match x {
        Ok(calculation) => {
            println!("The calculation is {}", calculation);
        }
        Err(error_message) => {
            eprintln!("Something really wrong happens {error_message}");
        }
        /* _ means rest */
        _ => {

        }
    }
}

/**
 *  let x = String::from("Ownership!");
    let y = x;

    in this case, x is dead, since all ownership will be unique
    and can be determined when compiling.
    Only basic types and tuple with only basic types can be copy
    other types are `clone` 

    And for stuct and enum, you can use the following macro to make 
    the struct with all basic types to be copy
    #[derive(Copy, Clone)]


    Actually, String is a struct of vec, and it has 
    [ptr, length, capacity], just what we seen in CPP vector
    and *ptr only stores the concrete data, no cookies.
 */
fn ownership() {
    /* String will use malloc() and free() automatically */
    let x = String::from("Ownership!");
    let y = x.clone();
    println!("x: {}, y: {}", x, y);
    let s = y.clone();
    let s = takes_a_string(s);
    println!("I got {s} back");
}


fn takes_a_string(s: String) -> String {
    println!("I got the string {s}");
    s
}
