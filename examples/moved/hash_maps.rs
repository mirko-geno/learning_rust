use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();
    map.insert("First", 10);
    
    if let Some(value) = map.get("First") {println!("Value: {value}");}

    match map.get("First") {
        Some(value) => println!("Value: {value}"),
        None => println!("No value bound to that key")
    }
}
