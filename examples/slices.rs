fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}

fn main(){
    let string: String = String::from("Mirko es mi nombre");
    let slice: &str = first_word(&string);
    println!("{slice}")
}
