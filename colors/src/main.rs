#[macro_use]
extern crate fstrings;
extern crate colored;

use colored::*;

fn main() {
    let a = "like this";
    println!("{}", "Hello, world!...but using colors".purple().bold());
    println!("This works {}", "normally".bold().bright_cyan());
    let b = a.bold().bright_red();
    println_f!("F strings also work {b}");
    println_f!(
        "{text}",
        text = ["or", a].join(" ").italic().bold().bright_green()
    );
}
