// 'fn' keyword creates a pointer to function, so, they can be
// passed as a parameter to other functions.

fn add_one(x: u32) -> u32 {
    x + 1
}

fn do_twice(f: fn(u32) -> u32, arg: u32) -> u32 {
    f(arg) + f(arg)
}

#[allow(dead_code)]
#[derive(Debug)]
enum Status {
    Value(u32),
    Stop
}


/*
Closures can't be returned normally, they must be inside a Box type

fn returns_closure() -> dyn Fn(i32) -> i32 {
    |x| x + 1
}
*/

fn returns_closure() -> Box<dyn Fn(i32) -> i32> {
    Box::new( |x| x + 1)
}


fn main() {
    let result = do_twice(add_one, 5);
    assert_eq!(12, result);

    let vec = vec![1, 2, 3];
    let vec_of_strings: Vec<String> = vec.iter().map( |i| i.to_string()).collect();
    
    let vec_of_strings2: Vec<String> = vec.iter().map(ToString::to_string).collect();
    
    assert_eq!(vec!["1", "2", "3"], vec_of_strings);
    assert_eq!(vec!["1", "2", "3"], vec_of_strings2);

    
    // Since each variant of an enum is an initializer function, we can
    // map it in this way instead of using a closure
    let list_of_values: Vec<Status> = (0u32..20).map( |value| Status::Value(value)).collect();
    let list_of_values2: Vec<Status> = (0u32..20).map(Status::Value).collect();

    println!("{:?}", list_of_values);
    println!("{:?}", list_of_values2);
}
