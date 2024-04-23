fn main() {
    //Creating an iterator
    //In Rust, iterators are lazy, meaning they have no effect until you call methods that consume the iterator to use it up
    let v1 = vec![1, 2, 3];
    //The iterator is stored in the v1_iter variable. Once we’ve created an iterator, we can use it in a variety of ways.
    let v1_iter = v1.iter();

    //Using an iterator in a for loop
    //we iterated over an array using a for loop to execute some code on each of its items
    for val in v1_iter {
        println!("Got: {}", val);
    }

    //The Iterator Trait and the next Method
    //All iterators implement a trait named Iterator that is defined in the standard library. The definition of the trait looks like this
    pub trait Iterator {
        type Item;

        fn next(&mut self) -> Option<Self::Item>;

        // methods with default implementations elided
    }
}
