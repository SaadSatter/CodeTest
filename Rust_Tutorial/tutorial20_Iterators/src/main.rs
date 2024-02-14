use std::iter::Iterator;

fn main() {
    println!("Hello, world!");

    let v1 = vec![1, 2, 3];

    let v1_iter = v1.iter();

    for val in v1_iter {
        println!("Got: {}", val);
    }
    
    
    /*type keyword */
    type Kilometers = i32;
    let x: i32 = 5;
    let y: Kilometers = 5;
    println!("x + y = {}", x + y);

    type Thunk = Box<dyn Fn() + Send + 'static>;

    let f: Thunk = Box::new(|| println!("hi"));

 
    
}

/*Iterator trait */

// pub trait Iterator {
//     type Item;

//     fn next(&mut self) -> Option<Self::Item>;

//     // methods with default implementations elided
// }



/*Demonstration of Iterators  */
#[test]
fn iterator_demonstration() {
    let v1 = vec![1, 2, 3];

    let mut v1_iter = v1.iter();

    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));
    assert_eq!(v1_iter.next(), None);


}



/*Consumer  */
#[test]
fn iterator_sum() {
    let v1 = vec![1, 2, 3];

    let v1_iter = v1.iter();

    let total: i32 = v1_iter.sum();

    assert_eq!(total, 6);

    
}

/*Adaptor */
#[test]
fn mapping(){
    let v1: Vec<i32> = vec![1, 2, 3];

    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();

    assert_eq!(v2, vec![2, 3, 4]);
}

/*Creating your own iterator */
struct Counter{
    count: u32,
}

impl Counter{
    fn new(count: u32) -> Counter{
        Counter { count }
    }
}

impl Iterator for Counter{
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count < 5 {
            self.count += 1;
            Some(self.count)
        }
        else{
            None
        }
    }
}

#[test]
fn calling_next_test(){
    let mut counter = Counter::new(0 as u32);

    assert_eq!(counter.next(), Some(1));
    assert_eq!(counter.next(), Some(2));
    println!("counter {}", counter.count);
    assert_eq!(counter.next(), Some(3));
    assert_eq!(counter.next(), Some(4));
    match counter.next() {
        Some(val) => println!("counter {}", val),
        None => println!("counter is NONE"),
    }
    assert_eq!(counter.next(), None);

}

#[test]
fn using_defualt_iterator_trait_methods(){
    let sum: u32 = Counter::new(0 as u32)
        .zip(Counter::new(0 as u32).skip(1))
        .map(|(a, b)| a * b)
        .filter(|x| x % 3 == 0)
        .sum();

    assert_eq!(sum, 18);
}