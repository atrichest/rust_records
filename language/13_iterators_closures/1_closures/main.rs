use std::thread;
use std::time::Duration;

#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColor {
    Red,
    Blue,
}
struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        // "|| self.most_stocked()" is Closure expression
        user_preference.unwrap_or_else(|| self.most_stocked())
    }

    fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;

        for color in &self.shirts {
            match color {
                ShirtColor::Red => num_red += 1,
                ShirtColor::Blue => num_blue += 1,
            }
        }
        if num_red > num_blue {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}

fn generate_workout(intensity: u32, random_number: u32) {
    //"expensive_closure" is closure
    let expensive_closure = |num: u32| -> u32 {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    };

    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_closure(intensity));
        println!("Next, do {} situps!", expensive_closure(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!("Today,run for {} minutes!", expensive_closure(intensity));
        }
    }
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let store = Inventory {
        shirts: vec![ShirtColor::Red, ShirtColor::Blue, ShirtColor::Red],
    };
    let user_pref1 = Some(ShirtColor::Red);
    let giveaway1 = store.giveaway(user_pref1);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref1, giveaway1
    );
    let user_pref2 = None;
    let giveaway2 = store.giveaway(user_pref2);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref2, giveaway2
    );

    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;
    generate_workout(simulated_user_specified_value, simulated_random_number);

    //
    fn add_one_v1(x: u32) -> u32 {
        x + 1
    }
    let add_one_v2 = |x: u32| -> u32 { x + 1 };
    // rustfmt will remove "{" and "}"
    //let add_one_v3 = |x| {x + 1};

    // type must be known at this point
    //let add_one_v4 = |x| x + 1;
    let add_one_v4 = |x: u32| x + 1;

    let example_closure = |x| x;
    let s = example_closure(String::from("hello"));
    // error: because the closure was earlier called with an argument of type `String`
    //let n = example_closure(5);

    //borrowing immutably、borrowing mutably、taking ownership

    //we define a closure that captures an immutable reference to the vector named list
    //because it only needs an immutable reference to print the value
    let list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);
    let only_borrows = || println!("From closure: {:?}", list);
    println!("Before calling closure: {:?}", list);
    only_borrows();
    println!("After calling closure: {:?}", list);

    //captures a mutable reference
    let mut list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);
    //first borrow occurs due to use of `list` in closure
    let mut borrows_mutably = || list.push(7);
    //an immutable borrow to print isn’t allowed because no other borrows are allowed when there’s a mutable borrow
    //println!("Before calling closure: {:?}", list);
    borrows_mutably();
    println!("After calling closure: {:?}", list);

    let list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);
    //If you want to force the closure to take ownership of the values it uses in the environment
    //even though the body of the closure doesn’t strictly need ownership,
    //you can use the 'move' keyword before the parameter list.
    //
    //print the vector in a new thread rather than in the main thread
    //even though the closure body still only needs an immutable reference,
    //we need to specify that list should be moved into the closure by putting the move keyword at the beginning of the closure definition
    thread::spawn(move || println!("From thread:{:?}", list))
        .join()
        .unwrap();

    let mut list = [
        Rectangle {
            width: 10,
            height: 1,
        },
        Rectangle {
            width: 3,
            height: 5,
        },
        Rectangle {
            width: 7,
            height: 12,
        },
    ];
    list.sort_by_key(|r| r.width);
    println!("{:#?}", list);
    println!("\n");

    let mut sort_operations = vec![];
    let value = String::from("by key called");

    list.sort_by_key(|r| {
        //cannot move out of `value`, a captured variable in an `FnMut` closure
        //move occurs because `value` has type `String`, which does not implement the `Copy` trait
        //sort_operations.push(value);
        r.height
    });
    println!("{:#?}", list);
    println!("\n");

    list.sort_by_key(|r| {
        sort_operations.push(&value);
        r.width
    });
    println!("{:#?}", list);
    println!("{:#?}", sort_operations);
    println!("\n");

    let mut num_sort_operations = 0;
    list.sort_by_key(|r| {
        num_sort_operations += 1;
        r.height
    });
    println!("{:#?}, sorted in {num_sort_operations} operations", list);
}
