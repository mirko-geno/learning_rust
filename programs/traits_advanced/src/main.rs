use traits_advanced::{Animal, Dog, Meters, Millimeters, OutlinePrint, Point};

fn main() {
    let p1 = Point::new(2.0, 5.0);
    let p2 = Point::new(10.0, 3.0);

    let p3 = p1 + p2;
    println!("Point 3 x: {}, y: {}", p3.x, p3.y);


    let meters = Meters::new(2);
    let millis = Millimeters::new(250);

    let result_in_millis = millis + meters;

    println!("Result in millis: {}", result_in_millis.value());

    println!("A baby dog is called a {}", Dog::baby_name());

    /*
    In general, fully qualified syntax is defined as follows,
    it is used when we have to call an specific method which's name
    is equal from an implemented one by the data type:
    <Type as Trait>::function(receiver_if_method, next_arg, ...);
    */
    println!("A baby dog is called a {}", <Dog as Animal>::baby_name());
    
    // Point implementing fmt::Display and OutlinePrint
    p3.outline_print();

}