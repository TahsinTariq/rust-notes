use std::thread;
use std::time::Duration;

fn main() {
    handle_join();
    sleep_sort();
    print!("{}", 0);
    println!("{}", "");
}

#[allow(unused)]
fn handle_join() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap();
}

// Sleep sort
#[allow(unused)]
fn sleep_sort() {
    println!("{}", "\n\nSorting:");
    let v = vec![10, 3, 2, 8, 7, 6, 4, 5];
    let v2: Vec<_> = v
        .into_iter()
        .map(|i| {
            thread::spawn(move || {
                thread::sleep(Duration::from_millis(i));
                print!("{} ", i);
            })
        })
        .collect();
    for t in v2 {
        t.join();
    }
}
