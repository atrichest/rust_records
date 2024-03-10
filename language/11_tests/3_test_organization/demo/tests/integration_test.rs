use demo;

mod sub;

//The three sections of output include the unit tests, the integration test, and the doc tests.
//Note that if any test in a section fails, the following sections will not be run.
//For example, if a unit test fails, there won’t be any output for integration and doc tests because those tests will only be run if all unit tests are passing.

//cargo test --test integration_test
#[test]
fn it_adds_two() {
    assert_eq!(4, demo::add_two(2));
}

#[test]
fn it_adds_two_with_sub() {
    sub::setup();
    assert_eq!(4, demo::add_two(2));
}
