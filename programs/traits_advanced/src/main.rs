use traits_advanced::Point;

fn main() {
    let p1 = Point::new(2.0, 5.0);
    let p2 = Point::new(10.0, 3.0);

    let p3 = p1 + p2;
    println!("Point 3 x: {}, y: {}", p3.x, p3.y);
}