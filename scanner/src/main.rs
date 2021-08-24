// use scanners::Scanner;
use std::io::stdin;
#[allow(unused_imports)]
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
    // replace_string();
    // petya_and_str()
    // b_matrix();
    // helpful_mafz();
    word_capitalization2();
}
#[allow(dead_code)]
fn word_capitalization() {
    let mut scan = Scanner::default();
    let s = scan.next::<String>();
    // let mut s = "something".to_string();
    let p: String = s
        .chars()
        .enumerate()
        .map(|(i, c)| {
            if i == 0 {
                c.to_uppercase().next().unwrap()
            } else {
                c
            }
        })
        .collect();
    println!("{}", p);
}

#[allow(dead_code)]
fn word_capitalization2() {
    let mut scan = Scanner::default();
    let mut s = scan.next::<String>();
    // let mut s = "something".to_string();
    let c = s.to_uppercase().chars().next().unwrap();
    // let c = s.to_uppercase().chars().nth(0).unwrap();
    s.replace_range(0..1, &c.to_string());
    println!("{}", s);
}

#[allow(dead_code)]
fn helpful_mafz() {
    let mut scan = Scanner::default();
    let s = scan.next::<String>();
    let mut p: Vec<&str> = s.split("+").collect();
    p.sort_unstable();
    // let mut pp: Vec<i32> = p.iter().map(|x| x.trim().parse::<i32>().unwrap()).collect();
    println!("{}", p.join("+"));
}

#[allow(dead_code)]
fn b_matrix() {
    let mut scan = Scanner::default();
    for i in 0..5 {
        let s: Vec<i32> = (0..5).map(|_| scan.next()).collect();
        for (j, n) in s.iter().enumerate() {
            if n == &1 {
                // println!("{}", ((i - 2) as i32).abs() + ((j - 2) as i32).abs());
                println!("{}", (j as i32 - 2).abs() + (i as i32 - 2).abs());
            }
        }
    }
}

#[allow(dead_code)]
fn petya_and_str() {
    let mut scan = Scanner::default();
    let s = scan.next::<String>().to_lowercase();
    let s1 = scan.next::<String>().to_lowercase();
    use std::cmp::Ordering;
    println!(
        "{}",
        match s.cmp(&s1) {
            Ordering::Less => -1,
            Ordering::Equal => 0,
            Ordering::Greater => 1,
        }
    );
}

#[allow(dead_code)]
fn replace_string() {
    let mut scan = Scanner::default();
    let s = scan.next::<String>();
    let VOWELS: Vec<char> = vec!['a', 'e', 'i', 'o', 'u', 'y'];
    let st: String = s
        .to_lowercase()
        .chars()
        .filter(move |item| !VOWELS.contains(item))
        .map(|item| format!(".{}", item))
        .collect::<Vec<String>>()
        .join("");
    println!("{}", st);
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
