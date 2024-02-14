use std::io;

fn main() {
    println!("Hello, world!");
    let mut input = String::new();
    // You must read in using strings
    // Rust is pass by reference, by default only a copy is passed to functions
    // we need to pass in the reference to the String 
    io::stdin().read_line(&mut input).expect("Failed to read input");
    // .expect is not necessary but handles errors
    println!("{}", input);
}
