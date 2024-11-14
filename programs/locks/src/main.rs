use std::{clone, thread};
use std::sync::Mutex;


fn sum_one(m: Mutex<i32>) {
    let mut num = m.lock().unwrap();
    *num += 1;
}

fn main() {
    let counter = Mutex::new(0);

    for _ in 0..10 {
        thread::spawn(|| sum_one(counter));

    }







    let m = Mutex::new(5);

    {
        let mut num = m.lock().unwrap();
        *num = 6;
    }
    println!("m = {m:?}");
}
