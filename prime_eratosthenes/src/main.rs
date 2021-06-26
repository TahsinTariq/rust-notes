use std::time::Instant;

fn main() {
    let now = Instant::now();
    // println!("{:?}", now.);
    let n = 1_000_000_0;
    // for i in (50..n).step_by(5){
    //     println!("{}", i);
    // }
    let r = eratosthenes(n);
    // println!("{:?}", r);
    println!("number of primes found: {}", r.len());
    println!("last prime found: {}", r[r.len() - 1]);
    // println!("elapsed time: {:?}", now.elapsed());
    println!("elapsed time: {:?}", Instant::now().duration_since(now));
}

fn eratosthenes(n: i64) -> Vec<i64> {
    let mut primes = vec![true; (n) as usize];
    let limit = (n as f64).sqrt();
    for i in 2..(limit as i64) {
        if primes[i as usize] {
            for j in (i * i..n).step_by(i as usize) {
                primes[j as usize] = false;
            }
        }
    }

    let mut result = vec![];
    for (i, p) in primes.iter().enumerate() {
        if *p {
            result.push(i as i64);
        }
    }
    result.remove(0);
    result.remove(0);
    result
}
