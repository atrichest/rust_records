//enum List {
//   Cons(i32, Box<List>),
//   Nil,
//}

//A definition of List that uses Rc<T>
enum List {
    Cons(i32, Rc<List>),
    Nil,
}

use crate::List::{Cons, Nil};
use std::rc::Rc;

fn main() {
    //Demonstrating we’re not allowed to have two lists using Box<T> that try to share ownership of a third list
    //error
    //   let a = Cons(5, Box::new(Cons(10, Box::new(Nil))));
    //   let b = Cons(3, Box::new(a));
    //   let c = Cons(4, Box::new(a));

    //Printing the reference count
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("count after creating a = {}", Rc::strong_count(&a));
    let b = Cons(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));
    {
        let c = Cons(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));
}
