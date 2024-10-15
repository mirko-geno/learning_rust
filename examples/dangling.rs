fn dangle() -> &String {
    let s = String::from("hello");
    &s //  returns a reference to data owned by the current function so it's an error
}


fn main() {
    let _reference_to_nothing = dangle();
}
