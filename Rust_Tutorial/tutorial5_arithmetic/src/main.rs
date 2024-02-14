use std::io;
fn main() {
    println!("Hello, world!");

    let x: u8 = 255; 
    let y: u8 = 1;
    //let z = x + y;// error OVERFLOW, 256 is greater than 255
    //let z = y - x; // error OVERFLOW, uint cannot be signed 
    //println!("{}", z); 
    let x: u8 = 12; 
    let y: i8 = 10;
    // let z = x + y;
    //println!("{}", z); // error cannot add two different types 
    let x: u8 = 255;
    let y: u8 = 10;
    let z = x / y;
    println!("{}", z); // z = 25, the 25.5 gets truncated

    // let x = 20i8;//without underscore
    // let y = 20_i8;//with underscore
    let x= 200_000_i64; // this is equal to 200,000
    println!("x = {}", x); 

    let x = 200_000 as i64;
    let y = 200 as i32;
    let z = x / (y as i64); // converts type before arithmetic
    println!("{}", z); 
    let x = i32::MAX;
    println!("{}", x); 


    //getting an input string
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("expected string");
    println!("{}", input);

    //Converting string to integer
    //you need to give an explicit type conversion to parse
    let x: i32 = input.trim().parse().unwrap();
    println!("int of input + 1 = {}", (x+1));

    let str: String = String::from("56");
    let y: i32 = str.trim().parse().unwrap();
    println!("y - x = {}", (y-x));

}
