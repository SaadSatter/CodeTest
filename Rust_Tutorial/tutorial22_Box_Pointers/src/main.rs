

enum List {
    Cons(i32, Box<List>),
    Nil,
}
use List::{Cons, Nil};


fn main() {
    println!("Hello, world!");
    let b = Box::new(5);
    println!("b = {}", b);

    /*List Recursive type */
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
}
