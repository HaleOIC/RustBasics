# README

## my first borrow

```shell
(py38) ➜  my_first_borrow git:(main) ✗ autotest
Found cargo project: my_first_borrow
/tmp/tmp.w3RTSflol5
Located autotests for my_first_borrow
cargo build --target-dir target # crate.tar
   Compiling my_first_borrow v0.1.0 (/tmp/tmp9vi3ztpq/autotest)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.27s
Test 0 (./target/debug/my_first_borrow) - passed
1 tests passed 0 tests failed
```

## Pusheen

```rust
/**
 * Reason for this program can not compile:
 *  first of all, declare vec as a mutable variable, and then make a reference of it
 *  by using new notation a, but it can not be borrowed twice using another new notation
 *  b, which is illegal.
 *
 * Fix:
 *  Just exchange the order of commands, this will tell compiler that `a` has finished
 *  his work and can be borrowed to `b`.
 */
```


## Annotate Lifetimes
```shell
 *  Executing task: cargo test --doc --package annotate_lifetimes -- identity --show-output 

    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.00s
   Doc-tests annotate_lifetimes

running 1 test
test src/lib.rs - identity (line 4) ... ok

successes:

successes:
    src/lib.rs - identity (line 4)

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 1 filtered out; finished in 0.12s

    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.00s
   Doc-tests annotate_lifetimes

running 1 test
test src/lib.rs - split (line 16) ... ok

successes:

successes:
    src/lib.rs - split (line 16)

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 1 filtered out; finished in 0.13s
```

## Typed Lifetimes
```shell
 *  Executing task: cargo test --package type_lifetimes --lib -- tests --show-output 

   Compiling type_lifetimes v0.1.0 (/home/hale/self-learning/UNSW_COMP6991/labs/lab03/type_lifetimes)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.37s
     Running unittests src/lib.rs (target/debug/deps/type_lifetimes-99fc227031f029fa)

running 1 test
test tests::main ... ok

successes:

successes:
    tests::main

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```


## Build A (christmas) Tree with Boxes

```shell
(py38) ➜  christmas_tree git:(main) ✗ autotest
Found cargo project: christmas_tree
/tmp/tmp.Sc2FJ5avSw
Located autotests for christmas_tree
cargo build --target-dir target # crate.tar
   Compiling proc-macro2 v1.0.44
   Compiling quote v1.0.21
   Compiling unicode-ident v1.0.4
   Compiling syn v1.0.100
   Compiling serde_derive v1.0.145
   Compiling serde v1.0.145
   Compiling bitflags v1.3.2
   Compiling base64 v0.13.0
   Compiling ron v0.8.0
   Compiling christmas_tree v0.1.0 (/tmp/tmpaopso0h9/autotest)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 4.91s
Test 0 (./target/debug/christmas_tree) - passed
Test 1 (./target/debug/christmas_tree) - passed
Test 2 (./target/debug/christmas_tree) - passed
Test 3 (./target/debug/christmas_tree) - passed
Test 4 (./target/debug/christmas_tree) - passed
5 tests passed 0 tests failed
```

**The following exercises do not have corresponding autotest.**

## First Home Buyers

```shell
(py38) ➜  first_home_buyers git:(main) ✗ cargo run
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/first_home_buyers`
House1 is owned by: Individual(Person { name: "John", age: 34 })
House2 is owned by: Bank("Bank of Melbourne")
House3 is owned by: Bank("Bank of Melbourne")
```

## Pokedex and Lifetimes!
```shell
(py38) ➜  pokedex git:(main) ✗ cargo run
   Compiling serde v1.0.203
   Compiling itoa v1.0.11
   Compiling ryu v1.0.18
   Compiling serde_json v1.0.117
   Compiling pokedex v0.1.0 (/home/hale/self-learning/UNSW_COMP6991/labs/lab03/pokedex)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 2.40s
     Running `target/debug/pokedex`
wit
sha
フシギ
妙蛙
Clawitzer            : wit;
Oshawott             : sha;
Musharna             : sha;
Mienshao             : sha;
Bisharp              : sha;
Marshadow            : sha;
Bulbasaur            : フシギ; 妙蛙;
Ivysaur              : フシギ; 妙蛙;
Venusaur             : フシギ; 妙蛙;
```

## My Caesar 

```shell
(py38) ➜  my_caesar git:(main) ✗ cargo run
   Compiling my_caesar v0.1.0 (/home/hale/self-learning/UNSW_COMP6991/labs/lab03/my_caesar)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.20s
     Running `target/debug/my_caesar`
Dr. Taylor Swift is the worlds greatest musician!
Shifted ascii by 5 is: Iw. Yfdqtw Xbnky nx ymj btwqix lwjfyjxy rzxnhnfs!
(py38) ➜  my_caesar git:(main) ✗ cargo run -- 10
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/my_caesar 10`
And I know it's long gone and 👋 That magic's not here no more 😭🫀🇧​🇷​🇪​🇦​🇰
Shifted ascii by 10 is: Kxn S uxyg sd'c vyxq qyxo kxn 👋 Drkd wkqsm'c xyd robo xy wybo 😭🫀🇧​🇷​🇪​🇦​🇰
T_T crying
Shifted ascii by 10 is: D_D mbisxq
```
