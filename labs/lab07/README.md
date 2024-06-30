# README

## fun_types

```shell
(base) ➜  fun_types git:(main) ✗ autotest
Found cargo project: fun_types
/tmp/tmp.8pvGoK9Dyq
Located autotests for fun_types
cargo build --target-dir target # crate.tar
   Compiling fun_types v0.1.0 (/tmp/tmpk_pho1u4/autotest)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.44s
Test 0 (./target/debug/fun_types) - passed
Test 1 (./target/debug/fun_types 1) - passed
Test 2 (./target/debug/fun_types 2) - passed
Test 3 (./target/debug/fun_types 1 2) - passed
Test 4 (./target/debug/fun_types 3) - passed
Test 5 (./target/debug/fun_types 1 2 3) - passed
6 tests passed 0 tests failed
```

## Average Macro
```shell
(base) ➜  average_macro git:(main) ✗ cargo run        
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.03s
     Running `target/debug/average_macro`
a = 3
b = 11
```

## repetition 

```shell
(base) ➜  repetition git:(main) ✗ cargo run 
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/repetition`
Yay, the if statement worked.
```

## Currying 

```shell
(base) ➜  currying git:(main) ✗ cargo run
   Compiling currying v0.1.0 (/home/hale/UNSW_COMP6991/labs/lab07/currying)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.22s
     Running `target/debug/currying`
[
    5,
    6,
]
[
    6,
    7,
    9,
]
(base) ➜  currying git:(main) ✗ cargo expand main
    Checking currying v0.1.0 (/home/hale/UNSW_COMP6991/labs/lab07/currying)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.06s

fn main() {
    let is_between = move |min: i32| {
        move |max: i32| { move |item: &i32| { { min < *item && *item < max } } }
    };
    let curry_filter_between = move |min: i32| {
        move |max: i32| {
            move |vec: &Vec<i32>| {
                {
                    let filter_between = is_between(min)(max);
                    vec.iter()
                        .filter_map(|i| if filter_between(i) { Some(*i) } else { None })
                        .collect()
                }
            }
        }
    };
    let between_3_7 = curry_filter_between(3)(7);
    let between_5_10 = curry_filter_between(5)(10);
    let my_vec = get_example_vec();
    let some_numbers: Vec<i32> = between_3_7(&my_vec);
    print_numbers(&some_numbers);
    let more_numbers: Vec<i32> = between_5_10(&my_vec);
    print_numbers(&more_numbers);
}
``` 

## HOOKED 

```shell
(base) ➜  hooked git:(main) ✗ autotest
Found cargo project: hooked
/tmp/tmp.c5ngdgmj3u
Located autotests for hooked
cargo build --target-dir target # crate.tar
   Compiling hooked v0.1.0 (/tmp/tmpg4x9_gze/autotest)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.40s
Test 0 (./target/debug/hooked) - passed
Test 1 (./target/debug/hooked 1) - passed
Test 2 (./target/debug/hooked 2) - passed
Test 3 (./target/debug/hooked 1 2) - passed
Test 4 (./target/debug/hooked 3) - passed
Test 5 (./target/debug/hooked 1 2 3) - passed
6 tests passed 0 tests failed
```

