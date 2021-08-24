use std::time::Instant;

fn main() {
    let now = Instant::now();
    // let n = 10_000_000;
    // let n = 15_485_864;
    let n = 990_000_000;
    let r = atkin(n);
    println!("number of primes found: {}", r.len() - 2);
    println!("last prime found: {}", r[r.len() - 1]);
    println!("elapsed time: {:?}", now.elapsed());
}

#[allow(clippy::vec_init_then_push)]
fn atkin(n: u64) -> Vec<u64> {
    let mut primes = vec![false; (n + 1) as usize];
    primes[2] = true;
    primes[3] = true;
    let limit = (n as f64).sqrt().ceil();
    for x in 1..(limit as u64) {
        for y in 1..(limit as u64) {
            let num = 4 * x * x + y * y;
            if num <= n && (num % 12 == 1 || num % 12 == 5) {
                primes[num as usize] = true;
            }
            let num = 3 * x * x + y * y;
            if num <= n && (num % 12 == 7) {
                primes[num as usize] = true;
            }
            if x > y {
                let num = 3 * x * x - y * y;
                if num <= n && (num % 12 == 11) {
                    primes[num as usize] = true;
                }
            }
        }
    }

    for i in 5..(limit as u64) {
        if primes[i as usize] {
            // result.push(i);
            for j in (i * i..n).step_by(i as usize) {
                primes[j as usize] = false;
            }
        }
    }
    let mut result = vec![2, 3];
    // result.push(2);
    // result.push(3);
    for (i, p) in primes.iter().enumerate() {
        if *p {
            result.push(i as u64);
        }
    }
    result
}
