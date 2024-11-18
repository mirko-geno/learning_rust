/*
The use of functions is a very good way to avoid deadlocks
due to the fact that the lock of a variable is dropped
when it goes out of scope, so a variable locked inside
a function called by a threead rather than itself will be
locked only during the execution of the function.

In the example below using the function sum_one avoids a
deadlock as described before.
*/

use std::{clone, thread};
use std::sync::{Mutex, Arc};
use std::time::Duration;

fn sum_one(arc_ref: &Arc<Mutex<i32>>) {
    let mut num = arc_ref.lock().unwrap();
    *num += 1;
    println!("Value number after sum is {}", *num);
}


pub fn deadlock() {
    let var1 = Arc::new(Mutex::new(12));
    let var2 = Arc::new(Mutex::new(24));

    let ref1 = Arc::clone(&var1);
    let ref2 = Arc::clone(&var2);

    let handle1 = thread::spawn(move || {
        sum_one(&ref1);
        // let mut num = ref1.lock().unwrap();
        // *num += 1;
        thread::sleep(Duration::from_secs(2));
        
        sum_one(&ref2);
        // let mut num = ref2.lock().unwrap();
        // *num += 1;
        thread::sleep(Duration::from_secs(5));
    });

    let ref3 = Arc::clone(&var1);
    let ref4 = Arc::clone(&var2);

    let handle2 = thread::spawn(move|| {
        sum_one(&ref4);
        // let mut num = ref4.lock().unwrap();
        // *num += 1;
        thread::sleep(Duration::from_secs(2));
        
        sum_one(&ref3);
        // let mut num = ref3.lock().unwrap();
        // *num += 1;
        thread::sleep(Duration::from_secs(5));
    });

    handle1.join().unwrap();
    handle2.join().unwrap();
}