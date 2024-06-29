# MacroKata

Welcome to MacroKata, a set of exercises which you can use to learn how to write
macros in Rust. When completing each task, there are three goals:

- Get your code to compile without warnings or errors.
- Get your code to "work correctly" (i.e. produce the same output)
- Importantly, *generate the same code* as what the sample solution does.

You should complete the kata in order, as they increase in
difficulty, and depend on previous kata.

This set of exercises is written for people who have already spent some time
programming in Rust. Before completing this, work through a Rust tutorial
and build some small programs yourself.

## Getting Started

Clone this repository:

``` sh
git clone https://www.github.com/tfpk/macrokata/
```

You will also need to install the Rust "nightly" toolchain, so that we can show
expanded macros:

``` sh
rustup toolchain install nightly
```

Next, install `cargo-expand`:

``` sh
cargo install cargo-expand
```

Build the main binary provided with this repo:

``` sh
cargo build --bin macrokata
```

You can find the first kata (`my_first_macro`) inside `exercises/01_my_first_macro`.
Read the [first chapter of the book](https://tfpk.github.io/macrokata/01_README.html)
and get started by editing the `main.rs` file.

To compare your expanded code to the "goal", use the `test` subcommand:

``` sh
cargo run -- test 01_my_first_macro
```

You can run your own code as follows:

``` sh
cargo run --bin 01_my_first_macro
```

## How To Learn About Procedural Macros

I was originally planning to expand `macrokata` into discussing procedural
macros as well. As I was researching that, I found dtolnay's superlative [Proc
Macro Workshop](https://github.com/dtolnay/proc-macro-workshop).
[Jon Gjengset's video on proc-macros](https://www.youtube.com/watch?v=geovSK3wMB8)
is also a phenomenal resource (despite its length).

I've put my attempt to write something like that on hold because I think the
above is better in every way. Do file an issue if there's something that we
could do here to complement that workshop though.

## Finished records

```shell
(base) ➜  macrokata git:(main) cargo run -- test 01_my_first_macro
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.05s
     Running `target/debug/macrokata test 01_my_first_macro`

This is the expansion you produced:

fn main() {
    show_output()
}

The expansion we expected is:

fn main() {
    show_output()
}

Congratulations! You solved it.
```

```shell
(base) ➜  macrokata git:(main) ✗ cargo run -- test 02_numbers
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.05s
     Running `target/debug/macrokata test 02_numbers`

This is the expansion you produced:

fn main() {
    print_result(1 + 2 + 3);
}

The expansion we expected is:

fn main() {
    print_result(1 + 2 + 3);
}

Congratulations! You solved it.
```

```shell
(base) ➜  macrokata git:(main) ✗ cargo run -- test 03_literal_variables
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.05s
     Running `target/debug/macrokata test 03_literal_variables`

This is the expansion you produced:

fn main() {
    print_result(3 + 5);
    print_result(2 * 2);
}

The expansion we expected is:

fn main() {
    print_result(3 + 5);
    print_result(2 * 2);
}

Congratulations! You solved it.
```

```shell
(base) ➜  macrokata git:(main) ✗ cargo run -- test 04_expression_variables
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.05s
     Running `target/debug/macrokata test 04_expression_variables`

This is the expansion you produced:

fn main() {
    let var = 5;
    print_result((2 * 3) + var);
    print_result(var * var);
}

The expansion we expected is:

fn main() {
    let var = 5;
    print_result((2 * 3) + var);
    print_result(var * var);
}

Congratulations! You solved it.
```

```sh
(base) ➜  macrokata git:(main) ✗ cargo run -- test 05_more_complex_example
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s
     Running `target/debug/macrokata test 05_more_complex_example`

This is the expansion you produced:

fn main() {
    for row in 1..5 {
        let row: i32 = row;
        for col in 2..7 {
            let col: i32 = col;
            { (Coordinate { x: col, y: row }).show() }
        }
    }
    let values = [1, 3, 5];
    for x in values {
        let x: u16 = x;
        for y in values {
            let y: u16 = y;
            {
                (Coordinate {
                    x: x.into(),
                    y: y.into(),
                })
                    .show()
            }
        }
    }
}

The expansion we expected is:

fn main() {
    for row in 1..5 {
        let row: i32 = row;
        for col in 2..7 {
            let col: i32 = col;
            { (Coordinate { x: col, y: row }).show() }
        }
    }
    let values = [1, 3, 5];
    for x in values {
        let x: u16 = x;
        for y in values {
            let y: u16 = y;
            {
                (Coordinate {
                    x: x.into(),
                    y: y.into(),
                })
                    .show()
            }
        }
    }
}

Congratulations! You solved it.
```

```sh
(base) ➜  macrokata git:(main) ✗ cargo run -- test 06_repetition
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.07s
     Running `target/debug/macrokata test 06_repetition`

This is the expansion you produced:

fn main() {
    if false || 0 == 1 || true {
        print_success();
    }
}

The expansion we expected is:

fn main() {
    if false || 0 == 1 || true {
        print_success();
    }
}

Congratulations! You solved it.
```

```shell
(base) ➜  macrokata git:(main) ✗ cargo run -- test 07_more_repetition
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.11s
     Running `target/debug/macrokata test 07_more_repetition`

This is the expansion you produced:

fn main() {
    let value = "my_string";
    let my_hashmap = {
        let mut hm = HashMap::new();
        hm.insert("hash", "map");
        hm.insert("Key", value);
        hm
    };
    print_hashmap(&my_hashmap);
}

The expansion we expected is:

fn main() {
    let value = "my_string";
    let my_hashmap = {
        let mut hm = HashMap::new();
        hm.insert("hash", "map");
        hm.insert("Key", value);
        hm
    };
    print_hashmap(&my_hashmap);
}

Congratulations! You solved it.
```

```sh
(base) ➜  macrokata git:(main) ✗ cargo run -- test 08_nested_repetition
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.09s
     Running `target/debug/macrokata test 08_nested_repetition`

This is the expansion you produced:

#[allow(clippy::vec_init_then_push)]
fn main() {
    let my_graph = {
        let mut vec = Vec::new();
        vec.push((1, 2));
        vec.push((1, 3));
        vec.push((1, 4));
        vec.push((1, 5));
        vec.push((2, 1));
        vec.push((2, 3));
        vec.push((3, 2));
        vec.push((5, 1));
        vec.push((5, 2));
        vec.push((5, 3));
        vec
    };
    print_vec(&my_graph);
}

The expansion we expected is:

#[allow(clippy::vec_init_then_push)]
fn main() {
    let my_graph = {
        let mut vec = Vec::new();
        vec.push((1, 2));
        vec.push((1, 3));
        vec.push((1, 4));
        vec.push((1, 5));
        vec.push((2, 1));
        vec.push((2, 3));
        vec.push((3, 2));
        vec.push((5, 1));
        vec.push((5, 2));
        vec.push((5, 3));
        vec
    };
    print_vec(&my_graph);
}

Congratulations! You solved it.
```

```shell
(base) ➜  macrokata git:(main) ✗ cargo run -- test 10_macros_calling_macros
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.07s
     Running `target/debug/macrokata test 10_macros_calling_macros`

This is the expansion you produced:

fn main() {
    let my_number = "93720".parse::<u32>().unwrap();
    let my_other_number = "124680".parse::<u32>().unwrap();
    {
        ::std::io::_print(format_args!("{0}\n", my_number + my_other_number));
    };
}

The expansion we expected is:

fn main() {
    let my_number = "93720".parse::<u32>().unwrap();
    let my_other_number = "124680".parse::<u32>().unwrap();
    {
        ::std::io::_print(format_args!("{0}\n", my_number + my_other_number));
    };
}

Congratulations! You solved it.
```

```shell
(base) ➜  macrokata git:(main) ✗ cargo run -- test 11_macro_recursion
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.05s
     Running `target/debug/macrokata test 11_macro_recursion`

This is the expansion you produced:

fn main() {
    {
        ::std::io::_print(format_args!("=== defining functions ===\n"));
    };
    let is_between = move |min: i32| {
        print_curried_argument(min);
        move |max: i32| {
            print_curried_argument(max);
            move |item: &i32| {
                print_curried_argument(item);
                { min < *item && *item < max }
            }
        }
    };
    let curry_filter_between = move |min: i32| {
        print_curried_argument(min);
        move |max: i32| {
            print_curried_argument(max);
            move |vec: &Vec<i32>| {
                print_curried_argument(vec);
                {
                    let filter_between = is_between(min)(max);
                    vec.iter()
                        .filter_map(|i| if filter_between(i) { Some(*i) } else { None })
                        .collect()
                }
            }
        }
    };
    {
        ::std::io::_print(format_args!("=== create between_3_7 ===\n"));
    };
    let between_3_7 = curry_filter_between(3)(7);
    {
        ::std::io::_print(format_args!("=== create between_5_10 ===\n"));
    };
    let between_5_10 = curry_filter_between(5)(10);
    let my_vec = get_example_vec();
    {
        ::std::io::_print(format_args!("=== call between_3_7 ===\n"));
    };
    let some_numbers: Vec<i32> = between_3_7(&my_vec);
    print_numbers(&some_numbers);
    {
        ::std::io::_print(format_args!("=== call between_5_10 ===\n"));
    };
    let more_numbers: Vec<i32> = between_5_10(&my_vec);
    print_numbers(&more_numbers);
}

The expansion we expected is:

fn main() {
    {
        ::std::io::_print(format_args!("=== defining functions ===\n"));
    };
    let is_between = move |min: i32| {
        print_curried_argument(min);
        move |max: i32| {
            print_curried_argument(max);
            move |item: &i32| {
                print_curried_argument(item);
                { min < *item && *item < max }
            }
        }
    };
    let curry_filter_between = move |min: i32| {
        print_curried_argument(min);
        move |max: i32| {
            print_curried_argument(max);
            move |vec: &Vec<i32>| {
                print_curried_argument(vec);
                {
                    let filter_between = is_between(min)(max);
                    vec.iter()
                        .filter_map(|i| if filter_between(i) { Some(*i) } else { None })
                        .collect()
                }
            }
        }
    };
    {
        ::std::io::_print(format_args!("=== create between_3_7 ===\n"));
    };
    let between_3_7 = curry_filter_between(3)(7);
    {
        ::std::io::_print(format_args!("=== create between_5_10 ===\n"));
    };
    let between_5_10 = curry_filter_between(5)(10);
    let my_vec = get_example_vec();
    {
        ::std::io::_print(format_args!("=== call between_3_7 ===\n"));
    };
    let some_numbers: Vec<i32> = between_3_7(&my_vec);
    print_numbers(&some_numbers);
    {
        ::std::io::_print(format_args!("=== call between_5_10 ===\n"));
    };
    let more_numbers: Vec<i32> = between_5_10(&my_vec);
    print_numbers(&more_numbers);
}

Congratulations! You solved it.
```

```shell
(base) ➜  macrokata git:(main) ✗ cargo run -- test 12_hygienic_macros
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.08s
     Running `target/debug/macrokata test 12_hygienic_macros`

This is the expansion you produced:

fn main() {
    let four_dim = crate::fourth_dimension::Coordinate {
        x: 1,
        y: 2,
        z: 3,
        t: 1000,
    };
    let three_dim = four_dim.as_3d();
    let two_dim = three_dim.as_2d();
    {
        ::std::io::_print(format_args!("{0:?}\n", two_dim));
    };
}

The expansion we expected is:

fn main() {
    let four_dim = crate::fourth_dimension::Coordinate {
        x: 1,
        y: 2,
        z: 3,
        t: 1000,
    };
    let three_dim = four_dim.as_3d();
    let two_dim = three_dim.as_2d();
    {
        ::std::io::_print(format_args!("{0:?}\n", two_dim));
    };
}

Congratulations! You solved it.
```
