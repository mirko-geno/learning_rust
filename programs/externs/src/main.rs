// To implement functions from other languages the 'extern' keyword is used in this way:
extern "C" {
    fn abs(input: i32) -> i32;
}
fn main() {
    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }
}


// In order for other languages to call Rust functions:
// #[no_mangle] is a must for telling Rustc not to mangle its name
#[no_mangle]
pub extern "C" fn call_from_c() {
    println!("Just called a Rust function from C!");
}