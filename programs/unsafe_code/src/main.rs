unsafe fn dangerous() { }

#[allow(dead_code)]
fn fake_safe_function() {
    println!("This function is safe and contains an unsafe block of code inside");
    
    unsafe {
        let num = 0x7fff4f9e0b5cusize;
        let p = num as *const usize;
        println!("Number inside safe function is {num}, its address is {p:?}, and its content: {:?}", *p);
    }
}


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
        println!("num: {}, r1 address: {:?}, r1 content: {}", num, r1, *r1);
    }

    // calling unsafe functions must be done inside an unsafe block
    unsafe {
        dangerous();
    }


    let mut v = vec![1, 2, 3, 4, 5, 6];
    let r = &mut v[..];
    let (a, b) = r.split_at_mut(3);
    assert_eq!(a, &mut [1, 2, 3]);
    assert_eq!(b, &mut [4, 5, 6]);

    // calling a function that has an unsafe block inside doesn't need to be 
    // called via unsafe block but the compiler doesn't know if it has a memory problem
    // fake_safe_function();

}