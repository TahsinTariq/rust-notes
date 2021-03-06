#![allow(unused)]
use std::fs::File;
use std::io;
use std::io::Read;

// fn read_username_from_file() -> Result<String, io::Error> {
//     let f = File::open("hello.txt");
//     let mut f = match f {
//         Ok(file) => file,
//         Err(e) => return Err(e),
//     };
//     let mut s = String::new();
//     match f.read_to_string(&mut s) {
//         Ok(_) => Ok(s),
//         Err(e) => Err(e),
//     }
// }

fn read_username_from_file() -> Result<String, io::Error> {
    let mut s = String::new();

    File::open("hello.txt")?.read_to_string(&mut s)?;

    Ok(s)
}

fn main() {
    // match read_username_from_file() {
    //     Ok(s) => println!("{}", s),
    //     Err(e) => println!("{}", e),
    // }

    // read_username_from_file().expect("error!!!");

    read_username_from_file().unwrap();
}
