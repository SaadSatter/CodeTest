
/*
Practicing With Mods
*/
mod front_of_house{
    pub mod hosting{
        pub fn add_to_waitlist(){}

        fn seat_at_table(){}
    }

    mod serving {
        fn take_order(){}

        fn serve_order(){}

        fn take_payment(){}
    }
}

fn eat_at_restaurant(){
    //Absolute Path
    crate::front_of_house::hosting::add_to_waitlist();

    //Relative Path
    front_of_house::hosting::add_to_waitlist();
}

/*
Accessibility
*/
fn serve_order(){}
mod back_of_house{
    fn fix_incorrect_order(){
        
        cook_order();
        super::serve_order();
    }
    fn cook_order(){}

}

/*
Structs and Enums in Mods
*/
mod cafe{
    pub struct Breakfast{
        pub toast : String,
        soup_of_the_day : String,
    }

    impl Breakfast{
        pub fn summer(toast: &str)->Breakfast{
            Breakfast{
                toast : String::from(toast),
                soup_of_the_day: String::from("chowder"),

            }
        }
    }
}



pub fn eat_breakfast(){
    let mut meal = cafe::Breakfast::summer("Wheat");

    // meal.soup_of_the_day = String::from("chicken noodle");
    meal.toast = String::from("Rye");

    // let new_meal = cafe::Breakfast{
    //     toast : String::from("Sourdough"),
    //     soup_of_the_day : String::from("hot and spicy")
    // };
}

mod Meal{
    pub enum Appetizer{
        Soup,
        Salad,
    }
}

pub fn order(){
    let order1 = Meal::Appetizer::Soup;
    let order2 = Meal::Appetizer::Salad;
}

/*
Use Keyword
*/

mod BBQ{
    pub mod hosting{
        pub fn add_to_waitlist(){}
    }
}

//Absolute Path
// use crate::BBQ::hosting;

//Relative Path
use self::BBQ::hosting;
pub fn eat_at_BBQ(){
    
    hosting::add_to_waitlist();

    //Relative Path
    front_of_house::hosting::add_to_waitlist();
}

use std::io::Result as IoResult;
use std::fmt::Result;

pub use self::cafe::Breakfast;
//Glob operator
use std::io::*;

mod Parent;

use Parent::child::new_fn;





mod rest {
    pub mod bed{
        pub struct Sleep{
            hours: i32,
            time: String,
        }

        pub enum Coin{
            Penny,
            Dime,
        }

        impl Sleep{
            pub fn new(hours: i32, time: String)-> Sleep{
                Sleep{
                    hours,
                    time,
                }
            }
        }
    } 
}

use rand::{Rng, CryptoRng};

pub fn fk_main(){
    use self::rest::bed::Sleep;
    new_fn();
    let x = Sleep::new(3, String::from("ok"));
    let y = rest::bed::Sleep::new(3, String::from(""));

    let x = rest::bed::Coin::Penny;
    use rest::bed::Coin::Dime;
    use rest::bed::Coin as Cn;

    enum Coin {
        Quarter,
    }
    let y = Cn::Dime;
    let z = Dime;

    // let x: i32 = Rng::gen(&mut self);

}