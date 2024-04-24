use std::env;
use std::process;

use minigrep::Config;

//cargo run > output.log
//Now we see the error onscreen and output.log contains nothing, which is the behavior we expect of command line programs.
//
//cargo run -- To poem.txt > output.log
//We won’t see any output to the terminal, and output.log will contain our results
//
//IGNORE_CASE=1 cargo run -- name poem.txt > output.log
//PS> $Env:IGNORE_CASE=1; cargo run -- TO poem.txt > output.log
//PS> Remove-Item Env:IGNORE_CASE
fn main() {
    //reading a file
    // cargo run -- test poem.txt

    //Returning a Result Instead of Calling panic!
    //Calling Config::build and Handling Errors
    //Passing the return value of env::args to Config::build
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments:{err}");
        process::exit(1);
    });
    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);
    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error:{e}");
        process::exit(1);
    }
}
