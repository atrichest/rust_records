//Tuple structs are useful
//when you want to give the whole tuple a name and make the tuple a different type from other tuples,
//and when naming each field as in a regular struct would be verbose or redundant.
//
//Each struct you define is its own type,
//even though the fields within the struct might have the same types
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

//Unit-Like Structs Without Any Fields
//Unit-like structs can be useful when you need to implement a trait on some type
//but don’t have any data that you want to store in the type itself
//
//see in Chapter 10 how to define traits and implement them on any type
//
//declaring a unit struct named AlwaysEqual
struct AlwaysEqual;

fn main() {
    //black and origin values are different types because they’re instances of different tuple structs
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    //instantiating a unit struct named AlwaysEqual
    let subject = AlwaysEqual;
}
