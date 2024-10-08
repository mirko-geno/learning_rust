use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main(){
    println!("<<<<<Welcome to the guessing game!>>>>>\nEnter a number between 1 and 100 to start!");
    let r_number: u8 = rand::thread_rng().gen_range(1..=100);
    let mut guess: String = String::new();
    let mut i_guess: u8;

    loop{
        println!("Please enter your guess");
        io::stdin().read_line(&mut guess).expect("Failed to read line");
        i_guess = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) =>{
                println!("Failed, {}", guess);
                continue}
        };
        guess.clear();

        match i_guess.cmp(&r_number){
            Ordering::Less => println!("Too Small"),
            Ordering::Greater => println!("Too Big"),
            Ordering::Equal => {
                println!("You Won!");
                break;
            }
        }
    }
}