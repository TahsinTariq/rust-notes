use traitss::other_foo;
use traitss::{self, cond_impl::Pair, Summary, Tweet};
use utils::other_bar;
mod utils;

struct SelfImportStruct {
    v1: i32,
    v2: i32,
}

enum SelfImportEnum {
    opt1,
    opt2 { x: i32 },
}

use crate::SelfImportEnum::{opt1, opt2};

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

    let using_self_import = SelfImportStruct { v1: 3, v2: 2 };
    let using_self_import_enum = opt2 { x: 3 };
}
