# Revision Questions

### Q1

In order to set up rust in Ubuntu, we prefer to use the following command, and once finished, it will echo `Rust is installed now. Great!`.
```shell
$ curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
```

### Q2 
Input the following codes into `main.rs`

```rust
fn main() {
    println!("Hello world!");
}
```
and then using the commands 
```shell
$ rustc main.rs
$ ./main
```
Right now, convert it into Cargo version, which plays an important roles in construction of project.
```shell
$ cargo new hello_world
$ cd hello_world
$ cargo run
```
It will display the following hint message:
```shell
Compiling hello_world v0.1.0 (/home/hale/UNSW_COMP6991/lectrues/week1/RQ/Ex2/hello_world)
    Finished dev [unoptimized + debuginfo] target(s) in 0.43s
     Running `target/debug/hello_world`
Hello, world!
```

### Q3
Given C program shows how to determine a integer is positivie or negative.
```rust
fn main() {
    let my_integer = 42;
    if my_integer > 0 {
        println!("{} is positive\n", my_integer);
    } else if my_integer < 0 {
        println!("{} is negative\n", my_integer);
    } else {
        println!("{} is zero\n", my_integer);
    }
}
```

### Q4 
Given C program is to count self-increase until it is bigger than or equals to 10.
```rust 
fn main() {
    let mut count = 0;
    loop {
        if count >= 10 {
            break;
        }
        print!("{}\n", count);
        count = count + 1;
    }
}
```

### Q5
Given C program is to enumerate each value in range `[0, 10)` and print out.
```rust
fn main() {
    for count in 0..10 {
        println!("{}", count);
    }
}
```

### Q6
```rust
fn main() {
    // This varible 'x' is unmutable
    let x = 42;
    my_function(None);
    my_function(Some(&x));
}

// Optional can avoid null pointer dereferences and explicit null checks.
fn my_function(x: Option<&i32>) {
    match x {
        Some(val) => {
            println!("x has value {}", val);
        }
        None => {
            println!("no value for x");
        }
    }
}
```

Some key aspects for this program:
1. **Option Type**: Instead of using raw pointers and NULL, Rust uses the Option type for values that may be absent (None) or present (Some). This makes code safer by avoiding null pointer dereferences and explicit null checks.

2. **Pattern Matching**: Rust uses pattern matching with match to handle Option values, which makes the code more expressive and clear about what is being checked.

3. **Safety**: The Rust version does not use raw pointers directly, reducing the risk of null pointer dereferences, segmentation faults, and similar issues common in C due to improper pointer handling.


**Advantages and Disadvantages:**
- Advantage: The Rust approach with Option and pattern matching increases the safety and readability of the code. Rust's strict type system and ownership model prevent many classes of bugs that are common in C, such as null pointer dereferences and memory leaks.

- Disadvantage: For developers familiar with languages that make extensive use of pointers and null values (like C), Rust's borrowing and ownership rules, as well as its use of Option for optional values, may have a steeper learning curve. Additionally, in scenarios where direct and low-level memory manipulation is required, Rust's safety features might impose additional restrictions that need to be worked around using unsafe code blocks, although such scenarios are less common.

Overall, the Rust language design emphasizes safety, concurrency, and memory management without a garbage collector. The use of Option for representing potential absence of a value is a clear example of Rust's approach to safety and type correctness, which can be seen as an advantage in terms of reducing certain types of runtime errors. However, it requires a shift in thinking from traditional C programming practices.