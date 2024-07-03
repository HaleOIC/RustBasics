# README

## Task 1

**Requirement:** Implement the find function, such that it takes an i32, and returns the first index of the vec that has a matching number, or -1 otherwise. Ensure that you can configure the number of threads that your program uses.

**Answer:**

```shell
(base) ➜  ws08 git:(main) ✗ cargo run
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.01s
     Running `target/debug/viscose`
Test find_normal: 19.24ms
Test find_parallel: 7.44ms
```

## Task 2

**Requirement:** Once you've implemented find, test how large the vec has to be before it's faster to use the parallel version compared to the inbuilt version. Try and figure out what the optimal number of threads to use is before it slows down again. Try and explain your results (and see if they're the same on somebody else's computer)

**Answer:**
I have tried `thread_num` with 6 possible options: `2, 4, 8, 16, 32, 64`. The best choice of this is `32`, double times the core of my computer:

```shell
(base) ➜  ws08 git:(main) ✗ cargo run
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.01s
     Running `target/debug/viscose`
Test find_normal: 18.28ms
Test find_parallel: 6.74ms
```

## Task 3

**Requirement:** Implement the find_all function, such that it takes a number, and returns all indexes that have the matching number.

**Answer:**
Have fullfilled in the code and then write some test:

```rust
fn find_all(&self, search_for: T) -> Vec<usize> {
    let chunk_size = self.iter.len() / NUM_THREADS;
    let result = Arc::new(Mutex::new(Vec::new()));

    std::thread::scope(|scope| {
        for (index, chunk) in self.iter.chunks(chunk_size).enumerate() {
            let result = Arc::clone(&result);
            let chunk = chunk.to_vec();

            scope.spawn(move || {
                let mut res = Vec::new();
                for (i, &num) in chunk.iter().enumerate() {
                    if num == search_for {
                        res.push(index * chunk_size + i);
                    }
                }
                let mut result = result.lock().unwrap();
                result.extend(res);
            });
        }
    });
    Arc::try_unwrap(result)
        .expect("Lock still has multiple owners")
        .into_inner()
        .unwrap()
}
```

```shell
(base) ➜  ws08 git:(main) ✗ cargo test
running 4 tests
test tests::test_find_large ... ok
test tests::test_performance_find ... ok
test tests::test_performance_find_all ... ok
test tests::test_find_all_large ... ok

test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.24s
```

## Task 4

**Requirement:** Convert your code such that it works for a vector of any type (not just i32). You will need to use generics, and to ensure some of your generics implement the appropriate traits.

**Answer:**

Add the following generic parameter into the code block and then convert it to more general version of code

```rust
struct ParallelIterator<T> {
    iter: Vec<T>,
}

// acaquire current number of cpus
static NUM_THREADS: usize = 32;

impl<T> ParallelIterator<T>
where
    T: Send + Sync + Copy + PartialEq + std::fmt::Debug,
```

## Task 5

**requirement:** Implement the map function such that you can write a closure which maps from a T to another T, and apply that to every element of the vector in parallel.

**Answer:**

Use the following codes to convert map function result from `type T` to `type U`:

```rust
fn map<U, F>(&self, f: F) -> Vec<U>
    where
        U: Send + Sync + Copy + PartialEq + std::fmt::Debug,
        F: Fn(T) -> U + Send + Sync + Copy,
```
