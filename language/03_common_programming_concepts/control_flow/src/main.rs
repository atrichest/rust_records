fn main() {
    if_expression();
    multiple_conditions();
    if_in_let();
    return_value_from_loops();
    loop_label_break();
    while_loop();

    let arr: [i32; 5] = [10, 20, 30, 40, 50];
    looping_while(&arr);
    looping_each(&arr);
    nicer_looping();
}

fn if_expression() {
    let number = 3;
    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }
}

fn multiple_conditions() {
    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
}

fn if_in_let() {
    let condition = true;
    let number = if condition { 5 } else { 6 };
    // let number = if condition {5}else{"six");//if and else have incompatible types
    println!("The value of number is: {number}");
}

fn return_value_from_loops() {
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The result of loop is {result}");
}
fn loop_label_break() {
    let mut count = 0;
    'counting_up: loop {
        println!("count={count}");
        let mut remaining = 10;
        loop {
            println!("remianing={remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("End count={count}");
}

fn while_loop() {
    let mut number = 3;
    while number != 1 {
        println!("number={number}");
        number -= 1;
    }
    println!("LIFTOFF!");
}

fn looping_while(arr: &[i32]) {
    let mut index = 0;
    while index < arr.len() {
        println!("arr[{index}]={}", arr[index]);
        index += 1;
    }
}

fn looping_each(arr: &[i32]) {
    for element in arr {
        println!("{element}");
    }
}

fn nicer_looping() {
    for num in (1..4).rev() {
        println!("{num}");
    }
}
