use traitss::other_foo;
use traitss::{self, cond_impl::Pair, Summary, Tweet};
use utils::other_bar;
mod utils;

fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());

    // ----------------------------->> conditional trait impl using trait bounds
    let p = Pair::new(4, 5);
    p.cmp_display();
    other_foo::bar();
    other_bar::bar();
}
