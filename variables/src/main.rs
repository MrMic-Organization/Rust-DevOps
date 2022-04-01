#[allow(unused_variables)]

fn main() {
    // VARIABLES
    let x = 5;
    println!("The value of x is {}", x);
    let x = "six";
    println!("The value of x is {}", x);

    // const SUBSCRIBER_COUNT: u32 = 100_000;

    // DATA TYPES
    // INTEGERS
    let a = 98_222; // Decimal
    let b = 0xff; //Hex
    let c = 0o77; // Octal
    let d = 0b1111_0000; // Binary
    let e = b'A'; // Byte (u8 only)

    let f: u8 = 255;
    // FLOATING-POINT NUMBERS
    let f = 2.0;
    let g: f32 = 3.0;

    // BOOLEANS
    let t = true;
    // CHARACTERS
    let c = 'z';

    // COMPOUND TYPES
    let tup = ("Let's Get Rusty", 100_000);
    let (channel, sub_count) = tup;
    let sub_count = tup.1;
    // ARRAYS
    let error_codes = [200, 404, 500];
    let byte = [0; 8];

    // FUNCTIONS
    let sum = my_function(11, 22);
    println!("The sum is: {}", sum);

    // CONDITION
    let number = 5;
    if number < 10 {
        println!("First condition was true");
    } else if number < 22 {
        println!("Second condition was true")
    } else {
        println!("Condition was false");
    }
}

fn my_function(x: i32, y: i32) -> i32 {
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
    x + y
}
