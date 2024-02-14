

/*
Defining an Enum
*/
#[derive(Debug)]
enum IpAddrKind{
    V4,
    V6(String),
    V8(u8,u8,u8,u8),
}

/*
Using Enum and Struct
*/
#[derive(Debug)]
struct IpAddr{
    kind: IpAddrKind,
    address: String,
}

/*
Making Associated Functions with Enums
*/
impl IpAddrKind{
    fn new_v6(address: String) -> IpAddrKind{
        let kind = IpAddrKind::V6(String::from(address));
        kind
    }
}

/*
Pattern Matching
*/
#[derive(Debug)]
enum UsState{
    NewYork,
    California,
    Alaska,
}
#[derive(Debug)]
enum Coin{
    Penny,
    Nickel,
    Dime(UsState),
}


fn value_in_cents(coin : Coin) -> u8 {
    match coin{
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime(state) => {
            println!("Dime from {:?}",state);
            10
        },
    }
}

fn plus_one(x: Option<i32>) -> Option<i32>{
    match x{
        Some(i) => Some(i+1),
        _ => Some(0),
    }
}

use IpAddrKind::V8;
fn main() {
    println!("Hello, world!");
    let four = IpAddr{kind : IpAddrKind::V4, address: String::from("ip address v4"),};
    println!("struct V4 ->{:#?}", four);
    let fr = V8(8,8,8,8);
    let six = IpAddrKind::V6(String::from("ip address v6 address"));
    
    println!("enum V6 ->{:?}", six);
    let six = IpAddrKind::new_v6(String::from("ip address new v6 address"));
    println!("enum V6 ->{:?}", six);

    /*
    Using Options
     */

    let x: i8 = 3;
    let y: Option<i8> = Some(5);
    let z: Option<i8> = None;
//  let sum = x+y; //error
    let sum = x + y.unwrap(); //returns a value if not None
    let sum = x + y.unwrap_or(0); //returns a default value is needed
    let none_sum = x + z.unwrap_or(10);
    println!("y -> {:?}, z -> {:?}, sum ->{:?}, 
    none_sum -> {:?}", y, z, sum, none_sum);

    /*
    Pattern Matching
     */

    println!("{}", value_in_cents(Coin::Dime(UsState::NewYork)));

    let x = Some(5);
    println!("x -> {}, x+1 -> {}",x.unwrap(), plus_one(x).unwrap());

    let some_value = plus_one(x);
    if let Some(6) = some_value{
        println!("Matched with {}", 6);
    }

}
