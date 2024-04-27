use rand::Rng;
use std::cmp::Ordering;
use std::io;
use std::net::IpAddr;

fn main() {
    //127.0.0.1 可以确定不会触发异常
    //如果改为用户输入，则可能引发异常
    let home: IpAddr = "127.0.0.1"
        .parse()
        .expect("Hardcoded IP address should be valid");

    //猜数字
    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please input your guess.");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        let guess: Guess = match guess.trim().parse() {
            Ok(num) => Guess::new(num),
            Err(_) => continue,
        };

        match guess.value().cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }

    pub struct Guess {
        value: i32,
    }

    impl Guess {
        pub fn new(value: i32) -> Guess {
            if value < 1 || value > 100 {
                panic!("Guess value must be between 1 and 100,got {}", value);
            }
            Guess { value }
        }
        //getter
        //value is private
        //想获取value 必须先Guess::new instance of Guess
        //code outside the module must use the Guess::new function to create an instance of Guess
        pub fn value(&self) -> i32 {
            self.value
        }
    }
}
