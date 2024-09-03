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
        MyRc {
            inner: Box::into_raw(Box::new(Inner {
                refcount: 1,
                data: value
            }))
        }
    }
}

impl<T> Clone for MyRc<T> {
    fn clone(&self) -> Self {
        unsafe {
            (*self.inner).refcount += 1;
        }
        MyRc { inner: self.inner }
    }
}

impl<T> Drop for MyRc<T> {
    fn drop(&mut self) {
        unsafe {
            (*self.inner).refcount -= 1;
            if (*self.inner).refcount == 0 {
                drop(Box::from_raw(self.inner));
            }
        }
    }
}

impl<T> Deref for MyRc<T> {
    type Target = T;

    fn deref(&self) -> &T {
        unsafe {
            &(*self.inner).data            
        }
    }
}
