// mod front_of_houst {
//     pub mod hosting {
//         pub fn add_to_waitlist() {}
//         fn seat_at_table() {}
//     }

//     mod serving {
//         fn take_order() {}
//         fn serve_order() {}
//         fn take_payment() {}
//     }
// }

// pub fn eat_at_restaurant() {
//     crate::front_of_houst::hosting::add_to_waitlist();

//     front_of_houst::hosting::add_to_waitlist();
// }

// ------------------------------------------------------------
// fn serve_order() {}

// mod back_of_house {
//     fn fix_incorrect_order() {
//         cook_order();
//         super::serve_order()
//     }

//     fn cook_order() {}

//     pub struct Breakfast {
//         pub toast: String,
//         pub seasonal_frust: String,
//     }

//     impl Breakfast {
//         pub fn summer(toast: &str) -> Breakfast {
//             Breakfast {
//                 toast: String::from(toast),
//                 seasonal_frust: String::from("peaches"),
//             }
//         }
//     }
// }

// pub fn eat_at_restaurant() {
//     let mut meal = back_of_house::Breakfast::summer("Rye");
//     meal.toast = String::from("Wheat");
//     println!("I'd like {} toast please", meal.toast);
//     meal.seasonal_frust = String::from("blueberries");
// }

mod front_of_house;


// module import
// use crate::front_of_house::hosting::add_to_waitlist;
// use crate::front_of_house::hosting;
use front_of_house::hosting;

pub fn eat_at_restaurant() {
    // add_to_waitlist();
    hosting::add_to_waitlist();
}