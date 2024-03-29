#[derive(Debug)]
pub enum Appetizer {
    Soup,
    Salad,
}

// We can make the struct public, but the struct’s fields will still be private.
// We can make each field public or not on a case-by-case basis.
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

fn fix_incorrect_order() {
    cook_order();
    super::serve_order();
}

fn cook_order() {}
