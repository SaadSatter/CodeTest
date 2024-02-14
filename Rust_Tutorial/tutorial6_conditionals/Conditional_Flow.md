# Condtional Flow

## Using conditionals for setting booleans

* Very similar to all other languages
  * Symbols : <,>, <=,>=, ==, !=
  * Operators : &&, ||, !
    * In built precedence order: !, &&, ||
  * **Note** : you need to compare with the same types very strict
  * Example:
    * `let x = 3 < 1; // x is false`
    * `let x = 3 > 1.5; // errors`

    ```rust
    let x = 3 < 1; // x is false
    println!("x = {}", x);
    //let x = 3 > 1.5; // errors
    let y = 2.5 > 1.3 && 5 > 2; // y is true
    println!("y = {}", y);
    let z = x || y; //y is true so z is true
    println!("z = {}", z);
    ```

* Conditional Flow
  * Similar to the conditional declaration, same type is needed for comparisons
  * Example:

    ```rust
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
    ```
