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

    #[allow(dead_code)]
    enum MatchResult {
        Win(u32, u32),
        Lose(u32, u32),
        Draw
    }

    let result = MatchResult::Win(3, 0);
    match result {
        MatchResult::Win(w, l) => println!("Local team won! \nresult: {w} - {l}"),
        MatchResult::Lose(l, w) => println!("Local team lost! \n: {l} - {w}"),
        MatchResult::Draw => println!("Draw!")
    }


    #[allow(dead_code)]
    enum Color {
        Rgb(u8, u8, u8),
        Hsv(i32, i32, i32),
    }

    struct Screen {
        resolution: (u32, u32),
        color: Color
    }

    let screen = Screen {
        resolution: (1920, 1080),
        color: Color::Rgb(2, 219, 59)
    };

    match screen {
        Screen {resolution: (1920, 1080), color: Color::Rgb(_r, _g, _b)} => println!("The screen has the default specs"),
        _ => println!("The screen doesn't have the default specs")
    }


    // The '..' syntax allows ignoring a range of values
    #[allow(dead_code)]
    struct Point3D {
        x: i32,
        y: i32,
        z: i32
    }

    let p_3d = Point3D { x: 6, y: 7, z: 10 };

    match p_3d {
        Point3D { x: 0..=3, ..} => println!("X value between 0 and 3, x: {x}"),
        Point3D {x: 4..=10, ..} => println!("X bewtwwn 4 and 10, x: {x}"),
        Point3D {..} => println!("Any other case")
    }

    
    let numbers = (2, 4, 8, 16, 32);

    match numbers {
        (first, .., last) => println!("Some numbers: {first}, {last}")
    }


    // A match guard consists in using a conditional inside a match arm
    let num = Some(4);
    match num {
        Some(x) if x % 2 == 0 => println!("The number {x} is even"),
        Some(x) => println!("The number {x} is odd"),
        None => (),
    }

}
