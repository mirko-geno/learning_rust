// The following codes won't compile


fn dangling_ref() -> &String {
    let s = String::from("hello");
    &s //  returns a reference to data owned by the current function so it's an error
}


fn dangling_lifetime<'a>(x: &str, y: &str) -> &'a str {
    let result = String::from("really long string");
    result.as_str()
    }

fn main() {
    let _reference_to_nothing = dangle();

    let name1: String = String::from("Farfa");
    let name2: String = String::from("Posho");

    let result = dangling_lifetime(name1, name2);
}
