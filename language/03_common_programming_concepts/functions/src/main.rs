fn main() {
    let x = five();
    println!("The value of x is {x}");
    let x = plus_one(x);
    println!("The x plus one is {x}");

    let x: (i32, i32, i32) = plus_y(1, 2);
    let a = x.0;
    let b = x.1;
    let c = x.2;
    println!("The {a} plus {b} is {c}");

    let y = {
        let x = 3;
        //注意不带分号(semicolon)
        x + 1
    };
    println!("The value of y is {y}");
}

fn five() -> i32 {
    return 5;
}
fn plus_one(x: i32) -> i32 {
    //注意不带分号(semicolon)
    x + 1
}

fn plus_y(x: i32, y: i32) -> (i32, i32, i32) {
    return (x, y, x + y);
}
