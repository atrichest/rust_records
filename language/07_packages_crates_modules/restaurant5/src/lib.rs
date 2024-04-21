mod back_of_house;
mod front_of_house;

pub use crate::back_of_house::{Appetizer, Breakfast};
pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();

    let mut meal = Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);
    // The next line won't compile if we uncomment it;
    // we're not allowed to see or modify the private field seasonal_fruit that comes with the meal
    // meal.seasonal_fruit = String::from("blueberries");

    let order1 = Appetizer::Soup;
    let order2 = Appetizer::Salad;
}

fn deliver_order() {}
