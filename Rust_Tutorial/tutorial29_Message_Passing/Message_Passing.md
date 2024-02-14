# Message Passing 

## Transfer Data Between Threads

* Rust has a tool called channel
* There are two parts of a channel, a transmitter and reciever 
  * Channel is closed when either the transmitter or reciever is dropped
* You need to import the `mpsc`(multi producer, single consumer FIFO queue) from the standard library
  * As the name suggests, you can have multiple producers for the queue but only one consumer
* Example:
    ```rust
    use std::sync::mpsc;
    use std::thread;
    fn main() {
        println!("Hello, world!");
        let (tx, rx) = mpsc::channel();
        thread::spawn(move ||{
            let msg = String::from("hi");
            tx.send(msg).unwrap();
        });
        let recieved = rx.recv().unwrap();
        println!("Got: {}", recieved);
    }

    ```
    * So here we can see that we need to use special function `send()` and `recv()` to populate the queue and recieve from it 
    * Note that both return a `Result<>` type and so we need to unwrap 
    * Also note tha the `recv()` function call will **block** the main thread until the queue is populated with at least one value 
      * If the channel closes or the transmitter is dropped in between, then we will recieve an error 
    * There is also a `try_recv()` method which will **not block** the main thread    
      * This will return a `Result<>` type right away, if there is something in the queue then it will consume it, otherwise just return error the queue is empty.
      * This is useful to keep the main thread moving 
    * Also note that in the `send()` function we can pass any type we want to 

## Ownership Rules In Concurrent Code

* Note that when sending something through the channel, the `send()` function takes ownership of the variable 
    ```rust
    let msg = String::from("hi");
    tx.send(msg).unwrap();
    //println!("{}", msg);
    ```
    * Printing after the send would result in an error     
    * This prevents memory leaks since we cannot drop or modify a value after passing to another thread

## Sending Multiple Values 

* Sending multiple values from a spawn thread to another thread is easy and we use the reciever as an interator itself
* Example:
    ```rust 
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {}", received);
    }
    ```
    * Here we make a vector and loop through it as an interator and pass each value of the vector into `tx.send()` 
    * Each time we send we also sleep for 1 second
    * Now looking at the reciever, note that the `rx` itself can be treated as an iterator where in each iteration it will call `recv()` and unwrap the value to `recieved`
      * The ending for this loop is when the channel itself **closes**, i.e as long as the channel is open and `tx` is not dropped, the loop will keep going 

## Multiple Producers Sending Messages

* Remember the channel supports multiple producers but one consumer 
* Although the channel only gives one producer by default, you can use `clone()` to clone the sender and make another handle to the producer side of the channel
* Example:
    ```rust
        // --snip--

    let (tx, rx) = mpsc::channel();

    let tx1 = tx.clone();
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {}", received);
    }

    // --snip--

    ```
    * Now there are two threads passing messages down the channel
    * Note that we made a `clone()` of the transmitter to allow for two producers
    * Also note that the for loop will not end until both threads have completed and all producers are dropped
    * Also remember that running multiple threads the output will vary 