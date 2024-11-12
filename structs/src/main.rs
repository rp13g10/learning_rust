// Structs serve to group related pieces of data together, they
// can be defined using the struct keyword
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64
}

// Note that you can define a struct to store a reference to a variable,
// but to do so you must implement a feature known as lifetimes. These are
// covered later on in the book

// Creating a struct without names generates a tuple struct
struct Colour(i32, i32, i32);
struct Point(i32, i32, i32);

// You can also create a struct with no data, this might be useful
// if you want to implement custom traits. These are known as
// unit structs
struct AlwaysEqual;

// Structs can be returned by functions
fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username: username, // We could just pass `username` here as both param and value are the same
        email: email,
        sign_in_count: 1,
    }
}

fn main() {
    // Instances of structs can then be defined by providing values
    // for each datapoint
    let mut user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    // Specific values can be retrieved or changed using dot notation
    user1.email = String::from("anotheremail@example.com");

    // Note that we have to specify that the instance is mutable for the 
    // above to work

    // We could create one user from another by explicitly passing in the
    // relevant values
    // let user2 = User {
    //     active: user1.active,
    //     username: user1.username,
    //     email: String::from("another@example.com"),
    //     sign_in_count: user1.sign_in_count
    // };

    // But Rust provides a shorthand syntax which saves some effort
    let user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };

    // NOTE: As username is a String, the concept of ownership applies. When
    // user2 is created, we are moving the value of username from user1 to user2,
    // and so cannot use user1 any more. If we were to create a new username, we
    // could safely reuser active and sign_in_count as these are held in the
    // stack

    // Tuple structs are instantiated as you would expect
    let black = Colour(0, 0, 0);
    let origin = Point(0, 0, 0);

    // Unit structs are instantiated without brackets
    let subject = AlwaysEqual;
}
