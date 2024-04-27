use std::fs;
use std::fs::File;
use std::io::ErrorKind;
use std::io::{self, Read};

fn main() {
    // the Result enum is defined as having two variants, Ok and Err, as follows
    // enum Result<T, E> {
    //     Ok(T),
    //     Err(E),
    //}

    // use_match();

    //match_diff_errors();

    //unwrap_or_else_method();

    //unwrap_method();

    //expect_method();
    assert_eq!(
        last_char_of_first_line("Hello, world\nHow are you today?"),
        Some('d')
    );

    assert_eq!(last_char_of_first_line(""), None);
    assert_eq!(last_char_of_first_line("\nhi"), None);
}

//Using a match expression to handle the Result variants that might be returned
#[allow(unused)]
fn use_match() {
    //The return type of File::open is a Result<T, E>
    let greeting_file_result = File::open("hello.txt");

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };
}

//Handling different kinds of errors in different ways
#[allow(unused)]
fn match_diff_errors() {
    //The return type of File::open is a Result<T, E>
    let greeting_file_result = File::open("hello.txt");

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error);
            }
        },
    };
}

//Alternatives to Using match with Result<T, E>
#[allow(unused)]
fn unwrap_or_else_method() {
    // using closures and the unwrap_or_else method
    let greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|e| {
                panic!("Problem creating the file: {:?}", e);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });
}

//without a hello.txt file, the Result is the Err variant, unwrap will call the panic! macro
#[allow(unused)]
fn unwrap_method() {
    let greeting_file = File::open("hello.txt").unwrap();
}

//The error message used by expect in its call to panic! will be the parameter that we pass to expect,
//rather than the default panic! message that unwrap uses.
#[allow(unused)]
fn expect_method() {
    //In production-quality code, most Rustaceans choose expect rather than unwrap
    //and give more context about why the operation is expected to always succeed.
    //That way, if your assumptions are ever proven wrong, you have more information to use in debugging.
    let greeting_file =
        File::open("hello.txt").expect("hello.txt should be included in this project");
}

//A function that returns errors to the calling code using match
//When a function’s implementation calls something that might fail,
//instead of handling the error within the function itself,
//you can return the error to the calling code so that it can decide what to do
#[allow(unused)]
fn read_username_from_file() -> Result<String, io::Error> {
    let username_file_result = File::open("hello.txt");

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}

//A function that returns errors to the calling code using the ? operator
#[allow(unused)]
fn read_username_from_file_short() -> Result<String, io::Error> {
    let mut username_file = File::open("hello.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}

// Chaining method calls after the ? operator
#[allow(unused)]
fn read_username_from_file_after() -> Result<String, io::Error> {
    let mut username = String::new();

    File::open("hello.txt")?.read_to_string(&mut username)?;

    Ok(username)
}

//need add  ”use std::fs;“
//Using fs::read_to_string instead of opening and then reading the file
#[allow(unused)]
fn read_username_from_file_shortest() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}

// Using the ? operator on an Option<T> value
#[allow(unused)]
fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}
