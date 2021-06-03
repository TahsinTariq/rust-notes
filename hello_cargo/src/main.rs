#[macro_use]
extern crate fstrings;
extern crate colored;

use colored::*;
use rand::Rng;
use std::cmp::Ordering;
use std::io::stdin;

fn main() {
    println!("{}", "insert A number ...".red().bold());
    // guess.pop();
    // println!("\n{} is an idiot", guess.trim());
    // let x = 1;
    // let y = 2;
    // println_f!("\nx : {x} , guess : {guess} and y : {y} ");
    let random = rand::thread_rng().gen_range(0, 50);
    println_f!("\na random number : {random}");
    // println_f!("your number is : {guess}");
    loop {
        println!("try another one");
        let mut guess = String::new();
        stdin().read_line(&mut guess).expect("pass");
        let guess: u32 = guess.trim().parse().expect("Oopsie! not a number");

        println_f!("your number is {guess}");

        match guess.cmp(&random) {
            Ordering::Less => println_f!("The number is too low"),
            Ordering::Greater => println_f!("The number is too high"),
            Ordering::Equal => {
                println_f!("'Tis a match !!!!!");
                break;
            }
        }
    }
}

// fn main() {
//     // let x = 5;
//     // let x = x + 1;
//     // println!("{}", x);
//     let tup: (i32, f64, u8) = (500, 6.4, 1);
//     let x = tup.0;
//     let y = (tup.1, tup.2);
//     println!("x: {} \ny :({}, {})", x, y.0, y.1);
// }
