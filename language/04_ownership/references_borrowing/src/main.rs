fn main() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("The length of '{}' is {}.", s1, len);

    let mut s2 = String::from("hello");
    // refrences mut and imut 不能同时使用，r2_1在第九行后不能使用
    let r2_1 = &s2;
    println!("{}", r2_1);
    // 多个mut references 不能同时存在，r2 仅在11-14行内有效
    {
        let r2 = &mut s2;
        change(r2);
    }
    // 同第8行
    let r2_2 = &s2;
    println!("{}", r2_2);

    change(&mut s2);
    println!("{s2}");
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(",world!");
}

//fn dangle() -> &String {
//    //dangel returns a reference to a String
//    let s = String::from("Hell");
//    &s //we return a reference to the String s
//} //Here s goes out of scope and is droped. Its memory goes away.
//Danger!

//the solution is to return the String directly
fn no_dangle() -> String {
    let s = String::from("Hello");
    s
}
