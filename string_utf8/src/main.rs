#[allow(unused_variables)]

fn main() {
    let data = "initial contents";

    let s = data.to_string();

    // the method also works on a literal directly:
    let s = "initial contents".to_string();
    println!("{}", s);

    for c in "Здравствуйте".chars() {
        print!("{}", c);
    }
}
