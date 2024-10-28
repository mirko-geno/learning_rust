fn get_longest<'a>(str1: &'a str, str2: &'a str) -> Option<&'a str> {
    if str1.len() > str2.len() {
        Some(str1)
    }else if str2.len() > str1.len(){
        Some(str2)
    }else {
        None
    }
}


fn print_longest<'a>(str1: &'a str, str2: &'a str) {
    match get_longest(str1, str2) {
        Some(name) => println!("The longest string is {name}"),
        None => println!("The two names have the same length")
    }
}


fn main() {
    let name1: String = String::from("Farfa");
    let name2: String = String::from("Posho");
    let name3: String = String::from("Piru");

    print_longest(&name1, &name2);
    print_longest(&name1, &name3);
}
