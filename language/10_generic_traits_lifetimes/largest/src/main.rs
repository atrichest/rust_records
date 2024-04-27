fn largest_i32(list: &[i32]) -> &i32 {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest_char(list: &[char]) -> &char {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

//the largest function using generic type parameters; this doesn’t yet compile
fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list {
        // item > largest need T: std::cmp::PartialOrd
        if item > largest {
            largest = item;
        }
    }
    largest
}

//A Point<T> struct that holds x and y values of type T
//The fields x and y must be the same type because both have the same generic data type T
struct Point<T> {
    x: T,
    y: T,
}

//Implementing a method named x on the Point<T> struct that will return a reference to the x field of type T
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
    fn y(&self) -> &T {
        &self.y
    }
}

//An impl block that only applies to a struct with a particular concrete type for the generic type parameter T
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

//A Point<T, U> generic over two types so that x and y can be values of different types
struct PointPro<T, U> {
    x: T,
    y: U,
}

//A method that uses generic types different from its struct’s definition
impl<T, U> PointPro<T, U> {
    fn mixup<T1, U1>(self, other: PointPro<T1, U1>) -> PointPro<T, U1> {
        PointPro {
            x: self.x,
            y: other.y,
        }
    }
}

//Performance of Code Using Generics
//In Enum Definitions
#[derive(Debug)]
enum Option_i32 {
    Some(i32),
    None,
}
#[derive(Debug)]
enum Option_f64 {
    Some(f64),
    None,
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest_i32(&number_list);
    println!("The largest number is {}", result);
    assert_eq!(*result, 100);

    let result = largest(&number_list);
    println!("generic: The largest number is {}", result);
    assert_eq!(*result, 100);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest_char(&char_list);
    println!("The largest char is {}", result);
    assert_eq!(*result, 'y');

    let result = largest(&char_list);
    println!("generic: The largest char is {}", result);
    assert_eq!(*result, 'y');

    let integer = Point { x: 5, y: 10 };
    println!("integer.x={}", integer.x);
    let float = Point { x: 1.0, y: 4.0 };
    println!("float.x={}", float.x);

    //Implementing a method named x on the Point<T> struct that will return a reference to the x field of type T
    let integer = Point { x: 5, y: 10 };
    println!("integer.x()={}", integer.x());
    let float = Point { x: 1.0, y: 4.0 };
    println!("float.x()={}", float.x());
    //An impl block that only applies to a struct with a particular concrete type for the generic type parameter T
    println!("distance_from_origin={}", float.distance_from_origin());

    //数据类型要一致
    //let error_use = Point { x: 1, y: 1.0 };
    let int_float = PointPro { x: 1, y: 2.1 };
    println!("int = {},float = {}", int_float.x, int_float.y);

    //A method that uses generic types different from its struct’s definition
    let p1 = PointPro { x: 5, y: 10.4 };
    let p2 = PointPro { x: "hello", y: "c" };
    let p3 = p1.mixup(p2);
    println!("p3.x={},p3.y={}", p3.x, p3.y);

    //Performance of Code Using Generics
    let integer = Some(5);
    let float = Some(5.0);
    println!("Some(5)={:?} Some(5.0)={:?}", integer, float);

    let integer = Option_i32::Some(5);
    let float = Option_f64::Some(5.0);
    println!(
        "Option_i32::Some(5)={:?}\nOption_f64::Some(5.0)={:?}",
        integer, float
    );
}
