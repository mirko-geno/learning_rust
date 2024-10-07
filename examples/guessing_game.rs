use std::io;

fn main(){
    println!("<<<<<Welcome to the guessing game!>>>>>\nEnter a number between 1 and 100 to start!");
    println!("Please enter your guess");

    let mut guess: String = String::new();
    io::stdin().read_line(&mut guess)
        .expect("Failed to read line");

    if guess == "4"{
        println!("You guessed {}", guess);
    }
}