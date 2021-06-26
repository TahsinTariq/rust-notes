fn main() {
    let v1 = vec![1, 2, 3, 4, 5, 6];
    for v in v1.iter().step_by(2).skip(2) {
        println!("{}", v);
    }
}
