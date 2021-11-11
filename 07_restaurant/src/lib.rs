#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

// The semicolon tells Rust to load the module from a file with the same name
mod front_of_house;
mod back_of_house;

mod warehouse {
    pub fn get_ingredients() {}
}

fn serve_order() {}

// They can be used indistinctly
// use crate::front_of_house::hosting;
pub use self::front_of_house::hosting;
// By using pub use, external code can now call the add_to_waitlist

pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();

    hosting::add_to_waitlist();

    // Order a breakfast in the summer with Rye toast
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // Change our mind about what bread we'd like
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast, please", meal.toast);

    // The next line won't compile if we uncomment it; we're not allowed
    // to see or modify the seasonal fruit that comes with the meal
    // meal.seasonal_fruit = String::from("blueberries");

    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;

    println!("Order 1: {:?}", order1);
    println!("Order 2: {:?}", order2);
}
