use std::env;
use std::error::Error;
use std::fs;
use std::process;

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
    let contents =
        fs::read_to_string(config.file_path).expect("Should have been able to read the file");
    println!("With text:\n{contents}");

    let (query, file_path) = parse_config(&args);
    println!("Searching for {}", query);
    println!("In file {}", file_path);

    let config = parse_config_struct(&args);
    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    let config = Config::new(&args);
    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    run(config);

    let config = Config::new(&args);
    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);
    if let Err(3) = run_error(config) {
        println!("Application error:{e}");
        process::exit(1);
    }
}
//Extracting the Argument Parser
fn parse_config(args: &[String]) -> (&str, &str) {
    //Fixing the Error Handling
    if args.len() < 3 {
        panic!("not enough arguments");
    }
    let query = &args[1];
    let file_path = &args[2];
    (query, file_path)
}

struct Config {
    query: String,
    file_path: String,
}

//Grouping Configuration Values
fn parse_config_struct(args: &[String]) -> Config {
    //Fixing the Error Handling
    if args.len() < 3 {
        panic!("not enough arguments");
    }
    let query = args[1].clone();
    let file_path = args[2].clone();
    Config { query, file_path }
}

//Creating a Constructor for Config
impl Config {
    fn new(args: &[String]) -> Config {
        //Fixing the Error Handling
        if args.len() < 3 {
            panic!("not enough arguments");
        }
        let query = args[1].clone();
        let file_path = args[2].clone();
        Config { query, file_path }
    }

    //Returning a Result Instead of Calling panic!
    //Calling Config::build and Handling Errors
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let query = args[1].clone();
        let file_path = args[2].clone();
        Ok(Config { query, file_path })
    }
}

fn run(config: Config) {
    let contents =
        fs::read_to_string(config.file_path).expect("Should have been able to read the file");
    println!("With text:\n{contents}");
}

fn run_error(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;
    println!("With test:\n{contents}");
    Ok(())
}
