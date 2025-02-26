enum List {
    Cons(i32, Box<List>),
    Nil,
}

use crate::List::{Cons, Nil};

fn main() {
    //Storing an i32 value on the heap using a box
    let b = Box::new(5);
    println!("b = {b}");

    //Definition of List that uses Box<T> in order to have a known size
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
}
