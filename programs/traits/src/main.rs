use traits::{Summary, Notif, Tweet, NewsArticle, Point};

fn main() {
    let mut notif_vec = Vec::new();

    notif_vec.push(
        Notif::Tweet(Tweet {
            username: String::from("GianTV"),
            content: String::from("Seeee paaa es esaa"),
            reply: false,
            retweet: true,
        })
    );

    notif_vec.push(
        Notif::NewsArticle(NewsArticle{
            headline: String::from("Bananirou died"),
            content: String::from("After multiple hours of streaming he had a heart attack"),
            location: String::from("Tigre, Buenos Aires, Argentina"),
            author: String::from("The mysterious Papu")
        })
    );
    
    for notif in notif_vec {
        match notif {
            Notif::Tweet(tweet) => println!("1 new tweet: {}", tweet.summarize()),
            Notif::NewsArticle(article) => println!("1 new article: {}", article.summarize()),
            Notif::None => println!("There are no notifications")
        }
    }

    let point: Point<u8> = Point::new(5, 2);
    point.cmp_display();
}
