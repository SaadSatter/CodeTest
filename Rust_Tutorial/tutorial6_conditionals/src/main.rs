fn main() {
    println!("Hello, world!");
    let x = 3 < 1; // x is false
    println!("x = {}", x);
    //let x = 3 > 1.5; // errors
    let y = 2.5 > 1.3 && 5 > 2; // y is true
    println!("y = {}", y);
    let z = x || y; //y is true so z is true
    println!("z = {}", z);

    //just note that the paranthesis are not needed 
        //for a simple conditional
    let food = "cookie";
    if food != "cookie"{
      println!("I dont like cookies, I like {}", food);
    }
    else if (food == "fruit"){
      println!("I love {}", food);
    }
    else{
      println!("I just like {}", food);
    }
}
