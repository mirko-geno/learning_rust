use traits::{Summary, Tweet, NewsArticle};


fn main() {
    let tweet = Tweet {
        username: String::from("GianTV"),
        content: String::from("Seeee paaa es esaa"),
        reply: false,
        retweet: true,
    };
    println!("1 new tweet: {}", tweet.summarize());

    let article: NewsArticle = NewsArticle{
        headline: String::from("Bananirou died"),
        content: String::from("After multiple hours of streaming he had a heart attack"),
        location: String::from("Tigre, Buenos Aires, Argentina"),
        author: String::from("The mysterious Papu")
    };
    println!("1 new article: {}", article.summarize());
}
