fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}


fn main() {
    let five = Some(5);
    let six = plus_one(five);
    println!("{:?}", six);
    let _none = plus_one(None);
}