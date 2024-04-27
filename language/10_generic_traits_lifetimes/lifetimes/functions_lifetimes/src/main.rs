#[allow(unused)]
fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);

    //Using the longest function with references to String values that have different concrete lifetimes
    different_lifetimes();

    //A struct that holds a reference, requiring a lifetime annotation
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };

    //Generic Type Parameters, Trait Bounds, and Lifetimes Together
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result =
        longest_with_an_announcement(string1.as_str(), string2, "Today is someone's birthday!");
    println!("The longest string is {}", result);
}

//An implementation of the longest function that returns the longer of two string slices but does not yet compile
//the longest function to find the longer of two string slices
//```
//fn longest(x: &str, y: &str) -> &str {
//    if x.len() > y.len() {
//        x
//    } else {
//        y
//    }
//}
//```

//The longest function to find the longer of two string slices
//The longest function definition specifying that all the references in the signature must have the same lifetime 'a
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

//this would be a dangling reference because the value will go out of scope at the end of the function
//```
//fn longest<'a>(x: &str, y: &str) -> &'a str {
//    let result = String::from("really long string");
//    result.as_str()
//}
//```

//Using the longest function with references to String values that have different concrete lifetimes
fn different_lifetimes() {
    let string3 = String::from("long string is long");
    {
        let string4 = String::from("xyz");
        let result = longest(string3.as_str(), string4.as_str());
        println!("The longest string is {}", result);
    }
}

//Attempting to use result after string2 has gone out of scope
//```
//fn out_of_scope() {
//    let string5 = String::from("long string is long");
//    let result;
//    {
//        let string6 = String::from("xyz");
//        result = longest(string5.as_str(), string6.as_str());
//    }
//    // Attempting to use result after string6 has gone out of scope
//    println!("The longest string is {}", result);
//}
//```

//A struct that holds a reference, requiring a lifetime annotation
struct ImportantExcerpt<'a> {
    part: &'a str,
}

//Generic Type Parameters, Trait Bounds, and Lifetimes Together
use std::fmt::Display;

fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
