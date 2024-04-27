use traits::Pair;
use traits::{notify, NewsArticle, Summary, Tweet};
pub mod traits;

fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course,as you probably already know,people"),
        reply: false,
        retweet: false,
    };
    println!("1 new tweet: {}", tweet.summarize());
    println!("1 new tweet: {}", tweet.summarize_imple());

    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh,PA,USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best hockey team in the NHL.",
        ),
    };
    println!("New article available!{}", article.summarize_imple());

    notify(&tweet);

    //Conditionally implementing methods on a generic type depending on trait bounds
    let p = Pair { x: 1, y: 2 };
    p.cmp_display();

    //Because the standard library has this blanket implementation,
    //we can call the to_string method defined by the ToString trait on any type that implements the Display trait
    let s = 3.to_string();
}
