/*
We can define a default behaviour for a trait, and, if necessary,
can be modified according the implementation. In the example below
NewsAtricle struct implement defautlt Summary. Tweet, however, 
overrides the summarize function from the trait to create a 
more suitable functionality.
*/

pub trait Summary {
    fn summarize(&self) -> String {
        String::from("(Read more...)")
    }
}


pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}


pub struct Point<T> {
    x: T,
    y: T
}

impl <T> Point<T> {
    pub fn new(x: T, y: T) -> Point<T> {
        Point{ x, y }
    }
}

impl <T> std::fmt for Point<T> {}

pub fn notify(item: &impl Summary) {
    println!("New notification: {}", item.summarize())
}