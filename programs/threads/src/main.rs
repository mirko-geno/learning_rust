/*
thread::spawn() creates a new thread receiving a closure. This method
returns a JoinHandle<()> type, which can be used to call join in order to
wait until the completion of the selected thread.

sync::mpsc::channel() creates a channel that allows communication 
between threads. The send function takes ownership of its parameter,
and when the value is moved, the receiver takes ownership of it.

*/

use std::thread;
use std::time::Duration;
use std::sync::mpsc; // mpsc stands for multiple producer, single consumer.


fn iterative_func() {
    for i in 1..10 {
        println!("Hi number {i} from the spawned thread!");
        thread::sleep(Duration::from_millis(1));
    }
    println!("Spawned thread finished");
}


fn main() {
    let thread = thread::spawn(|| iterative_func()); 

    for i in 1..5 {
        println!("hi number {i} from the main thread!");
        thread::sleep(Duration::from_millis(1));
    }

    println!("1");
    thread.join().unwrap();
    println!("2");
    println!("Main thread finished");


    let v = vec![1, 2, 3];
    let thread = thread::spawn(move|| {
        println!("Here's a vector: {v:?}");
    });

    thread.join().unwrap();


    let (tx, rx) = mpsc::channel();

    let thread = thread::spawn(move || {
        let message = String::from("Sending this message from spawned thread through tx");
        tx.send(message).unwrap();
    });

    // rx.recv() blocks the thread until receiving a message
    // on the other hand, rx.try_recv is non blocking and checks
    // instantly if there's any data available in the channel
    let received = rx.recv().unwrap();
    println!("Received message: \n{received}");
    thread.join().unwrap();


    // rx.recv can be implicitely called when treating the receiver as an iterator
    let (tx, rx) = mpsc::channel();
    let tx_2 = tx.clone();

    let _thread = thread::spawn(move || {
        let messages = vec![
            String::from("Hi,"),
            String::from("from"),
            String::from("the"),
            String::from("thread.")
            ];

        for message in messages {
            tx.send(message).unwrap();
            thread::sleep(Duration::from_millis(750));
        };
    });

    let _thread_2 = thread::spawn(move || {
        let messages = vec![
            String::from("Bye,"),
            String::from("from"),
            String::from("the"),
            String::from("other thread.")
            ];

        for message in messages {
            tx_2.send(message).unwrap();
            thread::sleep(Duration::from_millis(750));
        }
    });

    for message in rx {
        println!("Got {message}");
    }

}