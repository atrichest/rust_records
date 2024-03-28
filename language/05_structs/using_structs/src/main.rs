fn area(width: u32, height: u32) -> u32 {
    width * height
}

//Refactoring with Tuples
fn area_tuple(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

//Refactoring with Structs
#[derive(Debug)] //Adding the attribute to derive the Debug trait and printing the Rectangle instance using debug formatting
struct Rectangle {
    width: u32,
    height: u32,
}

fn area_struct(rectangle: Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
fn area_struct_slice(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

fn main() {
    //Calculating the area of a rectangle specified by separate width and height variables
    let width1 = 30;
    let height1 = 50;

    println!(
        "The area of the rectangle is {} square pixels.",
        area(width1, height1)
    );

    let rect1 = (30, 50);

    println!(
        "The area of the rectangle is {} square pixels.",
        area_tuple(rect1)
    );

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    println!("rectangle is {:?}", &rect1);
    println!(
        "The area of the rectangle is {} square pixels.",
        area_struct(rect1) //rect1 moved here
    );

    //Calling the dbg! macro prints to the standard error console stream (stderr),
    //as opposed to println!, which prints to the standard output console stream (stdout)
    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };
    println!("rectangle is {:?}", &rect1);
    println!(
        "The area of rectangle is {} square pixels.",
        area_struct_slice(&rect1)

    dbg!(&rect1);
}
