
#[derive(Debug)]
struct User{
    username: String,
    email: String,
    sign_in_count: i32,
    active: bool,
}

#[derive(Debug)]
struct Rectangle{
    width: i32,
    height: i32,
}

impl Rectangle {
    fn area(&self) -> i32{
        self.width * self.height
    }

    fn make_square(&mut self) {
        self.width = self.height;
    }
}

impl Rectangle {
    fn make_new_square(size: i32) -> Rectangle{
      let rect : Rectangle = Rectangle{
        width : size,
        height: size,
      };
      return rect;
    }
}

fn main() {
    println!("Hello, world!");
    
    let user1 = User{
        email : String::from("world@example.com"),
        username : String::from("some_name"),
        sign_in_count : 0,
        active : false,
    };
    /*
    Building a new struct with a constructor
    */
    let name = user1.username;
    println!("Hello {} from {}", name, user1.email);

    let user2 = build_user(String::from("earth@example.com"), String::from("newUser"));

    /*
    Reusing a previous struct to make a new struct
    */
    // let user3 = User{
    //     username : String::from("user3"),
    //     ..user2
    // };
    // println!("{}", user2.email); causes a ownership error
    let user3 = User{
        email : String::from("user3@example.com"),
        username : String::from("user3"),
        ..user2
    };
    println!("user 3 -> {}, user 2 -> {}", user3.sign_in_count, user2.sign_in_count);


    /*
    Tuple Struct
    */
    #[derive(Debug)]
    struct Color(i32, i32, i32);
    #[derive(Debug)]
    struct Point(i32, i32, i32);

    let new_color = Color(0, 0, 0);
    let new_point = Point(0, 0, 0);
    println!("new_point -> {:#?}, user3 -> {:#?}", new_point, user3);

    
    /*
    Using member methods and Associated Functions
     */
    let mut rect = Rectangle{
        width : 10,
        height : 20,
    };
    println!(" rect -> {:#?}", rect);
    println!("rect area -> {:#?}", rect.area());
    rect.make_square();
    println!("make rectangle into a square -> {:#?}", rect);

    let rect3 = Rectangle::make_new_square(10);
}

fn build_user(email: String, username: String) -> User {
    let user =  User{
        email,
        username,
        sign_in_count: 0,
        active: false,
    };
    
    user
}
