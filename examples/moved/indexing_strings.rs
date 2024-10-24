fn main() {
    let s: String = String::from("Mirko");
    // The following end up in an error because Strings can't be indexed this way
    // let ch = s[0]
    // Correct way:
    let ch = &s[..3];
    println!("{ch}");

    // For iterating over String type and get each character
    for ch in s.chars() {
        println!("{ch}");
    }

    // For getting their contents as bytes
    for ch in s.bytes() {
        println!("{ch}")
    }
}