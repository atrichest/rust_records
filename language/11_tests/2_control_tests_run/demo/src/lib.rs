//cargo test -- --help

//Running Tests in Parallel or Consecutively
//cargo test -- --test-threads=1

//Showing Function Output
//cargo test -- --show-output
fn prints_and_returns_10(a: i32) -> i32 {
    println!("I got the value {}", a);
    10
}

//Running a Subset of Tests by Name
//Running Single Tests
//cargo test add_two_and_two
//
//Filtering to Run Multiple Tests(tests’ names contain add)
//cargo test add
//
//Ignoring Some Tests Unless Specifically Requested
//#[ignore]
//cargo test -- --ignored
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
    fn this_test_will_pass() {
        let value = prints_and_returns_10(4);
        assert_eq!(10, value);
    }

    #[test]
    fn this_test_will_fail() {
        let value = prints_and_returns_10(8);
        assert_eq!(5, value);
    }

    #[test]
    fn add_two_and_two() {
        assert_eq!(4, add_two(2));
    }

    #[test]
    fn add_three_and_two() {
        assert_eq!(5, add_two(3));
    }

    #[test]
    fn one_hundred() {
        assert_eq!(102, add_two(100));
    }

    #[test]
    #[ignore]
    fn expensive_test() {
        //code that takes an hour to run
    }

    #[test]
    fn internal() {
        assert_eq!(4, internal_adder(2, 2));
    }
}
