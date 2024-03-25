fn main() {
    let mut s = String::from("hello world");
    let word = first_word_noslice(&s); //word will get the value 5
    s.clear(); // this empties the String, making it equal to ""
               // word still has the value 5 here, but there's no more string that
               // we could meaningfully use the value 5 with. word is now totally invalid!
    println!("the space index of '{}' is {}.", s, word);

    //A string slice is a reference to part of a String
    let mut s = String::from("hello world");
    let hello = &s[0..5];
    let hello2 = &s[..5];

    let len = s.len();
    let world = &s[6..11];
    let world2 = &s[6..len];
    let world3 = &s[6..];
    println!("{hello} {hello2} {world} {world2} {world3}");

    let hw = &s[..];
    println!("{hw}");

    let mut s = String::from("hello world");
    let word = first_word_slice(&s);
    //s.clear(); //error
    println!("the first word is '{word}'");

    s.push_str(" hello rust");
    let len = s.len();
    println!("the first word is '{}'", first_word(&s[..len]));
    println!("the first word is '{}'", first_word(&s));

    let literal_str = "Hello World";
    println!("the first word is '{}'", first_word(&literal_str[..]));
    println!("the first word is '{}'", first_word(&literal_str));
    println!("the first word is '{}'", first_word(literal_str));
    println!("the first word is '{}'", first_word_lifetime(literal_str));

    //other slice
    let a = [1, 2, 3, 4, 5, 6];
    let slice = &a[1..3]; //type &[i32]
    assert_eq!(slice, &[2, 3]);
}

//The first_word function that returns a byte index value into the String parameter
fn first_word_noslice(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    s.len()
}

fn first_word_slice(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    return &s[..];
}

//Improving the first_word function by using a string slice for the type of the s parameter
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}

//lifetime
fn first_word_lifetime<'a>(s: &'a str) -> &'a str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}
