# Questions

1) I saw someone's code fail to compile because they
were trying to send non-thread-safe data across threads.
How does the Rust language allow for static (i.e. at compile time)
guarantees that specific data can be sent/shared acrosss threads?

> Answer: only the data have `send` and `sync` trait can be transfered across threads
> These two traits enforece ownership and borrowing rules, preventing data races.
> Trait `Send` is an auto one and should not be implemented by code designer.
> another fact is `send` does not mean `sync`, `sync` definition is any reference of data can be transferred safely across threads.

2) Do you have to then implement the Send and Sync traits for
every piece of data (i.e. a struct) you want to share and send across threads?

> No, actually, it does not. the complier can automatically derives these traits.
> Rust will automatically implement them if all the components of the struct also implement `Send` and `Sync`.

3) What types in the course have I seen that aren't Send? Give one example,
and explain why that type isn't Send

> Rc is not `Send`, cause it can not reference counting atomically. Non-atomic operations are not safe to use across multiple threads because they do not ensure mutual exclusion. This can lead to race conditions where multiple threads try to update the reference count simultaneously, causing undefined behavior or memory corruption.

4) What is the relationship between Send and Sync? Does this relate
to Rust's Ownership system somehow?

> Send allows data transfer between threads, while Sync allows data to be shared across threads. Both traits ensure Rust's ownership and borrowing rules are upheld, preventing data races and ensuring thread safety.

5) Are there any types that could be Send but NOT Sync? Is that even possible?

> Yes, it is possible for a type to be Send but not Sync. For example, `std::cell::RefCell<T>` is Send if T is Send, but it is not Sync.

6) Could we implement Send ourselves using safe rust? why/why not?

> Of course, it is just a marker trait, hos no methods need to be implemented.
