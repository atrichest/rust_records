//cargo new restaurant2 --lib --vcs none
//A front_of_house module containing other modules that then contain functions

//use crate::front_of_house
mod front_of_house {
    //Exposing Paths with the pub Keyword
    //use crate::front_of_house::hosting;
    pub mod hosting {
        //use crate::front_of_house::hosting::add_to_waitlist;
        pub fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}

pub fn eat_at_restaurant() {
    //Absolute path
    crate::front_of_house::hosting::add_to_waitlist();
    //Relative path
    front_of_house::hosting::add_to_waitlist();

    //Order a breakfast in the summer with Rye toast
    let mut meal = back_of_house::Breakfast::summer("Rye");
    //Change our mind about what bread we'd like
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);
    // The next line won't compile if we uncomment it;
    // we're not allowed to see or modify the private field seasonal_fruit that comes with the meal
    // meal.seasonal_fruit = String::from("blueberries");

    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}

//Starting Relative Paths with super
fn deliver_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        //Calling a function using a relative path starting with super
        super::deliver_order();
    }
    fn cook_order() {}

    //Making Structs and Enums Public

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
}
