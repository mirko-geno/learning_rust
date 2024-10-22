// For constant variables (same as C #DEFINES)
const BANANA: u8 = 4;

fn main(){
    // boolean types: true and false (one byte in size)
    let t= true;

    // int types: u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize
    let i: u8 = 24;

    // float types: f32 and f64 for double precission
    let f = 2.0; //infers f64 by default

    // char literals use single quotes and utf-8 encoding
    let ch: char = 'M';

    // Tuples
    let tup: (u8, f64, i16, String, (u8,f32)) = (2, 7.5, -2040, String::from("Banana"), (3, -83.2));
    // Tuple destructuring
    let (u_int, double, s_int, string, (sub_u_int, sub_float)) = tup;
    println!("{}", sub_u_int);

    // Arrays [fixed data type ; length]
    let arr: [f64; 5] = [264.2, 6445.0, -1758.0, 2.0, -87.3];
    let stablished_arr: [i32; 5] = [2; 5]; // [2, 2, 2, 2, 2]

    // String literals  Stored in binaries
    let string_literal: &str = "My name is Mirko";
    // Strings  Stored in stack: ptr to heap, length, capacity  Stored in heap: content 
    let string: String = String::from("My name is Mirko");
    // String slices    Stored in stack: ptr to starting heap point of String variable, length
    let string_slice: &str = &string[11..];

    // Vectors
    let v: Vec<u8> = Vec::new();
    // vectors can be instantiated using the macro vec! as follows:
    let vec = vec![4, 5, 6];
}