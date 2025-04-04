
mod back_of_house {
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
}

pub fn eat_at_restaurant() {
    // Order a breakfast in the summer with Rye toast
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // Change our mind about what bread we'd like
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // The next line won't compile if we uncomment it; we're not allowed
    // to see or modify the seasonal fruit that comes with the meal
    meal.seasonal_fruit = String::from("blueberries");
}


mod customer {
    use crate::front_of_house::hosting;
    pub fn eat_at_restaurant() {
        hosting::add_to_waitlist();
    }
}

//HOW TO DO WITHOUT THE PUBLIC FUNCTION (BOTH FIELDS OF STRUCT ARE PUBLIC)
/*
mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        pub seasonal_fruit: String, // Now public
    }
}

fn eat_at_restaurant() {
    // Directly creating an instance of Breakfast
    let meal = back_of_house::Breakfast {
        toast: String::from("Whole Wheat"),
        seasonal_fruit: String::from("Banana"), // No constructor needed
    };
    println!("Enjoy your breakfast with {} toast and {} fruit!", meal.toast, meal.seasonal_fruit);
}
*/
// EASIER WITH ENUM USAGE
/*
mod back_of_house {
    pub enum Appetizer {
        Soup,
        Salad,
    }
}

pub fn eat_at_restaurant() {
    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}
*/

mod front_of_house;

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}
