fn fix_incorrect_order() {
    cook_order();
    //Calling a function using a relative path starting with super
    super::deliver_order();
}

fn cook_order() {}

//A struct with some public fields and some private fields
pub struct Breakfast {
    pub toast: String,
    seasonal_fruit: String,
}

impl Breakfast {
    pub fn summer(toast: &str) -> Breakfast {
        Breakfast {
            toast: String::from(toast),
            seasonal_fruit: String::from("peaches"),
        }
    }
}

//Designating an enum as public makes all its variants public
pub enum Appetizer {
    Soup,
    Salad,
}
