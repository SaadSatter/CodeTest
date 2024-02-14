use std::collections::HashMap;
use std::thread;
use std::hash::{Hash, Hasher};

use std::time::Duration;

fn expensive_function(num: u32) -> u32 {
    println!("calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    num
}

fn main() {
    println!("Hello, world!");
    problem_function(10, 10);


    /*FnMut */
    let x = 5;

    let func = || x ;

    /*FnMut */
    let mut x = 5;

    let mut func = || x = 7;

    /*FnOnce */
    let x = String::from("hi");

    let func = move || x == String::from("5");

    // println!("x: {}", x);

}

struct Closures<T, U> where T: Fn(U) -> U, U: Copy + PartialEq + Eq + Hash{
    calculation: T,
    value: HashMap<U, U>,
}

impl <T,U> Closures<T, U>  where T: Fn(U) -> U, U: Copy + PartialEq + Eq + Hash{
    fn new(calculation: T) -> Closures<T, U> {
        Closures{
            calculation: calculation,
            value: HashMap::new(),
        }
    }

    fn get_result(&mut self, arg: U) -> U {
        match self.value.get(&arg) {
            Some(val) => *val, 
            None => {
                let val = (self.calculation)(arg);
                self.value.insert(arg, val);
                val
            }
        }
    }
}

fn problem_function(min: u32, tired: u32){
    
    let expensive_closure = |num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    };

    let mut struct_closure = Closures::new(expensive_closure);

    if min < 25 as u32 {
        println!("It's too early: you should do {} situps", struct_closure.get_result(tired));
        println!("It's too early: you should do {} pushups", struct_closure.get_result(tired));
    }
    else {
        if tired == 0 as u32 {
            println!("your too tired, take a break");
        }
        else{
            println!("You got some energy left, cooldown time {}", struct_closure.get_result(tired));
        }
    }
}