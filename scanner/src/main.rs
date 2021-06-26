// use scanners::Scanner;
use std::io::stdin;
use std::process;
// use std::io::{stdout, BufWriter, Write};

#[derive(Default)]
struct Scanner {
    buffer: Vec<String>,
}
impl Scanner {
    fn next<T: std::str::FromStr>(&mut self) -> T {
        loop {
            if let Some(token) = self.buffer.pop() {
                return token.parse().ok().expect("Failed parse");
            }
            let mut input = String::new();
            stdin().read_line(&mut input).expect("Failed read");
            self.buffer = input.split_whitespace().rev().map(String::from).collect();
        }
    }
}

fn main() {
    // bit_plus();
    // maximus();
    replace_string();
}

#[allow(dead_code)]
fn replace_string() {
    // let mut scan = Scanner::default();
    // let n = scan.next::<i64>();
    // // let k = scan.next::<i64>();
    // let a: Vec<i64> = (0..n).map(|_| scan.next()).collect();
    let mut s = String::from("some stirng");
    // println!("{}", s.replace("s" | "o", to: &str));
    let VOWELS: Vec<char> = vec!['a', 'e', 'i', 'o', 'u'];
    for i in s[0..5] {
        if VOWELS.contains(i) {
            i = ".";
        }
    }
}

#[allow(dead_code)]
fn maximus() {
    use std::cmp::max;
    let mut scan = Scanner::default();
    let n = scan.next::<i64>();
    // let k = scan.next::<i64>();
    let a: Vec<i64> = (0..n).map(|_| scan.next()).collect();
    let sum = a.iter().fold(0, |acc, c| {
        print!("{} ", c + acc);
        max(acc, c + acc)
    });
}

#[allow(dead_code)]
fn bit_plus() {
    let mut scan = Scanner::default();
    let n = scan.next::<i64>();
    // let k = scan.next::<i64>();
    let a: Vec<String> = (0..n).map(|_| scan.next()).collect();
    let mut c = 0;
    for i in a.iter() {
        if i.contains("+") {
            c += 1;
        } else {
            c -= 1;
        }
    }
    println!("{}", c);
}
