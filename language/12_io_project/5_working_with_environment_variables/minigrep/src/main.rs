use std::env;
use std::process;

use minigrep::Config;

//IGNORE_CASE=1 cargo run -- name poem.txt
//PS> $Env:IGNORE_CASE=1; cargo run -- TO poem.txt
//PS> Remove-Item Env:IGNORE_CASE
fn main() {
    //reading a file
    // cargo run -- test poem.txt
    let args: Vec<String> = env::args().collect();

    //Returning a Result Instead of Calling panic!
    //Calling Config::build and Handling Errors
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments:{err}");
        process::exit(1);
    });
    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);
    if let Err(e) = minigrep::run(config) {
        println!("Application error:{e}");
        process::exit(1);
    }
}
