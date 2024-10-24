use std::fs::File;
use std::io::{self, Read};

/* 
Handling errors can be done in different ways using match or if let statements,
but the '?' operator brings in a simpler approach. If everything works,
the result is bound to what is was supposed to; if instead, an error appears,
it is propagated, i.e. returned from the current scope.


fn read_username_from_file(path: &str) -> Result<String, io::Error> {
    let username_file_result = File::open(path);
    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };
    let mut username = String::new();
    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}


fn read_username_from_file(path: &str) -> Result<String, io::Error> {
    let mut username_file = File::open(path)?;
    let mut username: String = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}
*/


fn read_username_from_file(path: &str) -> Result<String, io::Error> {
    let mut username: String = String::new();

    File::open(path)?.read_to_string(&mut username)?;
    Ok(username)
}

fn main() {
    let user = read_username_from_file("programs/error_handling/username.txt");
    println!("{user:?}");
}