/*
When working with vectors if we index regularly and try to access an element which is out of bounds the program panics
during execution. If instead we use the "get" method from Vec, we are able to handle the possibility using 
Option<&T> enum and match or if let operators
*/


struct Car {
    name: String,
    _motor: String,
    _top_speed: u16
}

impl Car {
    fn new(name: &str, motor: &str, _top_speed: u16) -> Car {
        Car {
            name: String::from(name),
            _motor: String::from(motor),
            _top_speed
        }
    }
}


enum SpreadsheetCell {
    Int(i32),
    Float(f32),
    Text(String)
}


fn main() {
    // Regular usage of vectors with only one datatype
    let mut vect: Vec<Car> = vec![Car::new("Mitsubishi Eclipse", "Chrysler 420A", 253)];
    vect.push(Car::new("Lamborghini Aventador", "L539 V12", 350));

    let car_ref: &Car = &vect[1];
    println!("My favourite car is {}", car_ref.name);

    let sec_car_ref: Option<&Car> = vect.get(0);
    match sec_car_ref {
        Some(car) => println!("My brother's favourite car is {}", car.name),
        None => println!("Indexing out of bounds")
    }

    let row: Vec<SpreadsheetCell> = vec![
        SpreadsheetCell::Int(20),
        SpreadsheetCell::Float(-54.21),
        SpreadsheetCell::Text(String::from("Hola me llamo Mirko"))
    ];

    match row.get(0) {
        Some(data) => {
            match data {
                SpreadsheetCell::Int(value) => println!("Value is {value}"),
                SpreadsheetCell::Float(value) => println!("Value is {value}"),
                SpreadsheetCell::Text(value) => println!("Value is {value}"),
            }
        },
        None => ()
    }
}