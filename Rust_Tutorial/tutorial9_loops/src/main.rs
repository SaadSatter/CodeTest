fn main() {
    println!("Hello, world!");
    
    //Infinite loops
    println!("Infinite loops");
    let mut x = 1;
    loop{
        if x == 11{
            break;
        }
        println!("{x:?}");
        x += 1;
    }
    
    //While loops
    println!("While loop");
    let mut n = 0;
    while n < 50 {
        n += 1;
        if n%5 == 0{
            println!("{n:?}");
        }
        else{
            continue;
        }
        
        
    }

    //For loops
    println!("For loop");
    let range = 3..21;
    for i in range{
        if i%3 == 0{
            println!("{i:?}");
        }
    }
    let lst = ["cat", "dog", "cow"];
    for i in lst.iter(){
        println!("{i:?}");
    }

    let lst = ["wolf", "tiger", "fox", "elephant"];
    for (i, val) in lst.iter().enumerate(){
        if i % 2 == 0 {
            println!("{:?} -> {:?}",i,val);
        }

    }
}
