// We can split the code defined in the restaurant crate across
// multiple files to make it a bit easier to work with

// This tells the compiler to look for a front_of_house module in the
// filesystem
mod front_of_house;

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}