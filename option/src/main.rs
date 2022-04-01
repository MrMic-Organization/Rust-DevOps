#![allow(unused)]

fn main() {
    // enum Option<T> {
    //     Some(T),
    //     None,
    // }

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    // println!("six => {}, none => {}", six, none);

    let some_value = Some(3);
    if let Some(3) = some_value {
        println!("three")
    }

    let some_string = Some("A string");

    let absent_number: Option<i32> = None;

    let x: i8 = 5;
    // let y: Option<i8> = Some(5);
    let y: Option<i8> = None;
    let sum: i8 = x + y.unwrap_or(0);
    println!("SUM = {}", sum);
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        // None => None,
        Some(i) => Some(i + 1),
        _ => None,
    }
}
