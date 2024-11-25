use trait_objects::{Shape, Circle, Rectangle};

/*
Since we want to use the print_shape_info function with Rectangle AND Circle,
instead of explicitely making two functions for each type, Rust allows creating a 
function that receives a trait object (a parameter that implements certain trait).
The 'dyn' keyword is a must to achieve this because it indicates that the parameter
is of an unkwnown size in order to allow recieving non equal object types.

When using generics the compiler makes static dispatch, this means that all
the implementations made in the generic code will be generated for each nongeneric
type that received it.

On the other hand, when using trait objects, the dispatch is dynamic, meaning that
this process occurs on runtime instead of compile time, losing performance. This can
be fixed by limitating the type a generic parameter can receive to only one that 
implements certain trait.

The following commented code shows a function that uses a trait object, it isn't 
efficient so below it there's a code that accomplishes the same functionality 
without losing efficiency.
*/

/*
fn print_shape_info(shape: &dyn Shape) {
    println!("Shape: {}, Area: {}", shape.name(), shape.area());
}
*/

fn print_shape_info<T: Shape>(shape:&T) {
    println!("Shape: {}, Area: {}", shape.name(), shape.area());
}


fn main() {
    let circle = Circle { radius: 3.0 };
    let rectangle = Rectangle { width: 4.0, height: 5.0 };

    print_shape_info(&circle);
    print_shape_info(&rectangle);

    let shapes: Vec<Box<dyn Shape>> = vec![
        Box::new(Circle { radius: 3.0 }),
        Box::new(Rectangle { width: 4.0, height: 5.0 }),
    ];

    println!("Treating shapes as iterator");
    for shape in shapes {
        println!("Shape: {}, Area: {}", shape.name(), shape.area());
    }
}
