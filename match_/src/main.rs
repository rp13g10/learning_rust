#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    // -- etc
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    // We know from the last chapter that we can associate values with enum variants
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    // We can use match to return different values based on a pattern
    match coin {
        Coin::Penny => {
            // We can use curly braces to execute additional code
            println!("Lucky penny!");
            1 // This value is returned by the match (if this condition matches)
        },
        Coin::Nickel => 5, // For simple value returns, braces are not required
        Coin::Dime => 10,
        // This would trigger if we called value_in_cents on
        // Coin::Quarter(UsState::Alaska), for instance
        Coin::Quarter(state) => {
            println!("State quarter from {state:?}!");
            25
        },
    }
}

// We can also use match to handle Option objects
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None, // I believe this to be shorthand for Option::None baked into the language
        Some(i) => Some(i + 1),
        // Note that matches are exhaustive, the compiler knows that None and Some are
        // the only two variants of Option. If either were missing, a compiler error would
        // be raised
    }
}

// If the last entry in a match flow is a variable declaration rather than a condition,
// it will be called for all cases which did not match a previous arm. Declaring a variable
// as _ denotes that the value will not be used in the output
// let dice_roll = 9;
// match dice_roll {
//     3 => add_fancy_hat(),
//     7 => remove_fancy_hat(),
//     other => move_player(other),
// }

// fn add_fancy_hat() {}
// fn remove_fancy_hat() {}
// fn move_player(num_spaces: u8) {}

// Building on this, if we want to have anything which matches on the
// wildcard result in no action being taken, we can use () like so
// let dice_roll = 9;
// match dice_roll {
//     3 => add_fancy_hat(),
//     7 => remove_fancy_hat(),
//     _ => (),
// }

// fn add_fancy_hat() {}
// fn remove_fancy_hat() {}


fn main() {
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
}
