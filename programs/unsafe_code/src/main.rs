// inmutable and mutable raw pointers, dereferencing must be done
// inside an unsafe block of code

fn main() {
    let mut num: i32 = 5;
    let r1 = &num as *const i32;
    let _r2 = &mut num as *mut i32;

    // Doing this is possible but not recommended:
    /*
    let address = 0x012345usize;
    let r = address as *const i32;    
    */

    unsafe {
        println!("num: {}, r1 adress: {:?}, r1 content: {}", num, r1, *r1);
    }


}