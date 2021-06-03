fn main() {
    let a: [i32; 5] = [3; 5];
    println!("{}", 53u8);
    for i in 1..5 {
        println!("{}", i);
    }
    for i in a.iter() {
        print!("{}", i);
    }
    let x = {
        let mut p = 0;
        for i in 1..10 {
            p += i;
        }
        p
    };
    println!("\nOwO {}", x);
}

// fn main() {
//     let s = String::from("hello");
// }
//     s.push_str(", world");
