struct MyBox<T>(T);
struct MyBox2<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> MyBox2<T> {
    fn new(x: T) -> MyBox2<T> {
        MyBox2(x)
    }
}

use std::ops::Deref;

impl<T> Deref for MyBox<T> {
    //The type Target = T; syntax defines an associated type for the Deref trait to use
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> Deref for MyBox2<T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.0
    }
}

fn hello(name: &str) {
    println!("Hello, {name}!");
}

fn main() {
    //Using the dereference operator to follow a reference to an i32 value
    let x = 5;
    let y = &x;
    assert_eq!(5, x);
    assert_eq!(5, *y);

    //Using the dereference operator on a Box<i32>
    let x = 5;
    let y = Box::new(x);
    assert_eq!(5, x);
    assert_eq!(5, *y);

    //Implementing Deref on MyBox<T>
    let x = 5;
    let y = MyBox::new(x);
    assert_eq!(5, x);
    assert_eq!(5, *y);

    //Calling hello with a reference to a MyBox<String> value, which works because of deref coercion
    let m = MyBox::new(String::from("Rust"));
    hello(&m);

    //The code we would have to write if Rust didn’t have deref coercion
    let m = MyBox2::new(String::from("Rust"));
    hello(&(*m)[..]);
}
