fn main() {
    println!("Hello, world!");
    
    //the last line `y+1` has no semi colon, denoting its expression and 
    //returning a value rather than being a statement
    let x = {
        let y = 5;
        y+1
    };
    println!("x = {}", x);
    test_run();
    add_numbers(5,6);
    println!("multiplying 8 and 5 = {}", mult_numbers(8, 5));
    let result = sub_numbers(28, 5);
    println!("result = {}", result);
    let result = sub_numbers(8, 5);
    println!("result = {}", result);
}


fn test_run(){
    println!("test_run has been called");
}

fn add_numbers(x:i32, y:i32){
    println!("add_numbers {} + {} = {}", x, y, (x+y));
}

fn sub_numbers(x:i32, y:i32) -> i32{
    let result = x - y;
    if result < 10 {
        return result+10 //returning with an expression
    }
    return result //returning with a return expression
}

fn mult_numbers(x:i32, y:i32) -> i32{
    x * y //note this is an expression
}