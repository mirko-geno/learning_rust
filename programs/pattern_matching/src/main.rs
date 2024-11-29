/*
Pattern matching consists in comparing a value to a pattern, if they match
a part of code executes allowing the use of that value.


The following two variants of pattern matching are two ways to do the same,
while if let can be more readable, match ensures exhaustive checking
*/

fn main() {
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
}
