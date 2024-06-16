use std::ops::Deref;

struct Inner<T> {
    refcount: usize,
    data: T
}

pub struct MyRc<T> {
    inner: *mut Inner<T>
}

impl<T> MyRc<T> {
    pub fn new(value: T) -> Self {
        // TODO: Create a MyRc. You will need to:
        //  - use Box::into_raw to create an Inner
        //  - set refcount to an appropriate value.
        todo!()
    }
}

impl<T> Clone for MyRc<T> {
    fn clone(&self) -> Self {
        // TODO: Increment the refcount,
        // and return another MyRc<T> by copying the
        // inner struct of this MyRc.
        todo!()
    }
}

impl<T> Drop for MyRc<T> {
    fn drop(&mut self) {
        // TODO: Decrement the refcount..
        // If it's 0, drop the Rc. You will need to use
        // Box::from_raw to do this.
        todo!()
    }
}

impl<T> Deref for MyRc<T> {
    type Target = T;

    fn deref(&self) -> &T {
        // TODO: Return a &T.
        todo!()
    }
}
