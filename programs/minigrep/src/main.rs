use std::{env, fs};


fn main() {
    let args: Vec<String> = env::args().collect();
    dbg!(&args);

    // Since &args[0] is the location of the binary we start taking
    // values from &args[1]
    let query = &args[1];
    let file_path = &args[2];

    println!("Searching for {query}");
    println!("In file {file_path}");

    let content = fs::read_to_string(file_path)
                                .expect("Couldn't be able to read the file");
    println!("With text:\n{content}");
}