fn main() {
    // let s1 = gives_ownership();
    // let s2 = String::from("hello");
    // let s3 = takes_and_gives_back(s2);
    // println!("s1 = {}, s3 = {}", s1, s3);
    let mut s1 = String::from("hello");
    // let len = calculate_length(&s1);
    change(&mut s1);
    // println!("The length of '{}' is {}.", s1, len);
}

// fn gives_ownership() -> String {
//     let some_string = String::from("hello");
//     some_string
// }

// fn takes_and_gives_back(a_string: String) -> String {
//     a_string
// }

// fn calculate_length(s: &String) -> (usize) {
//     let length = s.len();

//     length
// }

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}