//defining structs
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

//defining methods
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn width(&self) -> bool {
        self.width > 0
    }

    fn can_hold(&self, rect: &Rectangle) -> bool {
        self.width > rect.width && self.height > rect.height
    }
}

//Multiple impl Blocks
//Each struct is allowed to have multiple impl blocks
impl Rectangle {
    //Rectangle::square(10);
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let rectangle = Rectangle {
        width: 10,
        height: 20,
    };

    dbg!(rectangle.area());
    dbg!(&rectangle.area());
    println!(
        "The area of the rectangle is {} square pixels.",
        rectangle.area()
    );

    dbg!(rectangle.width());
    dbg!(rectangle.width);
    if rectangle.width() {
        println!(
            "The rectangle has a nonzero width; it is {}",
            rectangle.width
        );
    }

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };
    //This automatic referencing behavior works because methods have a clear receiver—the type of self.
    //Given the receiver and name of a method,
    //Rust can figure out definitively whether the method is reading (&self),
    //mutating (&mut self),
    //or consuming (self).
    dbg!(rect1.can_hold(&rect2));
    dbg!(&rect1.can_hold(&rect3));
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", &rect1.can_hold(&rect3));

    let rect3 = Rectangle::square(30);
    dbg!(rect3);
}
