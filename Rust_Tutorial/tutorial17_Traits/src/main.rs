use std::fmt::Display;



#[derive(Debug)]
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

#[derive(Debug)]
pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

/*
Defining a Trait
*/


// pub trait Summary {
//     fn summarize(&self) -> String;
// }

// impl Summary for NewsArticle {
//     fn summarize(&self) -> String {
//         format!("{}, by {} ({})", self.headline, self.author, self.location)
//     }
// }
// impl Summary for Tweet {
//     fn summarize(&self) -> String {
//         format!("{}: {}", self.username, self.content)
//     }
// }

/*
Default implementation for Traits
*/
pub trait Summary: std::fmt::Debug {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}

impl Summary for NewsArticle {
    fn summarize_author(&self) -> String {
        format!("Author: {}", self.author)
    }
}
impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }

    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

impl Tweet {
    pub fn foo(&self){
        println!("hi");
    }
}


/*
Trait Parameters
*/
pub fn notify(item: &impl Summary, item2: &impl Summary) {
   
    println!("Breaking news! {} and {}", item.summarize(), item2.summarize());

}

pub fn notfy3(item: &impl std::fmt::Debug){
    println!("Breaking news! {:?}", item);
}

pub fn notify2<T: Summary>(item: &T, item2: &T){
    println!("Breaking news! {} and {}", item.summarize(), item2.summarize());
}

//inherting multiple traits
pub fn some_fun(item: &(impl Summary + Display)) {}
pub fn some_fun2<T: Summary + Display>(item: &T) {}

fn some_function(t: &(impl Display + Clone), u: &(impl Clone + Summary)) -> i32 {0}
fn some_function2<T: Display + Clone, U: Clone + Summary>(t: &T, u: &U) -> i32 {0}

//Where operator
fn some_function3<T, U>(t: &T, u: &U) -> i32 
where   T: Display + Clone,
        U: Clone + Summary
{
    0
}


/*
Trait return Types
*/

fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    }
}

/*
Conditionally Implementing Traits
*/
struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}

fn main() {
    println!("Hello, world!");

    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    };

    let article = NewsArticle{
        author: String::from("John Smith"),
        headline: String::from("The Sky is falling"),
        content: String::from("The sky is not actually falling"),
        location: String::from("New York")

    };

    let article2 = NewsArticle{
        author: String::from("Doe Joan"),
        headline: String::from("The Ocean is rising"),
        content: String::from("Its all basically fine"),
        location: String::from("California")

    };

    

    println!("1 new tweet: {}", tweet.summarize());
    println!("1 new article: {}", article.summarize());
    notify(&article, &tweet);
    notify3(&article);
    notify2(&article, &article2);

    println!("Summarizable ->: {}", returns_summarizable().summarize());
}

