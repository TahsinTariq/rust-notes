#[macro_use]
extern crate colored;

use colored::*;

fn main() {
    let v : Vec<i32> = Vec::new();
    let v = vec![1, 2, 3];
    println!("{} {:?}", "Using the vec! macro:".red().bold(), v);
    
    let mut v = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);
    println!("{} {:?}", "Adding using push:".blue(), v);
    
    println!("{}", "Accessing elements using [] and get()".cyan().bold());
    let v = vec![1, 2, 3, 4, 5];
    
    let third: &i32 = &v[2];
    println!("The third element is {}", third);
    
    match v.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }
    
    let does_not_exist = v.get(100);
    println!("{:?}", does_not_exist);
    // let does_not_exist = &v[100];
    
    
    // Immutable looping
    println!("{}", "Immutable looping: ".purple().bold());
    let v = vec![100, 32, 57];
    for i in &v {
        println!("{}", i);
    }
    
    
    // Mutable looping
    println!("{}", "Mutable looping and changing values: ".purple().bold());
    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
    }
    println!("{:?}", v);
    
    
    // using enum to store multiple types of values in a vector
    println!("{}", "Multiple types of values in a vector: ".green().bold());

    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    // TODO: implement fmt::Display for SpreadsheetCell
    
    // use std::fmt;
    // impl fmt::Display for SpreadsheetCell {
    //     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    //         // write!(f, "{}", self.0)
    //         match *self{
    //             SpreadsheetCell::Int => write!(f, "Int : {}", self), 
    //             SpreadsheetCell::Float => write!(f, "Float : {}", self), 
    //             SpreadsheetCell::Int => write!(f, "String : {}", self), 
    //         }
    //     }
    // }
    
    
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
    println!("{:?}", row)
}
