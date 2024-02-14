fn main() {
    println!("Hello, world!");
    let s1: String = String::from("hello");
    let s2: String = s1;//This is a move, s2 now is the owner and s1 is invalid
    //println!("{}", s1);try a cargo check  to see what happens
    println!("{}", s2);


    /*
    Ownership basics with functions
     */
    let mut s1: String = String::from("taking my ownership");
    let x:i32 = 5;
    take_ownership(s1, x);
    //println!("{}", s1); error
    println!("I still own -> {}", x); // only x maintains ownership since variables are copied by default
    
    s1 = gives_ownership();
    println!("{}", s1); //this is ok now since we return ownership
    

    //You can take and give back ownership but thats very tedious
    let s2 = take_ownership_and_gives_back(s1);
    println!("S2 now owns -> {}", s2); //this takes the ownership given to s1 and gives it to s2

    //For instance maybe you want to calculate the length of a string 
    let (s2, len) = cal_len(s2);
    println!("s2 still owns -> {}, and its length is {}", s2, len);


    /*
    Alternatively you can borrow values and ownership
     */

    //Basics of borrowing ownership
    let s1: String = String::from("borrowing example");
    borrows(&s1);
    println!("{}", s1);// gets back ownership after the function call

    //Passing reference is immutable by default 
    //We need to pass a mutable reference
    let mut s1 : String = String::from("mutable reference");
    change(&mut s1);
    println!("{}", s1);


    //Lets trying multi referencing and mixing mutable and immutable
    let mut s1 : String = String::from("new mutable string");

    
    let r1 : &String = &s1;
    let r2 : &String = &s1;

    // let mut s2: String = &mut s1; //error
    println!("{}, {}", r1, r2);

    let s2 : &mut String = &mut s1; 
    s2.push_str(", pushed change");
    println!("{}", s2);

    // let reference_to_notohing: &String = dangle();

    /*
    Lets try slicing 
     */

    let mut s: String = String::from("full string");

    let first_word : &str = &s[0..4];
    let second_word : &str = &s[5..];
    println!("{}, and {}", first_word, second_word);//note this is immutable
    s.push_str("hi");//push_str is a mutable reference
    println!("{}", s);
    


}

fn take_ownership(s: String, x: i32) {
    println!("{}", s);
}

fn gives_ownership() -> String {
    let strng: String = String::from("got new ownership");
    return strng;
}

fn take_ownership_and_gives_back(s: String) -> String{
    println!("Function now owns -> {}", s);
    return s;
}

fn cal_len(s1: String) -> (String, i32){
    let x = s1.len();
    return (s1, x as i32);
}

fn borrows(s1: &String){
    println!("{}", s1);
}

fn change(s1: &mut String){
    s1.push_str(", changed");
}

// fn dangle() -> &String{
//     let s1 : String = String::from("foo");
//     &s1 //this is dangling 
// }