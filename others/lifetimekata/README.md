# LifetimeKata

Welcome to LifetimeKata, a set of exercises which you can use to improve your
understanding of lifetimes in Rust. While many tasks involve writing compiling
code, some will also involve creating specific errors.

You should complete the kata in order, as they increase in
difficulty, and depend on previous kata.

## Getting Started

Clone this repository:

``` sh
$ git clone https://www.github.com/tfpk/lifetimekata/
```

Most exercises are run in two steps:

``` sh
$ cargo build --package ex04
```

And then either:

``` sh
$ cargo test --package ex04
```

or:

``` sh
$ cargo run --package ex04
```

depending on whether it's a binary or a library.



## Conclusion(useful views about lifetime)

Related rules for lifetime elision:

1. Each place that an input lifetime is left out(a.k.a 'elided') is filled with its own lifetime.
2. If there's exactly one lifetime on all the input references, that lifetime is assigned to *every output lifetime*
3. If there are multiple input lifetime positions, but one of them is `&self` or `&mut self`, the lifetime of the borrow of `self` is assigned to all elided output lifetimes.

