#[macro_use]
extern crate fstrings;
extern crate colored;

use colored::*;
use rand::Rng;
use std::cmp::Ordering;
use std::io::stdin;

fn main() {
    // hello_cargo();
    use rand::distributions::{Distribution, Uniform};

    let mut rng = rand::thread_rng();
    let die = Uniform::from(1..7);

    for _ in 0..15 {
        let throw = die.sample(&mut rng);
        // println!("Roll the die: {}", throw);
    }

    uniform_usage_generate_points();
}

#[allow(dead_code)]
fn uniform_usage_generate_points() {
    use rand::{thread_rng, Rng};

    use rand::distributions::Uniform;

    let mut rng = thread_rng();
    let side = Uniform::new(-10.0, 10.0);

    // sample between 1 and 10 points
    for _ in 0..rng.gen_range(1..=10) {
        // sample a point from the square with sides -10 - 10 in two dimensions
        let (x, y) = (rng.sample(side), rng.sample(side));
        println!("Point: {}, {}", x, y);
    }
}

#[allow(dead_code)]
fn hello_cargo() {
    println!("{}", "insert A number ...".red().bold());
    // guess.pop();
    // println!("\n{} is an idiot", guess.trim());
    // let x = 1;
    // let y = 2;
    // println_f!("\nx : {x} , guess : {guess} and y : {y} ");
    let random = rand::thread_rng().gen_range(0..50);
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
