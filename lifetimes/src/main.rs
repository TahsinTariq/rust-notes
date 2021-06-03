fn main() {
    let string1 = String::from("long string is long");

    {
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {}", result);
        println!("{}", longest2(&string1, &string2));
    }

    // Lifetime annotation in struct defination
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };
    println!("{}", i.part);

    // Lifetime Elision rule : https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html#lifetime-elision
    fn first_word(s: &str) -> &str {
        let bytes = s.as_bytes();
        for (i, &item) in bytes.iter().enumerate() {
            if item == b' ' {
                return &s[0..i];
            }
        }
        &s[..]
    }
    println!("{}", first_word("hello world"));

    // the syntax of specifying generic type parameters, trait bounds, and lifetimes
    let string2 = "string2";
    let result = longest_with_an_announcement(
        string1.as_str(),
        string2,
        "Generic type parameters, trait bounds, and lifetimes!",
    );
    println!("The longest string is {}", result);
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

use std::fmt::Display;

fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn longest2<'a>(x: &'a str, y: &str) -> String {
    // Dangling out of scope reference
    let result = String::from("really long string");
    result
}

struct ImportantExcerpt<'a> {
    part: &'a str,
}
