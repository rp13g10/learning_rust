fn main() {
    // Consider the following example, which should only take an action if
    // config_max has a value, and does nothing if it is None
    let config_max = Some(3u8);
    match config_max {
        Some(max) => println!("The maximum is configured to be {max}"),
        _ => ()
    }

    // We can use if let to achieve the same effect with fewer lines of code
    if let Some(max) = config_max {
        println!("The maximum is configured to be {max}");
    }

    // It is important to note that this implementation loses the built-in
    // exhaustive checking of values provided by match. To achieve this, we
    // would need to include an else block
    let mut count = 0;
    // Coin enum was defined in the 'match_' module
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {state:?}!");
    } else {
        count += 1;
    }
}
