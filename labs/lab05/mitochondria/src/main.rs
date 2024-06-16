use std::cell::Cell;

fn main() {
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
}
