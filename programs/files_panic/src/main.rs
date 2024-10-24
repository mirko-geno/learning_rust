use std::{fs::File, io::ErrorKind};


/*

The following are two ways to solve the same problem:
fn main(){
    let path: &str = "hi.txt";

    let file = match File::open(path) {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create(path) {
                Ok(file) => file,
                Err(err) => {panic!("Persistant error while creating the file {err:?}");}
            },
            other_error => panic!("Problem opening the file: {other_error:?}")
        }
    };
}
*/
fn main() {
    let _file = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {error:?}");
            })
        } else {
            panic!("Problem opening the file: {error:?}");
        }
    });
}