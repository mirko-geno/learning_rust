use ::std::fmt::Display;

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


pub enum Notif {
    Tweet(Tweet),
    NewsArticle(NewsArticle),
    None
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

impl<T> Point<T> {
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl <T: Display + PartialOrd> Point<T> {
    pub fn cmp_display(&self) {
        if self.x >= self.y {
            println!("x is larger than y: {}>{}", self.x, self.y);
        } else {
            println!("y is larger than x: {}>{}", self.y, self.x);
        }
    }
}