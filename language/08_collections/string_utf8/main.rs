fn main() {
    //Creating a new, empty String
    let mut s = String::new();
    println!("{s}");

    // Using the to_string method to create a String from a string literal
    let data = "initial contents";
    let s = data.to_string();
    println!("{s}");
    // the method also works on a literal directly:
    let s = "initial contents".to_string();
    println!("{s}");

    //Using the String::from function to create a String from a string literal
    let s = String::from("initial contents");
    println!("{s}");

    //Remember that strings are UTF-8 encoded, so we can include any properly encoded data in them,
    let hello = String::from("السلام عليكم");
    println!("{hello}");
    let hello = String::from("Dobrý den");
    println!("{hello}");
    let hello = String::from("Hello");
    println!("{hello}");
    let hello = String::from("שָׁלוֹם");
    println!("{hello}");
    let hello = String::from("नमस्ते");
    println!("{hello}");
    let hello = String::from("こんにちは");
    println!("{hello}");
    let hello = String::from("안녕하세요");
    println!("{hello}");
    let hello = String::from("你好");
    println!("{hello}");
    let hello = String::from("Olá");
    println!("{hello}");
    let hello = String::from("Здравствуйте");
    println!("{hello}");
    let hello = String::from("Hola");
    println!("{hello}");

    //Appending a string slice to a String using the push_str method
    let mut s = String::from("foo");
    s.push_str("bar");
    println!("{s}");

    //Using a string slice after appending its contents to a String
    let s2 = "s";
    s.push_str(s2);
    println!("{s}");
    println!("{s2}");

    //Adding one character to a String value using push
    s.push('l');
    println!("{s}");

    //Using the + operator to combine two String values into a new String value
    let hello = String::from("Hello, ");
    let world = String::from("world!");
    let hello_world = hello + &world; //note 'hello' has been moved here and can no longer be used
    println!("{hello_world}");
    println!("{world}");
    // println!("{hello}"); //error!!!
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    println!("{}-{}-{}", s1, s2, s3);
    let s = format!("{s1}-{s2}-{s3}");
    println!("{s}");
    let s = s1 + "-" + &s2 + "-" + &s3;
    println!("{s}");

    //Attempting to use indexing syntax with a String
    //the type `String` cannot be indexed by `{integer}`
    //let s1=String::from("hello");
    //let h=s1[0];

    for c in "hello".bytes() {
        println!("b:{c}");
    }
    for c in "hello".chars() {
        println!("c:{c}");
    }

    println!("一个汉字占 3 bytes");
    let s = "厚德载物";
    let s1 = &s[3..6];
    println!("{s1}");
    for c in "厚德载物".bytes() {
        println!("b:{c}");
    }

    for c in "厚德载物".chars() {
        println!("c:{c}");
    }

    println!("一个俄文占2bytes");
    let s = "Здравствуйте";
    let s1 = &s[2..4];
    println!("{s1}");
    for c in "Здр".bytes() {
        println!("b:{c}");
    }
}
