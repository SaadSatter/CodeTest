
use std::{thread, time::Duration};

fn main() {
    println!("Hello, world!");

    //Testing out creating and joining threads
    let handle = thread::spawn(||{
        for i in 1..10{
            println!("number {} from spawn thread", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5{
        println!("number {} from main thread", i);
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap();


    // Move closures with threads
    let v = vec![1, 2, 3, 4, 5];
    let handle = thread::spawn(move ||{
        println!("Here's a vector: {:?}", v);
    });

    handle.join().unwrap();
}
