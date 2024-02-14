use std::fs;
use std::{fs::File, io::ErrorKind};
use std::io::{self, Read};
use std::error::Error;
fn main() -> Result<(), Box<dyn Error>> {
    println!("Hello, world!");
    /*
    Panic Macro
     */
    // panic!("crash and burn");
    
    // a();

    /*
    Result Enum
     */
    let greeting_file_result = File::open("hello.txt");

    // let greeting_file = match greeting_file_result {
    //     Ok(file) => file,
    //     Err(error) => panic!("Problem opening the file: {:?}", error),
    // };

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error);
            }
        },
    };

    let f = File::open("hello.txt").unwrap();
    let f = File::open("hello.txt").expect("Failed to open the file");



    /*
    Propagating errors
     */
    
    let propagated_error = read_username_from_file()?;
    Ok(())
  
}





// fn read_username_from_file() -> Result<String, io::Error> {
    
//     let username_file_result = File::open("hello.txt");

//     let mut username_file = match username_file_result {
//         Ok(file) => file,
//         Err(e) => return Err(e),
//     };

//     let mut username = String::new();

//     match username_file.read_to_string(&mut username) {
//         Ok(_) => Ok(username),
//         Err(e) => Err(e),
//     }
// }

// fn read_username_from_file() -> Result<String, io::Error> {
//     let mut username_file = File::open("hello.txt")?;
//     let mut username = String::new();
//     username_file.read_to_string(&mut username)?;
//     Ok(username)
// }

// fn read_username_from_file() -> Result<String, io::Error> {
//     let mut username = String::new();
//     File::open("hello.txt")?.read_to_string(&mut username)?;
//     Ok(username)
// }

fn read_username_from_file() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}

fn a(){
    b();
}

fn b(){
    c(22);
}

fn c(i: i32){
    panic!("crash and burn");
}
