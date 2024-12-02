use traits_advanced::{Point, Millimeters, Meters};

fn main() {
    let p1 = Point::new(2.0, 5.0);
    let p2 = Point::new(10.0, 3.0);

    let p3 = p1 + p2;
    println!("Point 3 x: {}, y: {}", p3.x, p3.y);


    let meters = Meters::new(2);
    let millis = Millimeters::new(250);

    let result_in_millis = millis + meters;

    println!("Result in millis: {}", result_in_millis.value());

}