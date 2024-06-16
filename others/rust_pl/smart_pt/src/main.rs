fn main() {
    box_demo();
    sp_demo();
    // rc_demo();
    refcell_demo();
}

//////////////////  Box Demo //////////////////

use std::{cell::RefCell, ops::Deref, rc};

fn box_demo() {
    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y)
}

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> Self {
        Self(x)
    }
}

// Without the Deref trait, the compiler can only dereference & references.
// The deref method gives the compiler the ability to take a value of any type
// that implements Deref and call the deref method to get a & reference that
// it knows how to dereference.
impl<T> Deref for MyBox<T> {
    // define the ascociated type for Deref trait
    type Target = T;
    fn deref(&self) -> &Self::Target {
        // `0` means the first element of the tuple
        &self.0
    }
}

//////////////////  Box Demo //////////////////

//////////  Custom Smarter pointer Demo ////////////
fn sp_demo() {
    // Variables are dropped in the reverse order of their creation
    let mpt = MySmartPointer {
        data: String::from("my stuff"),
    };
    let _mpt2 = MySmartPointer {
        data: String::from("other stuff"),
    };

    println!("MySmartPointer created.");
    drop(mpt);
    println!("MySmartPointer dropped before the end of main.");
}

struct MySmartPointer {
    data: String,
}

impl Drop for MySmartPointer {
    fn drop(&mut self) {
        println!("Dropping MySmartPointer with data `{}`", self.data);
    }
}

//////////  Custom Smarter pointer Demo ////////////

///////// Refence Counting Demo ////////////
// enum List {
//     Cons(i32, Rc<List>),
//     Nil,
// }
// use crate::List::{Cons, Nil};
// use std::rc::Rc;

// fn rc_demo() {
//     // Rc::clone doesn't make a deep copy of all the data like clone does.
//     // The call to Rc::clone only increments the reference count,
//     // which doesn’t take much time.
//     let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
//     println!("count after creating a = {}", Rc::strong_count(&a));
//     let _b = Cons(3, Rc::clone(&a));
//     println!("count after creating b = {}", Rc::strong_count(&a));
//     {
//         let _c = Cons(4, Rc::clone(&a));
//         println!("count after creating c = {}", Rc::strong_count(&a));
//     }
//     println!("count after c goes out of scope = {}", Rc::strong_count(&a));
// }

///////// Refence Counting Demo ////////////

///////// RefCell Demo ///////
///
use crate::List::{Cons, Nil};
use std::rc::Rc;

#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}

fn refcell_demo() {
    let value = Rc::new(RefCell::new(5));

    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));

    let b = Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));
    let c = Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));

    *value.borrow_mut() += 10;

    println!("a after = {a:?}");
    println!("b after = {b:?}");
    println!("c after = {c:?}");
}
///////// RefCell Demo ///////
