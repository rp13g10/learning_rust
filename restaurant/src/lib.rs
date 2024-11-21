// Note: Create libs using cargo new <name> --lib
// Libs are expected to be imported, rather than called directly via the
// command line

// lib.rs (and main.rs) are known as crate roots, and are where the Rust
// compiler will start.

// We can use modules to group related functions. Modules can be
// defined inline, or in the file system.
mod front_of_house {

    // A module can contain another module. Collectively, these modules
    // are known as siblings (defined at the same level). In relation
    // to front_of_house, they are considered to be children.
    pub mod hosting {
        pub fn add_to_waitlist() {}
        pub fn seat_at_table() {}
    }

    pub mod serving {
        fn take_order() {}
        fn serve_order() {}
        fn take_payment() {}
    }
}

// We can reference modules in a crate using relative or absolute paths
// In libs, use `pub fn` to define a user-facing function. If `pub` is not
// used, the function will only be visible inside the crate
pub fn eat_at_restaurant() {

    // Note that in order to access the contents of hosting, we must define
    // it as a public module. Similarly, to reference and use functions, they
    // must be public functions

    // Absolute_path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();
}

fn deliver_order() {}

mod back_of_house {
    // We can use the `super` keyword to access code defined outside
    // of the current module without using absolute references. This
    // can make it easier to move code around without needing to update
    // any references by hand

    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order();
    }

    fn cook_order() {}

    // Structs and enums can also be made public
    pub struct Breakfast {
        // Struct fields must also be made public for external use
        pub toast: String,
        seasonal_fruit: String
    }

    impl Breakfast {

        // Because Breakfast has a private field, we must provide a public
        // facing function which creates an instance of it if the struct is to
        // be used outside of the back_of_house module
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    // In enums, all of the variants are public if the enum is public
    pub enum Appetizer {
        Soup,
        Salad
    }
}

pub fn eat_at_restaurant_2() {
    // Order a breakfast in the summer with Rye toast
    let mut meal = back_of_house::Breakfast::summer("Rye");

    // Change our mind about the bread
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // The next line won't compile if we uncomment it; we're not allowed
    // to see or modify the seasonal fruit that comes with the meal
    // meal.seasonal_fruit = String::from("blueberries");

    let order_1 = back_of_house::Appetizer::Soup;
    let order_2 = back_of_house::Appetizer::Salad;
}

// To avoid writing out paths, we can make use of the `use` keyword to
// bring functions from one scope into another

// Note that syntactically we could use add_to_waitlist directly rather than
// stopping at the module it's defined in. However, it is considered best practice
// to only import the module, as this makes it clear that the function being
// called is not defined locally
use crate::front_of_house::hosting;

pub fn eat_at_restaurant_3() {
    hosting::add_to_waitlist();
}

// When importing other items like structs & enums, by convention it is
// expected that the item will be used directly rather than via its defining
// module
use std::collections::HashMap;

fn use_hashmap() {
    let mut map = HashMap::new();
    map.insert(1, 2);
}

// The exception here is if two different modules expose an object with
// the same name. At which point we must use the module as an anchor to
// prevent compiler errors

// The other way around this would be to provide an alias
use std::fmt::Result;
use std::io::Result as IoResult;

// We can also use `use` to alter the public facing API of a library
// This would allow external functions to use the hosting module directly
// rather than going through front_of_house
pub use crate::front_of_house::hosting as pub_hosting;

// If we need to use more than one part of an external crate, we could have
// a separate line for each reference
// use std::cmp::Ordering;
// use std::io;

// But it's more convenient to use the following shorthand
// use std::{cmp::Ordering, io};

// If we want to use a module and an object from it directly, there's
// a shorthand for that too!
// use std::io;
// use std::io::Write;
// use std::io::{self, Write};

// To bring in all public items defined in a path, the glob operator can
// be used. Although it is not generally recommended as it can make it difficult
// to see where each function came from. The main exception to this is when
// writing unit tests.
// use std::collections::*;
