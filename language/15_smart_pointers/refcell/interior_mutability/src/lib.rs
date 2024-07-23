//A Use Case for Interior Mutability: Mock Objects
// A library to keep track of how close a value is to a maximum value and warn when the value is at certain levels
pub trait Messenger {
    fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T> LimitTracker<'a, T>
where
    T: Messenger,
{
    pub fn new(messenger: &'a T, max: usize) -> LimitTracker<'a, T> {
        LimitTracker {
            messenger,
            value: 0,
            max,
        }
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;

        let percentage_of_max = self.value as f64 / self.max as f64;

        if percentage_of_max >= 1.0 {
            self.messenger.send("Error: You are over your quota!");
        } else if percentage_of_max >= 0.9 {
            self.messenger
                .send("Urgent warning: You've used up over 90% of your quota!");
        } else if percentage_of_max >= 0.75 {
            self.messenger
                .send("Warning: You've used up over 75% of your quota!");
        }
    }
}
//An attempt to implement a MockMessenger that isn’t allowed by the borrow checker
#[cfg(test)]
mod tests {

    use super::*;
    use std::cell::RefCell;

    struct MockMessenger {
        sent_messages: Vec<String>,
    }
    //Using RefCell<T> to mutate an inner value while the outer value is considered immutable
    struct MockMessengerPro {
        sent_messages: RefCell<Vec<String>>,
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger {
                sent_messages: vec![],
            }
        }
    }

    impl MockMessengerPro {
        fn new() -> MockMessengerPro {
            MockMessengerPro {
                sent_messages: RefCell::new(vec![]),
            }
        }
    }

    impl Messenger for MockMessenger {
        //fn send(&self, message: &str) {
        //`self` is a `&` reference, so the data it refers to cannot be borrowed as mutable
        //     self.sent_messages.push(String::from(message));
        // }

        //  method `send` has an incompatible type for trait
        //fn send(&mut self, message: &str) {
        //    self.sent_messages.push(String::from(message));
        //}

        fn send(&self, message: &str) {}
    }

    impl Messenger for MockMessengerPro {
        fn send(&self, message: &str) {
            self.sent_messages.borrow_mut().push(String::from(message));

            //Creating two mutable references in the same scope to see that RefCell<T> will panic

            //let mut one_borrow = self.sent_messages.borrow_mut();
            // already borrowed: BorrowMutError
            //let mut two_borrow = self.sent_messages.borrow_mut();
            //one_borrow.push(String::from(message));
            //two_borrow.push(String::from(message));
        }
    }

    #[test]
    #[ignore]
    fn it_sends_an_over_75_percent_warning_message() {
        let mock_messenger = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);
        limit_tracker.set_value(80);
        assert_eq!(mock_messenger.sent_messages.len(), 1);
    }

    #[test]
    fn it_sends_an_over_90_percent_warning_message() {
        let mock_messenger = MockMessengerPro::new();
        let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);
        limit_tracker.set_value(94);
        assert_eq!(mock_messenger.sent_messages.borrow().len(), 1);
    }
}
