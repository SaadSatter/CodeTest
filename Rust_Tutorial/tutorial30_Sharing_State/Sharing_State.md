# Sharing State

* As opposed to a channel, a sharing state is to share variables or resources across threads and Rust enables this in multiple ways 

## The Mutex API

* Mutex Overview
  * A shared resource is handeled as a Mutex and to access the resource you need to acquire a lock 
  * A lock is a data structure that signifies which thread currently has exclusive access to the resource of the Mutex
  * Once a thread is done with the data, you need to unlock the mutex so that other threads can access 
* Example in Rust:
    ```rust
    use std::sync::Mutex;
    fn main() {
        let m = Mutex::new(5);

        {
            let mut num = m.lock().unwrap();
            *num = 6;
        }

        println!("m = {:?}", m);
    }
    ```
    * To start we need to make a `Mutex` struct which is similar to a smart pointer 
    * Then in an inner scope we take the data in the struct using `m.lock().unwrap()`
      * `lock()` will do 2 things
        * It will **block** the current thread until the Mutex is ready and unlocked
        * If ready then it will take the data as a `Result<>` type and unwrap it as a `MutexGuard<data-type>` smart pointer.
          * It unwraps as a result because if there is already a thread that holds the lock and the thread **fails** before releasing the lock then this will return an error
        * A `MutexGuard` implements the `Deref` trait of smart pointers such that using `*num` will access the interior data of the Mutex
        * It also implements the `drop` trait so that it **automatically unlocks** when the `MutexGuard` is dropped
          * This is why we used an inner scope to define the use of the `MutexGuard` 

## Sharing a Value between Threads

* So let's make an example of a counter that 10 threads will each increment one time sharing a Mutex with another
* Example:
    ```rust
    let counter = Mutex::new(0);
    let mut handles = vec![];

    for _ in 0..10 {
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
    ```
    * Here we have a for loop that creates a vector of threads that perform the same operation
      * Each iteration of the loop spawns a thread and creates a handle for it, and then pushes it in the vector of threads `handles`
      * Each thread will take a `MutexGuard` and mutate that value inside
    * Now there is a problem with this code, it will not compile
    * As we can see that there is an error with the `move` statement in the spawned threads
      * This is because we move `counter` (our mutex) in our first thread and then in the following iterations it can no longer be accessed as the main thread no longer owns the mutex
* What we would want to do is to have `counter` to have multiple owners 
  * Recalling back to the `Rc<>` smart pointer from tutorial 25
* We need to use `Rc<>` and wrap it around the mutex to allow for multiple owners of the same variable   
  * Example:
    ```rust
    let counter = Rc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Rc::clone(&counter);
        .....}
    ```
    * This should work but this is not thread safe
    * This is because `Rc` is not asynchronous pointer so its not safe between threads
* Rust has a solution for this which is a `Arc<>` smart pointer.
  * This is an Asynch/Atomic `Rc<>` smart pointer  
* New Fix:
    ```rust
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
    ```
    * Note that `counter` is an immutable data structure of `Mutex<>` but we can get a `mut` reference because `Mutex` is an **interrior mutability** smart pointer
      * Meaning it can mutate the object within it without mutating the wrapper itself
      * We have seen this behavior is the same fashion with `RefCell<>` smart pointers and remember it came with some unsafe issues as well          
      * `RefCell<>` can unsafely cause circular dependency while `Mutex<>` can cause deadlocks       