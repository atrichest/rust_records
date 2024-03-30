#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn main() {
    //A match that only cares about executing code when the value is Some
    let config_max = Some(3u8);
    match config_max {
        Some(max) => println!("The maxium is configured to be {}", max),
        _ => (),
    }
    //we could write this in a shorter way using if let
    if let Some(max) = config_max {
        println!("The maxium is configured to be {}", max);
    }

    //If you have a situation in which your program has logic that is too verbose to express using a match,
    //remember that if let is in your Rust toolbox as well.
    let mut count = 0;
    let coin = Coin::Penny;
    match coin {
        Coin::Quarter(state) => println!("State Quarter from {:?}", state),
        _ => count += 1,
    }
    println!("count = {}", count);

    let coin = Coin::Quarter(UsState::Alabama);
    if let Coin::Quarter(state) = coin {
        println!("State Quarter from {:?}", state);
    } else {
        count += 1;
    }
    println!("count = {}", count);
}
