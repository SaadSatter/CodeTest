fn main() {
    println!("Hello, world!");

    /*
    Borrow Checker
     */

     let r;          // ---------+-- 'a
                           //          |
    {                      //          |
    let x = 5;        // -+-- 'b  |
    r = &x;                //  |       |
    }                      // -+       |
                           //          |
    //println!("r: {}", r); //         | error
                           // ---------+

    let x = 5;         // ----------+-- 'b
                            //           |
    let r = &x;       // --+-- 'a  |
                            //   |       |
    println!("r: {}", r);   //   |       |
                            // --+       |
                            // ----------+


    /*
    Generic Lifetime Annotations
     */        
    
    // &i32        // a reference
    // &'a i32     // a reference with an explicit lifetime
    // &'a mut i32 // a mutable reference with an explicit lifetime
                    
    let string1 = String::from("Hello, world! 1");
    let string2 = String::from("Hello!");

    let mut result = longest(string1.as_str(), string2.as_str());
    println!("result: {}", result);

    
    // let string1 = String::from("Hello, world! 2");
    // {
    //     let string2 = String::from("Hello, Everyone!");
    //     result = longest(string1.as_str(), string2.as_str());
    //     println!("result in a new lifetime: {}", result);
    // }
    // println!("result back outside lifetime: {}", result);

    /*
    Thinkning in terms of lifetime
     */
    let string1 = String::from("Hello, world! 2");
    {
        let string2 = String::from("Hello, Everyone!");
        result = longest2(string1.as_str(), string2.as_str());
        println!("result in a new lifetime: {}", result);
    }
    println!("result back outside lifetime: {}", result);


    /*
    Lifetime Annotations In Structs
     */

    struct ImportantExcerpt<'a> {
        part: &'a str,
    }

    let novel = String::from("Call me Ishmael, Some Years ago...");
    let first_sentence = novel.split('.').next().expect("Couldn't find the sentene");
    let i = ImportantExcerpt{part: first_sentence};

    /*
    Lifetime Elision

        1st rule: each parameter that is a reference that is passed in 
            gets its own lifetime parameter 
        2nd Rule: IF there is exactly one input lifetime parameter, 
            then that lifetime parameter is automatically assigned to the 
            output lifetime 
        3rd Rule: If there are multiple input lifetime parameters, 
            but one of them is `&self`, or `&mut self`, the lifetime of 
            `self` is assigned to all output parameters.
     */


    /*
    Methods with Lifetime
     */

     impl<'a>  ImportantExcerpt<'a> {
        fn level(&self) -> i32 {
            3
        }
    }
    
    impl<'a> ImportantExcerpt<'a> {
        fn announce_and_return_part(&self, announcement: &str) -> &str {
            println!("Attention please: {}", announcement);
            self.part
        }
    }
    
    let novel = String::from("Call me Ishmael, Some Years ago...");
    let first_sentence = novel.split('.').next().expect("Couldn't find the sentene");
    let i = ImportantExcerpt{part: first_sentence};
    {
        let announcement: &'static str = "ANNNOUNCE!!";

    }

}                          

// fn longest(string1: &str, string2: &str) -> &str{
//     if string1.len() > string2.len(){
//         string1
//     }
//     else{
//         string2
//     }
// }

// fn longest3<'a>(string1: &str, string2: &str) -> &'a str{
//     let s1: = String::from("hi");
//     s1.as_str()
// }



fn longest2<'a>(string1: &'a str, string2: &str) -> &'a str{
    string1
}


fn longest<'a>(string1: &'a str, string2: &'a str) -> &'a str{
    if string1.len() > string2.len(){
        return string1;
    }
    else{
        return string2
    }
}
