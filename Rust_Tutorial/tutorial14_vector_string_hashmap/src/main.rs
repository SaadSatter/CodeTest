
// extern crate unicode_segmentation;

// use unicode_segmentation::UnicodeSegmentation;
use std::collections::HashMap;


fn main() {
    println!("Hello, world!");
    /*
    Vectors 
     */
    let mut vector : Vec<i32> = Vec::new();
    vector.push(32);
    let mut v2  = vec![1,2,3,4];
    // let third = &v2[20]; error
    let third = &v2[2]; //this does an immutable borrow 
    //v2.push(3); immutable and mutable reference conflict
    println!("Third -> {}, {}", third, &v2[2]);// two immutable borrows is fine

    match v2.get(2) {
        Some(third) => println!("Get Third -> {}, {}", third, v2.get(2).unwrap()),
        None => println!("Index out of range"),
    }

    for i in &mut v2{
        *i += 50;
        println!("Current value: {}", i);
        *i += 50;
    }

    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    match &row[0]{
        SpreadsheetCell::Int(value) => {
            println!("Row cell 1: {}", value);
        }
        _ => println!("Not an Int value"),
    }

    /*
    Strings
     */
    let s1 = String::new();
    let s1: &str = "hello, world";
    let s1 = s1.to_string();
    let s1 = String::from("hello, world");
    
    let mut s1 = String::from("foo");
    s1.push_str("bar");
    s1.push('!');

    //appending strings
    let s1 = String::from("Hello");
    let s2 = String::from(" World!");
    // let s3 = s1 + &s2;
    let s3 = format!("{}{}",s1, s2); 
    // let s3 = "hello" + "world";error

    //Indexing into a string 
    let hello  = String::from("नमस्ते");
    //Bytes [224, 164, 168, 224, 164, 174, 224, 164, 184, 224, 165, 141, 224, 164, 164, 224, 165, 135]
    for b in hello.bytes(){
        println!("{}",b);
    }
    //Scalar ['न', 'म', 'स', '्', 'त', 'े']
    for c in hello.chars(){
        println!("{}",c);
    }
    //Grapheme ["न", "म", "स्", "ते"]
    // for g in UnicodeSegmentation::graphemes("नमस्ते", true){
    //     println!("{}", g);
    // }


    /*
    Hashmaps
     */
    let blue = String::from("blue");
    let yellow = String::from("yellow");
    let mut scores = HashMap::new();
    scores.insert(blue, 10);
    scores.insert(yellow, 50);

    let team_name = String::from("blue");
    let score = scores.get(&team_name);
    let mut value = 0;
    match score{
        Some(sc) => {
            value = *sc;
        }
        None => {
            println!("Key Value pair doesn't exist.")
        }
    }

    for (key, value) in &scores{
        println!("{}, {}", key, value);
    } 

    scores.entry(String::from("yellow")).or_insert(30);

    //Updating example
    let mut map = HashMap::new();
    let text  = "hello world wonderful world";

    for word in text.split_whitespace(){
        let counter = map.entry(String::from(word)).or_insert(0);
        *counter += 1;
    }
    println!("{:?}", map);
    
}
