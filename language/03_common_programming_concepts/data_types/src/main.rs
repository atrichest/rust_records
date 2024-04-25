use std::io;
//Rust has four primary scalar types: integers, floating-point numbers, Booleans, and characters
fn main() {
    integer();
    floating_point();
    numeric_operations();
    boolean_type();
    character_type();
    tuple_type();
    array_type();
}

fn integer() {
    //8bit
    let x: i8 = -0b1000_0000;
    println!("i8 min = {x}");
    let x: i8 = 0b0111_1111;
    println!("i8 max = {x}");
    //8bit
    let x: u8 = 0x00;
    println!("u8 min = {x}");
    let x: u8 = 0xff;
    println!("u8 max = {x}");
    //16bit
    let x: i16 = -0x8000;
    println!("i16 min = {x}");
    //16bit
    let x: i16 = 0x7fff;
    println!("i16 max = {x}");

    println!("32bits i32 u32");
    println!("64bits i64 u64");
    println!("128bits i128 u128");
    println!("arch isize usize: 64bits if you're on a 64-bit architecture and 32bits if you're on a 32-bit architecture.");

    println!("Decimal 98_222");
    println!("Hex 0xff");
    println!("Octal 0o77");
    println!("Binary 0b1111_0000");
    println!("Byte(u8 only) b'A'");
}

fn floating_point() {
    let x = 2.0; // f64
    println!("let x = {x};// The default type is f64");
    let y: f32 = 3.0; // f32
    println!("let y:f32 = {y};//f32");
}

fn numeric_operations() {
    //addition
    let sum = 5 + 10;
    println!("5+10={sum}");
    //subtraction
    let difference = 95.5 - 4.3;
    println!("95.5-4.3={difference}");
    //multiplication
    let product = 4 * 30;
    println!("4x30={product}");

    //division
    let quotient = 56.7 / 32.2;
    println!("56.7/32.2={quotient}");
    let truncated = -5 / 3; //results in -1
    println!("-5/3={truncated}");
    //remainder
    let remainder = 43 % 5;
    println!("43%5={remainder}");
    //results in -2
    let truncated = -5 % 3;
    println!("-5%3={}", truncated);
}

//Booleans are one byte in size
fn boolean_type() {
    let t = true;
    println!("let t={t}");
    //with explicit type annotation
    let f: bool = false;
    println!("let f:bool={f}");
}

//char type is four bytes in size and represents a Unicode Scalar Value;
fn character_type() {
    let c = 'z';
    let z: char = 'ℤ'; //with explicit type annotation
    let heart_eyed_cat = '😻';
}

fn tuple_type() {
    let tup = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The value of x is {}", x);
    println!("The value of y is {}", y);
    println!("The value of z is {}", z);
    let x: (i32, f64, u8) = (500, 6.4, 1);
    let five_hundred = x.0;
    println!("The value of x.0 is {}", five_hundred);
    let six_point_four = x.1;
    println!("The value of x.1 is {}", six_point_four);
    let one = x.2;
    println!("The value of x.2 is {}", one);
}

fn array_type() {
    let a = [1, 2, 3, 4, 5];
    let months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    println!("Please enter an array index.");
    let mut index = String::new();
    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");
    let index: usize = index.trim().parse().expect("Index entered wasn't a number");
    let element = a[index];
    println!("The value of the element at index {index},is:{element}");
}
