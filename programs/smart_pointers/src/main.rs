use std::rc::Rc;

use smart_pointers::{CustomSmartPointer, List::{Cons, Nil}};

fn hello(name: &str) {
    println!("Hello {name}!");
}

fn main() {
    let _cons_list = Cons(1, Rc::new(Cons(2, Rc::new(Cons(3, Rc::new(Nil))))));

    let x: i32 = 5;
    let r: &i32 = &x;
    let b: Box<i32> = Box::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *r);
    assert_eq!(5, *b);

    let boxed_n = Box::new(String::from("Mirko"));
    hello(&boxed_n);

    let _d = CustomSmartPointer { data: String::from("my stuff") };

    let cd = CustomSmartPointer { data: String::from("some data") };
    println!("CustomSmartPointer created.");
    drop(cd);
    println!("CustomSmartPointer dropped before the end of main.");


    //Rc::clone creates a shallow copy and increments the reference counter
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("count after creating a: {}", Rc::strong_count(&a));
    let _b = Cons(3, Rc::clone(&a));
    println!("count after creating b: {}", Rc::strong_count(&a));
    {
        let _c = Cons(4, Rc::clone(&a));
        println!("count after creating c: {}", Rc::strong_count(&a));
    }
    println!("count after c goes out of scope: {}", Rc::strong_count(&a));

}