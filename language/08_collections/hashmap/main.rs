fn main() {
    //Creating a new hash map and inserting some keys and values
    use std::collections::HashMap;
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    //Accessing the score for the Blue team stored in the hash map
    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);
    println!("{team_name}'s score is {score}");

    //iterate over each key/value pair in a hash map
    for (key, value) in &scores {
        println!("{key}: {value}");
    }

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    //Showing that keys and values are owned by the hash map once they’re inserted
    let mut map = HashMap::new();
    map.insert(field_name, field_value); //之后 field_name、field_value 不可访问
                                         //field_name and field_value are invalid at this point
    println!("1 {:?}", map);

    //Replacing a value stored with a particular key
    let old_name = String::from("Favorite color");
    let new_value = String::from("Red");
    map.insert(old_name, new_value);
    println!("2 {:?}", map);

    //Using the entry method to only insert if the key does not already have a value
    scores.entry(String::from("Cyan")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);
    println!("3 {:?}", scores);

    //Counting occurrences of words using a hash map that stores words and counts
    let text = "hello world wanderful world";
    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        //count 指向map value的指针
        let count = map.entry(word).or_insert(0);
        //*count 解指针，将内存位置上的值+1
        *count += 1;
    }
    println!("{:?}", map);
}
