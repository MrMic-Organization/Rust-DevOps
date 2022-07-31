#![allow(unused)]

use std::collections::HashMap;
use unicode_segmentation::UnicodeSegmentation;

fn main() {
    // let a = [1, 2, 3];
    // let mut v: Vec<i32> = Vec::new();
    // v.push(1);
    // v.push(2);
    // v.push(3);

    // let third = &v[2];
    // println!("The third element is {}", third);
    let mut v2 = vec![1, 2, 3, 4, 5];

    match v2.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("there is no third element."),
    }

    for i in &mut v2 {
        // println!("{}", i);
        *i += 50;
        println!("{} ", i);
    }

    println!("=====================================");
    // STRINGS
    let s1 = String::new();
    let s2 = "initial contents";
    let s3 = s2.to_string();
    let s4 = String::from("initial content");

    let mut s = String::from("foo");
    s.push_str("bar");
    s.push('!');

    println!("=====================================");
    // HASHMAPS
    let blue = String::from("Blue");
    let yellow = String::from("yellow");

    let mut scores: HashMap<String, i32> = HashMap::new();

    scores.insert(blue, 10);
    scores.insert(yellow, 30);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name);

    for (key, value) in &scores {
        println!("{} {}", key, value);
    }

    let text = "Hello world wonderful world";
    let mut map = HashMap::new();

    for world in text.split_whitespace() {
        let count = map.entry(world).or_insert(0);
        *count += 1;
    }
    println!("{:?}", map);
}
