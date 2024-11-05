/*
Closures are anonymous functions which can be saved into variables or be passed
as arguments to other functions. They can also work like Python's lambda to modify variables.
Move keyword is useful for passing a closure to a new thread making it the new owner of the data. 
*/
use std::thread;
use closures::{ShirtColor, Inventory};

fn square_list(list: Vec<i32>) {
    let list_squared: Vec<i32> = list.iter().map(|x| x*x).collect();
    println!("From thread: {list_squared:?}");
}

fn main() {
    // Closures structure
    fn add_one_v1 (x: u32) -> u32 { x + 1 }
    let add_one_v2 = |x: u32| -> u32 { x + 1 };
    let add_one_v3 = |x| { x + 1 };
    let add_one_v4 = |x| x + 1 ;

    let var = 2;
    println!("{}, {}, {}, {}", add_one_v1(var), add_one_v2(var), add_one_v3(var), add_one_v4(var));

    // Different closures approaches: 
    // By reference
    let name = String::from("Alice");
    let greet = || println!("Hello, {}!", name);
    greet(); // Prints Hello Alice


    // By mutable reference
    let mut count = 0;
    let mut increment = || count += 1;
    increment();
    println!("Count: {}", count); // Prints "Count: 1"
    

    // By value (moving the variable)
    let lucky_number = 7;
    let display = move || println!("Your lucky number is: {}", lucky_number);
    display();
    
    // When sharing data through thread it is crucial to move the ownership from one to 
    // another to avoid trying to access data dropped by other thread.
    let list = vec![1, 2, 3];
    println!("Before defining closure: {list:?}");

    thread::spawn(move || square_list(list)).join().unwrap();


    let store = Inventory { shirts: vec![ShirtColor::Red, ShirtColor::Red, ShirtColor::Blue] };

    let user_pref1 = Some(ShirtColor::Red);
    let giveaway1 = store.giveaway(user_pref1);
    println!("The user with preference {:?} gets {:?}",user_pref1, giveaway1);

    let user_pref2 = None;
    let giveaway2 = store.giveaway(user_pref2);
    println!("The user with preference {:?} gets {:?}",user_pref2, giveaway2);

}