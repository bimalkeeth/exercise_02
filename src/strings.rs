#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_assignments)]

use std::collections::HashMap;

fn main() {
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used

    println!("{}",s3);


    for c in "Здравствуйте".chars() {
        println!("{c}");
    }

    for c in "Здравствуйте".bytes() {
        println!("{c}");
    }


    let mut score:HashMap<String,i32>=HashMap::new();
    score.insert(String::from("Blue"),10);
    score.insert(String::from("Yellow"),20);

    let ds=score.get(
        &String::from("Yellow")
    ).copied().unwrap_or(0);

    println!("{}",ds);
    score.entry(String::from("Red")).or_insert(50);

    for (key, value) in &score {
        println!("{key}: {value}");
    }
}

