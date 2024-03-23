fn main() {
    let mut s = String::from("hello");
    s.push_str(",world!");
    println!("{s}");
    //s包含指向字符串的指针，s 出栈 s2 入栈。可以避免多个指针指向字符串。此行后s不可访问
    let s2 = s;
    println!("{s2}");
    let s3 = s2.clone();
    println!("s2={s2},s3={s3}");

    let x = 3;
    //基础数据直接分配到栈上，x不会像s一样出栈
    let y = x;
    println!("x={x},y={y}");

    let s = String::from("hello");
    takes_ownership(s); // 作为参数传递给函数，s出栈，作为参数入栈，函数返回后，s不可访问

    let x = 5;
    makes_copy(x); //基础数据类型直接分配到栈上，x不会出栈仍可访问
    println!("{x}");

    let s1 = gives_ownership(); //s1 指向函数返回值入栈
    println!("{s1}");

    let s2 = String::from("hello");
    let s3 = takes_and_gives_back(s2); //s2出栈、作为参数传递给函数入栈，函数返回值入栈 赋给s3
    println!("{s3}");

    let (s4, len) = calculate_length(s3);
    println!("The length of '{s4}' is {len}");
}

fn takes_ownership(some_string: String) {
    println!("{some_string}");
}
fn makes_copy(some_integer: i32) {
    println!("{some_integer}");
}

fn gives_ownership() -> String {
    let some_string = String::from("yours");
    some_string
}
fn takes_and_gives_back(a_string: String) -> String {
    a_string
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}
