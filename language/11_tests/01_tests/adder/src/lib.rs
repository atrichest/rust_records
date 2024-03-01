pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        /*       if value < 1 || value > 100 {
           panic!("Guess value must be between 1 and 100,got {}.", value);
        }
        */
        if value < 1 {
            panic!(
                "Guess value must be greater than or equal to 1,got {}.",
                value
            );
        } else if value > 100 {
            panic!(
                "Guess value must be less than or equal to 100,got {}.",
                value
            );
        }
        Guess { value }
    }
}

pub fn greeting(name: &str) -> String {
    format!("Hello {}!", name)
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub fn add_two(a: i32) -> i32 {
    a + 2
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn exploration() {
        assert_eq!(2 + 2, 4);
    }

    //Each test is run in a new thread, and when the main thread sees that a test thread has died,the test is marked as failed. In Chapter 9, we talked about how the simplest way to panic is to call the panic! macro.
    #[test]
    #[ignore]
    fn another() {
        panic!("Make this test fail");
    }

    //Checking Results with the assert! Macro
    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };
        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };
        assert!(!smaller.can_hold(&larger));
    }

    //Testing Equality with the assert_eq! and assert_ne! Macros
    #[test]
    fn it_adds_two() {
        assert_eq!(4, add_two(2));
    }

    #[test]
    fn it_adds_three() {
        assert_ne!(add_two(3), 4);
    }

    //Adding Custom Failure Messages
    #[test]
    fn greeting_contains_name() {
        let result = greeting("Carol");
        assert!(result.contains("Carol"));
    }

    #[test]
    fn greeting_contains_name_failure() {
        let result = greeting("Carol");
        assert!(
            result.contains("word"),
            "Greeting did not contain name,value was '{}'",
            result
        );
    }
    //Checking for Panics with should_panic
    #[test]
    #[should_panic(expected = "greater than or equal to 1")]
    fn greater_than_100() {
        Guess::new(101);
    }

    #[test]
    #[should_panic(expected = "less than or equal to 100")]
    fn less_than_1() {
        Guess::new(0);
    }

    #[test]
    #[should_panic(expected = "greater than or equal to 1")]
    fn less_than_1_test_ok() {
        Guess::new(0);
    }

    //Using Result<T, E> in Tests
    #[test]
    fn it_works_two_plus_two() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }
}
