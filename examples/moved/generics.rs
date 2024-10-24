/*
Generics allows differentiating available methods between datatypes.

In the example below, the contructor, ("new" method), is always available 
due to be of generic type 'T'. Besides this, the "hypo" method is only available when
an instantiated Point's datatype is f32.
*/

struct Point<T> {
    x: T,
    y: T,
}

impl <T> Point<T> {
    fn new(x: T, y: T) -> Point<T> {
        Point { x, y }
    }

}

impl Point<f32> {
    fn hypo(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}


fn get_largest<T: std::cmp::PartialOrd>(arr: &[T]) -> &T {
    let mut largest = &arr[0];

    for elem in arr {
        if elem > largest {
            largest = elem;
        }
    }
    largest
}


fn main() {
    let arr = [2, 4, 10, 5];
    println!("{}", get_largest(&arr));

    let point = Point::new(4.0, 2.5);
    println!("{}", point.hypo());

    /*
    The following code wouldn't compile because Point<i32>
    doesn't have a "hypo" method implemented:
    
    let point = Point::new(10, 5);
    println!("{}", point.hypo())
    */
}   