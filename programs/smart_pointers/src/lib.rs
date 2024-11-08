use std::rc::Rc;
use std::ops::{Deref, Drop};


pub struct CustomSmartPointer {
    pub data: String
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}


pub struct MyBox<T>(T);

impl <T> MyBox<T> {
    pub fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}


pub enum List {
    Cons(i32, Rc<List>),
    Nil
}