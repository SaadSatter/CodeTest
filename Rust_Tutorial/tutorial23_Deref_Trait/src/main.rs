
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

use std::ops::Deref;

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}


fn hello(name: &str) {
    println!("Hello, {name}!");
}


fn main() {
    println!("Hello, world!");

    let x = 5;
    let y = &x;
    let z = Box::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);
    assert_eq!(5, *z);

    /*My Box */

    let y = MyBox::new(5);

    assert_eq!(x, *y);
    assert_eq!(x, *(y.deref()));

    /*Deref Coercion */
    let m = MyBox::new(String::from("Rust"));
    hello(&((*m)[..]));
    hello(&m);
    

}
