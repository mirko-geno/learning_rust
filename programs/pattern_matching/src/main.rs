// Pattern matching consists in comparing a value to a pattern, if they match
// a part of code executes allowing the use of that value.


// Parameters received by a function is pattern matching as well:
fn print_coordinates(&(x, y): &(i32, i32)) {
    println!("Current location: ({x}, {y})");
    }


fn main() {
    // The following two variants of pattern matching are two ways to do the same,
    // while if let can be more readable, match ensures exhaustive checking

    let config_max = Some(3u8);
    match config_max {
        Some(max) => println!("The maximum is configured to be {max}"),
        _ => (),
    }
    
    let config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("The maximum is configured to be {max}");
    }

    //while let and for loops are used for pattern matching and are quite the same:
    let mut stack = vec!(1, 2, 3);
    while let Some(top) = stack.pop() {
        println!("Pop {top}");
    }

    let v = vec!['a', 'b', 'c'];
    for (index, value) in v.iter().enumerate() {
        println!("{value} is at index {index}");
    }

    let point = (3, 5);
    print_coordinates(&point);

    // Multiple pattern matching:
    let number = 2;
    match number {
        1 | 2 => println!("Number is 1 or 2"),
        3 => println!("Number is 3"),
        _ => println!("anything")
    }

    let number = 5;
    match number {
        1..= 5 => println!("Number is between 1 and 5"),
        _ => println!("Whatever")
    }

    // Pattern matching can also be used to destructure structs, tuples, enums, etc
    struct Point {
        x: i32,
        y: i32,
    }

    let p = Point { x: 0, y: 7 };
    let Point { x: a, y: b } = p;

    assert_eq!(0, a);
    assert_eq!(7, b);

    // Better approach
    let Point { x, y } = p;
    assert_eq!(0, x);
    assert_eq!(7, y);

    match p {
        Point {x, y: 0} => println!("On the x axis at {x}"),
        Point {x: 0, y: 1..=10} => println!("On the y axis between 1 and 10 ({y})"),
        Point { x: 0, y } => println!("On the y axis at {y}"),
        Point { x, y } => println!("On neither axis: ({x}, {y})")
    }
}
