fn main() {
    // let x = 4;
    let x = vec![1, 2, 3];

    let equal_to_x = |z: Vec<i32>| z == x;

    println!("Can't use x here: {:?}", x);

    // let y = 4;
    let y = vec![1, 2, 3];

    assert!(equal_to_x(y));
}

// FnOnce, FnMut, Fn
