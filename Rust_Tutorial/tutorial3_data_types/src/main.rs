fn main() {
    println!("Hello, world!");

    let mut x = 5;
    let y = 10;
    x = y;

    //Compound Types
    
    // Tuples
    
    /*
    let mut x: (i32,bool, char) = (1,true,'s') 
    let y: (i8,bool, char) = (1,true, 'C')
    x = y // error, i32 and i8 does not match
     */


    /*
    if one tuple is only defined explicitly, 
    and the values can be of the same type 
    then the implicit will convert to 
    the explicit
     */
    let mut x = (1,true,'s'); //implicit 
    let y: (i8,bool, char) = (4,true, 'C');//explicit
    x = y; // no error, implicit i32 becomes i8
    println!("x = {}, y = {}", x.0, y.0);
    x.2 = 'A';
    println!("x.2 = {}", x.2);
    println!("x = {:?}", x);

    /*
    Arrays
     */

    let mut arr = [1,2,3];
    let arr2: [u128; 3] = [1,2,3];
    println!("arr = {:?}, arr2 = {:?}", arr, arr2);

}
