

// use std::path::Path;
// use std::process::{Child, Command, Stdio};
// use std::io::{self, Write, Error};
// use anyhow::{anyhow, bail, ensure, Context, Error, Result};

fn main() {
    // println!("Hello, world!");
    // let mut command = Command::new("C:\\Users\\ssatter\\OneDrive - Qualcomm\\Documents\\Code Test\\output\\test.exe");
    // println!("command : {:?}", command);
    // let output = command.spawn()?.wait()?;
    // println!("status: {}", output.status);
    // io::stdout().write_all(&output.stdout).unwrap();
    // io::stderr().write_all(&output.stderr).unwrap();
    // let hello = output.stdout;
    // println!("{:?}",hello);

    let mut s = String::from("hello");
    let o = Some(&mut s);
    
    println!("Option: {:?}", o);

    // Ok(0)
}
