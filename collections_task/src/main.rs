// Given a list of integers, use a vector and return the mean (the average value), median (when sorted, the value in the middle position), and mode (the value that occurs most often; a hash map will be helpful here) of the list.
// Convert strings to pig latin. The first consonant of each word is moved to the end of the word and “ay” is added, so “first” becomes “irst-fay.” Words that start with a vowel have “hay” added to the end instead (“apple” becomes “apple-hay”). Keep in mind the details about UTF-8 encoding!
// Using a hash map and vectors, create a text interface to allow a user to add employee names to a department in a company. For example, “Add Sally to Engineering” or “Add Amir to Sales.” Then let the user retrieve a list of all people in a department or all people in the company by department, sorted alphabetically.

// https://codereview.stackexchange.com/questions/173338/calculate-mean-median-and-mode-in-rust

fn main() {
    let mut list = vec![0, 0, 2, 5, 8, 7, 1, 4, 3, 6, 9];
    println!("Average, Median and Mode -> ");
    stat(&mut list);
    let mut plist = vec!["first", "apple", "pig", "latin"];
    println!("Pig Latin -> ");
    for string in &mut plist {
        piglatin(string);
    }
}

fn stat(list: &mut [i32]) {
    println!(
        "\taverage: {}",
        list.iter().sum::<i32>() as f32 / list.len() as f32
    );
    println!(
        "\tmedian: {:?}",
        match {
            list.sort();
            list.get(list.len() / 2)
        } {
            None => panic!("found none"),
            Some(i) => i,
        }
    );
    use std::collections::HashMap;
    let mut map = HashMap::new();
    for i in list.iter() {
        // let count = map.entry(i).or_insert(0);
        // *count += 1;
        *map.entry(i).or_insert(0) += 1;
    }

    let map = map.into_iter();
    // println!(
    //     "mode: {:?}",
    //     match map.max_by_key(|&(_val, count)| count) {
    //         Some(i) => i.0,
    //         None => panic!("None"),
    //     }
    // );
    println!(
        "\tmode: {:?}",
        map.max_by_key(|&(_val, count)| count)
            .map(|(val, _count)| val)
            .expect("Nothing")
    );
}

fn piglatin(pstr: &str) {
    match &pstr[0..1] {
        "a" | "e" | "i" | "o" | "u" => println!("\t{}-hay", &pstr[0..]),
        _ => println!("\t{}-{}ay", &pstr[1..], &pstr[0..1]),
    }
}
