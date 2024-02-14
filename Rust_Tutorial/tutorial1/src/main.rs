fn main() {
    println!("Hello, world!");

    let _x = 5;
    // x = 5; // error 
    println!("x = {}", _x);

    let y; //making a variable mutable
    y = 10;
    println!("y = {}", y);

    
    let _x = _x+ 10; //redefining a variable 
    println!("x = {}", _x);

    {
        let _x = 5 + _x; /*before this point in the scope, the value of _x is 15
                                so it uses the exterior x (its the only one defined) */
        println!("x = {}", _x);

        let y = _x + _x; /*Now _x has a inner scope definition, only the innerscope 
                                will be used */
        println!("y = {}", y);

    }
    //_x is back at 15 at this point in the scope
    let _x = "helli"; //redefining a variable  to a different type
    println!("x = {}", _x);

    //constants need to be in CAPITAL_SNAKE_CASE and the type must be defined
    const SECOND_IN_MINUTE: u32 = 60;
    println!("SECOND_IN_MINUTE = {}", SECOND_IN_MINUTE);
    //const SECOND_IN_MINUTE: u32 = 35; //error you cannot redefine a const

    

}
