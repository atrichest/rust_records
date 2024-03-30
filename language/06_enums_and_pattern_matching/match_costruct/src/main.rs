//Patterns That Bind to Values
#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

//An enum and a match expression that has the variants of the enum as its patterns
enum Coin {
    Penny,
    Nickel,
    Dime,
    //Quarter,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        //Coin::Penny => 1,
        //the following code prints “Lucky penny!” every time the method is called with a Coin::Penny, but still returns the last value of the block, 1
        Coin::Penny => {
            println!("Lucky penny");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        //Coin::Quarter => 25,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

//Matching with Option<T>
//A function that uses a match expression on an Option<i32>
//Especially in the case of Option<T>, when Rust prevents us from forgetting to explicitly handle the None case,
//it protects us from assuming that we have a value when we might have null
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

//Catch-all Patterns and the _ Placeholder
fn add_fancy_hat() {
    println!("add_fancy_hat");
}
fn remove_fancy_hat() {
    println!("remove_fancy_hat");
}
fn move_player(num_spaces: u8) {
    println!("{num_spaces}");
}

fn re_roll() {
    println!("re_roll");
}

fn main() {
    println!("Penny {}", value_in_cents(Coin::Penny));
    println!("Nickel {}", value_in_cents(Coin::Nickel));
    println!("Dime {}", value_in_cents(Coin::Dime));
    println!("Quarter {}", value_in_cents(Coin::Quarter(UsState::Alaska)));
    println!(
        "Quarter {}",
        value_in_cents(Coin::Quarter(UsState::Alabama))
    );

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    println!("{:?},{:#?}", six, none);

    let dice_roll = 9;

    //the code that runs for the other arm uses the variable by passing it to the move_player function
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        other => move_player(other),
    }

    //_ is a special pattern that matches any value and does not bind to that value
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => re_roll(),
    }

    //nothing else happens on your turn if you roll anything other than a 3 or a 7
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => (),
    }
}
