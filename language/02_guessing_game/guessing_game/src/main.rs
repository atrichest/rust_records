use rand::Rng;
//Specifying a nested path to bring multiple items with the same prefix into scope
use std::{cmp::Ordering,io}
//use std::cmp::Ordering;
//use std::io;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    let mut guess = String::new();

    loop {
        guess.clear();

        println!("Please input your guess.");

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        //Handling Invalid Input
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("You guessed: {guess}");
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            //Quitting After a Correct Guess
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
    println!("The secret number is: {secret_number}")
}
