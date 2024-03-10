pub fn add_two(a: i32) -> i32 {
    a + 2
}

//Rust’s privacy rules do allow you to test private functions
//Note that the internal_adder function is not marked as pub.
//Tests are just Rust code, and the tests module is just another module.
//As we discussed in the “Paths for Referring to an Item in the Module Tree” section, items in child modules can use the items in their ancestor modules.
//In this test, we bring all of the test module’s parent’s items into scope with use super::*, and then the test can call internal_adder.
//If you don’t think private functions should be tested,there’s nothing in Rust that will compel you to do so.
fn internal_adder(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn internal() {
        assert_eq!(4, internal_adder(2, 2));
    }
}
