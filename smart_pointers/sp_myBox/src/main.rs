#[allow(non_snake_case)]
fn main() {
    let x = 5;
    let y = &x;

    println!("{}, {}", *y, y);

    // Default deref coersion
    let s = Box::new(String::from("hello"));
    hello(&s);

    // Implimented deref coersion
    let s2 = MyBox::new(String::from("hello2"));
    hello(&s2);

    // Rust implements Deref coersion for String
    let s3 = String::from("Hello3");
    hello(&s3);
}

use std::ops::Deref;

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

fn hello(s: &str) {
    println!("{}", s);
}
