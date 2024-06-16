# README

## Pointy 

```shell
(py38) ➜  pointy git:(main) ✗ cargo test --doc
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.00s
   Doc-tests pointy

running 6 tests
test src/lib.rs - Point (line 31) ... ok
test src/lib.rs - Point<T>::distance (line 47) ... ok
test src/lib.rs - Point<T>::distance (line 54) ... ok
test src/lib.rs - first (line 5) ... ok
test src/lib.rs - Point (line 24) ... ok
test src/lib.rs - first (line 11) ... ok

test result: ok. 6 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.18s
```

## Dungeons and Rragons

```shell
(py38) ➜  dungeons_and_dragons git:(main) ✗ autotest
Found cargo project: dungeons_and_dragons
/tmp/tmp.CybXj1Cxfa
Located autotests for dungeons_and_dragons
cargo build --target-dir target # crate.tar
   Compiling libc v0.2.134
   Compiling cfg-if v1.0.0
   Compiling io-lifetimes v0.7.3
   Compiling bitflags v1.3.2
   Compiling rustix v0.35.11
   Compiling smallvec v1.10.0
   Compiling log v0.4.17
   Compiling linux-raw-sys v0.0.46
   Compiling memchr v2.5.0
   Compiling endian-type v0.1.2
   Compiling ppv-lite86 v0.2.16
   Compiling unicode-segmentation v1.10.0
   Compiling utf8parse v0.2.0
   Compiling unicode-width v0.1.10
   Compiling nibble_vec v0.1.0
   Compiling radix_trie v0.2.1
   Compiling getrandom v0.2.7
   Compiling dirs-sys-next v0.1.2
   Compiling nix v0.24.2
   Compiling dirs-next v2.0.0
   Compiling rand_core v0.6.4
   Compiling rand_chacha v0.3.1
   Compiling rand v0.8.5
   Compiling fd-lock v3.0.6
   Compiling rustyline v10.0.0
   Compiling dungeons_and_dragons v0.1.0 (/tmp/tmptx3xcvns/autotest)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 3.12s
Test 0 (./target/debug/dungeons_and_dragons) - passed
Test 1 (./target/debug/dungeons_and_dragons) - passed
Test 2 (./target/debug/dungeons_and_dragons) - passed
Test 3 (./target/debug/dungeons_and_dragons) - passed
Test 4 (./target/debug/dungeons_and_dragons) - passed
Test 5 (./target/debug/dungeons_and_dragons) - passed
6 tests passed 0 tests failed
```

## Lanugages 

```shell
(py38) ➜  languages git:(main) ✗ autotest
Found cargo project: languages
/tmp/tmp.tfWe2NjMrZ
Located autotests for languages
cargo build --target-dir target # crate.tar
   Compiling languages v0.1.0 (/tmp/tmp6akgwdld/autotest)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.20s
Test 0 (./target/debug/languages) - passed
1 tests passed 0 tests failed
```

## Mitochondria 

```rust
// Q) what is the type of my_cell
    // A) Cell<i32>

    // Q) is the my_cell type a mutable type?
    // A) No Cell<i32> itself is not mutable, but the value inside it is mutable
    let my_cell = Cell::new(5);

    // Q) what are the parameters of the set method below?
    // A) fn set<T>(&self, value: T)

    // Q) why does set not require a mutable reference?
    // A) Because Cell is not mutable, but the value inside it is mutable
    my_cell.set(6);

    // Q) what is interior mutability?
    // A) Interior mutability is a design pattern in Rust that allows you
    // to mutate data even when there are immutable references to that data

    // Q) what benefits do we as programmers get from interior mutability?
    // A) We can have multiple immutable references to the same data

    // Q) Is cell Safe? Why or why not?
    // A) Yes, Cell is safe because it enforces the borrowing rules at runtime

    // Q) Is it possible for Cell to be used in multiple threads? Why or why not?
    // A) No, Cell is not Sync, think this case, both threads want to set their own
    // version of value into the same cell, which one should be the winner?

    // Q) What is a good use case for Cell?
    // A) Cell is useful when you want to have multiple immutable references to the same data
    // especially for smaller data that easy to copy

    // Q) What is the difference between Cell and RefCell?
    // A) Cell is for Copy types, RefCell is for non-Copy types

    // Q) Did you find any other interesting observations?
    // A) Cell is not Sync, but RefCell is Sync

```
