#![allow(unused)]
fn main() {
    immutable();
    mutable();

    //Rust’s naming convention for constants is to use all uppercase with underscores between words
    //Constants are valid for the entire time a program runs, within the scope in which they were declared.
    // need #![allow(unused)]
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

    //shadowing
    shadowing();
    shadowing2();
}
fn immutable() {
    let x = 5;
    println!("The value of x is: {x}");
    //x = 6;
    //println!("The value of x is: {x}");
}
fn mutable() {
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");
}

fn shadowing() {
    let x = 5;
    let x = x + 1;
    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}"); //6*2=12
    }
    println!("The value of x is: {x}"); //5+1=6
}

fn shadowing2() {
    let spaces = "abc";
    println!("The value of spaces is: {spaces}");
    let spaces = spaces.len();
    println!("The value of spaces.len() is: {spaces}");
}
