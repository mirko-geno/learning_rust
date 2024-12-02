/*
Rust determines that using mutable static variables is unsafe because
they can be modified from different points of the program at the same 
time leading to a data race. Concurrency techniques and
thread-safe smart pointers are used to avoid this problems. 
*/
static mut COUNTER: i32 = 0;

fn add_to_counter(quant: i32) {
    unsafe {
        COUNTER += quant;
    }    
}


fn main() {
    add_to_counter(5);

    unsafe {
        println!("Counter: {COUNTER}");
    }
}