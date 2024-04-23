//Methods that Produce Other Iterators
fn main() {
    let v1: Vec<i32> = vec![1, 2, 3];

    //Calling the iterator adaptor map to create a new iterator
    //warning: unused `Map` that must be used
    //The code doesn’t do anything;
    //the closure we’ve specified never gets called.
    //The warning reminds us why: iterator adaptors are lazy, and we need to consume the iterator here
    v1.iter().map(|x| x + 1);

    //Calling the map method to create a new iterator and then calling the collect method to consume the new iterator and create a vector
    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();
    assert_eq!(v2, vec![2, 3, 4]);
}
