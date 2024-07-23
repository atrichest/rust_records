// Using Rc<RefCell<i32>> to create a List that we can mutate
#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}

use crate::List::{Cons, Nil};
use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    let value = Rc::new(RefCell::new(5));

    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));

    let b = Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));
    let c = Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));

    //The "value" uses the automatic dereferencing feature we discussed in Chapter 5 to dereference the Rc<T> to the inner RefCell<T> value.
    //The "borrow_mut" method returns a RefMut<T> smart pointer,
    //and we use the "*" dereference operator on it and change the inner value.
    //*((value).borrow_mut()) += 10;
    *value.borrow_mut() += 10;

    println!("a after = {a:?}");
    println!("b after = {b:?}");
    println!("c after = {c:?}");
}
