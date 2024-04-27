//The first rule is that the compiler assigns a lifetime parameter to each parameter that’s a reference.
//`fn foo<'a>(x:&'a i32)`
//`fn foo<'a,'b>(x:&'a i32,y:&'b i32)`

//The second rule is that, if there is exactly one input lifetime parameter, that lifetime is assigned to all output lifetime parameters
//`fn foo<'a>(x:&'a i32)->&'a i32`
//`fn foo<'a,'b>(x:&'a i32,y:&'b i32) -> &'a i32` the output lifetime must be assigned 'a or 'b

//The third rule is that, if there are multiple input lifetime parameters,
//but one of them is &self or &mut self because this is a method,
//the lifetime of self is assigned to all output lifetime parameters.

//the first rule example
//A function we defined that compiled without lifetime annotations,
//even though the parameter and return type are references
//fn first_word<'a>(s: &'a str) -> &'a str {
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
//the second rule example
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
//the third example
//Lifetime Annotations in Method Definitions
#[derive(Debug)]
struct ImportantExcerpt<'a> {
    part: &'a str,
}

#[allow(unused)]
impl<'a> ImportantExcerpt<'a> {
    //only parameter is a reference to self and whose return value is an i32, which is not a reference to anything
    //fn level<'d>(&'d self) -> i32 {
    fn level(&self) -> i32 {
        3
    }
}

#[allow(unused)]
impl<'a> ImportantExcerpt<'a> {
    //the third rule
    //fn announce_and_return_part<'d, 'b>(&'d self, announcement: &'b str) -> &'d str {
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}

#[allow(unused)]
fn main() {
    let my_string = String::from("hello world");

    // first_word works on slices of `String`s
    let word = first_word(&my_string[..]);
    println!("{word}");
    let my_string_literal = "hello world";

    // first_word works on slices of string literals
    let word = first_word(&my_string_literal[..]);
    println!("{word}");

    // Because string literals *are* string slices already,
    // this works too, without the slice syntax!
    let word = first_word(my_string_literal);
    println!("{word}");

    //Using the longest function with references to String values that have different concrete lifetimes
    let string1 = String::from("long string is long");
    {
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {}", result);
    }

    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };
    println!("{:?}", i);

    //The Static Lifetime
    //'static, which denotes that the affected reference can live for the entire duration of the program
    let s: &'static str = "I have a static lifetime.";
}
