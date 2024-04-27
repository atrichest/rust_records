use std::error::Error;
use std::fs::File;

// Changing main to return Result<(), E> allows the use of the ? operator on Result values
#[allow(unused)]
fn main() -> Result<(), Box<dyn Error>> {
    let greeting_file = File::open("hello.txt")?;
    Ok(())
}
