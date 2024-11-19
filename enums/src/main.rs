// Enums can be defined as a datatype in Rust
enum IpAddrKind {
    // Each variant can have associated data
    V4(u8, u8, u8, u8),
    // The type of data does not have to match between variants
    V6(String)
}

// There are no restrictions on the type of data which can be stored
// in an enum. The Rust standard library handles IP addresses in a similar
// manner, but uses Structs to represent IPV4/6 addresses

// Similarly to structs, we can use the impl keyword to define methods
// which are common across all variants of an enum
impl IpAddrKind {
    fn call(&self) {
        // Method body goes here!
    }
}

fn route(ip_kind: &IpAddrKind) {}

fn main() {
    // We can then instantiate enums using :: notation
    let home = IpAddrKind::V4(127, 0,  0,  1);
    let loopback = IpAddrKind::V6(String::from("::1"));

    route(&home);
    route(&loopback);

    home.call();
}

// One very helpful example of an enum is Option, which is how Rust
// deals with the concept of missing values. There is no NULL equivalent
// in Rust

// Note: <T> is a generic type parameter, to be covered later on
// enum Option<T> {
//     None,
//     Some(T),
// }

// We can define Some and None in the usual way

// let some_number = Some(5);
// let some_char = Some('e');

// let absent_number: Option<i32> = None;

// By making Option an explicitly different type, Rust is able to prevent
// runtime errors which could arise from missing data. This type of error
// can be caught at compilation time as i8 != Option<i8>