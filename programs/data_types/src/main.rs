use std::rc::Rc;

// For constant variables (same as C #DEFINES)
const BANANA: u8 = 4;

// 'type' keyword creates an alias based on a datatype
type Kilometers = i32;

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


    // Structs can have any quantity of fields of any type 
    struct MyStruct<T> {
        field0: String,
        field1: T,
        field2: T
    }

    // To instantiate a struct
    let my_struct = MyStruct {
        field0: String::from("Mirko"),
        field1: 5,
        field2: 8
    };


    // Hash maps work like dictionaries but can only store one data type at the same time
    use std::collections::HashMap;

    let mut map = HashMap::new();
    map.insert(String::from("Key"), String::from("Value"));


    // Option enum is useful to safely asume value we are working with aren't null
    // because null values will fall under None. 
    enum Option<T> {
        Some(T), // T stand for any data type that falls as Some's content
        None 
    }

    // Result enum lets us distinguish between data and
    // errors making the programmer able to handle them
    enum Result<T, E> {
        Ok(T),
        Err(E) // E stands for any error
    }


    // Box<T> stores a kind of data into the heap
    let b = Box::new(30);

    // In order to access or modify it's content '*' is used
    let mut b = Box::new(12);
    *b *= 2;
    assert_eq!(24, *b);

    
    // Raw pointers (inmutable and mutable)
    let mut num: i32 = 5;
    let _r1 = &num as *const i32;
    let _r2 = &mut num as *mut i32;


    // Global variables are called static, and are
    // a reference of 'static lifetime
    static HELLO_WORLD: &str = "Hello World";


    // All fields of an union share common storage 
    union MyUnion {
        field0: u32,
        field1: f32
    }

    let my_union = MyUnion { field0: 25 };

    // Accessing an union's field is unsafe
    let field = unsafe {my_union.field0};
}