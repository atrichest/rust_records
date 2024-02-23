use std::env;

fn main() {
    //Reading the Argument Values
    // cargo run -- hello world
    let args: Vec<String> = env::args().collect();
    //dbg!(args); //args moved here

    //Saving the Argument Values in Variables
    let query = &args[1];
    let file_path = &args[2];

    println!("Searching for {}", query);
    println!("In file {}", file_path);
}
