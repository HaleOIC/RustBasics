# README

### Tribonacci 
```shell
labs/lab02/tribonacci on  main [?] is 📦 v0.1.0 via 🦀 v1.77.1
❯ autotest
Found cargo project: tribonacci
/tmp/tmp.NCAyjeMD98
Located autotests for tribonacci
cargo build --target-dir target # crate.tar
   Compiling tribonacci v0.1.0 (/tmp/tmp81gmvwtl/autotest)
    Finished dev [unoptimized + debuginfo] target(s) in 0.55s
Test 0 (./target/debug/tribonacci) - passed
Test 1 (./target/debug/tribonacci 3) - passed
Test 2 (./target/debug/tribonacci 8) - passed
Test 3 (./target/debug/tribonacci 10) - passed
Test 4 (./target/debug/tribonacci 140) - passed
Test 5 (./target/debug/tribonacci 145) - passed
Test 6 (./target/debug/tribonacci -100) - passed
Test 7 (./target/debug/tribonacci hello) - passed
8 tests passed 0 tests failed
```


### To Upper 
```shell
labs/lab02/to_upper on  main [?] is 📦 v0.1.0 via 🦀 v1.77.1 took 2s
❯ autotest
Found cargo project: to_upper
/tmp/tmp.dPgAxltHgX
Located autotests for to_upper
cargo build --target-dir target # crate.tar
   Compiling to_upper v0.1.0 (/tmp/tmpvny0q109/autotest)
    Finished dev [unoptimized + debuginfo] target(s) in 0.57s
Test 0 (./target/debug/to_upper 'hello world!') - passed
Test 1 (./target/debug/to_upper '🥺👉👈') - passed
2 tests passed 0 tests failed
```


### Collections
```shell
labs/lab02/collections on  main [?] is 📦 v0.1.0 via 🦀 v1.77.1 took 5s
❯ cargo run
   Compiling collections v0.1.0 (/home/hale/UNSW_COMP6991/labs/lab02/collections)
    Finished dev [unoptimized + debuginfo] target(s) in 0.63s
     Running `target/debug/collections`
==== Vector ====
insert: 5.159287ms
remove: 4.54543906s
==== VecDeque ====
insert: 7.110916ms
remove: 4.845278ms
==== LinkedList ====
insert: 29.518921ms
remove: 15.083291ms
==== HashMap ====
insert: 293.060917ms
remove: 190.025866ms
```

### Data analysis
```shell
labs/lab02/data_analysis on  main [?] is 📦 v0.1.0 via 🦀 v1.77.1
❯ autotest
Found cargo project: data_analysis
/tmp/tmp.UTe8IBWVsk
Located autotests for data_analysis
cargo build --target-dir target # crate.tar
   Compiling serde v1.0.197
   Compiling memchr v2.7.2
   Compiling itoa v1.0.11
   Compiling ryu v1.0.17
   Compiling csv-core v0.1.11
   Compiling csv v1.3.0
   Compiling data_analysis v0.1.0 (/tmp/tmp12gofycm/autotest)
    Finished dev [unoptimized + debuginfo] target(s) in 3.85s
Test 0 (./target/debug/data_analysis) - passed
Test 1 (./target/debug/data_analysis) - passed
2 tests passed 0 tests failed
```
