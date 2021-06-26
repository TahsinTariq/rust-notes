// for small values
// fn main() {
//     let n : u128 = 34;
//     println!("{}", factorial(n));
// }

// fn factorial(n:u128)->u128{
//     if n<=1{
//         return 1;
//     }
//     return n*factorial(n-1);
// }

// for large values
// use num::{BigUint, One};

// fn factorial(n: usize) -> BigUint {
//     (1..=n).fold(BigUint::one(), |a, b| a * b)
// }

// fn main() {
//     println!("{}", factorial(100));
// }

fn main() {
    let x = 3.1415;
    let r = 8.0;
    println!("the area of the circle is {}", x * r * r);
}
