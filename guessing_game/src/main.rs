// `use` required to bring in functions beyond the default scope
use rand::Rng;
use std::cmp::Ordering;
use std::io;

// `fn` to define new function
fn main() {
    // ! after function name denotes a builtin rust macro
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is {secret_number}");

    loop {
        println!("Please input your guess.");

        // Define new variable, `guess` as an empty string
        // `mut` prefix denotes a mutable variable, which can be modified
        // after creation
        let mut guess = String::new();

        // Use the standard library to read user input and store the output as
        // `guess`. Note that as with variable declarations, references must also
        // be explicitly defined as mutable if they are to be changed.
        // `read_line` returns a `Result` object which will have either `Ok` or `Err`
        // state. `expect` defines how to deal with this result. `expect` will crash
        // out and display the provided message if an `Err` state is detected. Otherwise
        // it will return the output of the function which generated the result.
        io::stdin()
            .read_line(&mut guess) // `&` denotes a reference to an existing variable
            .expect("Failed to read line");

        // Convert the string input into an unsigned 32-bit int, erroring with a
        // message if parsing fails.
        // Redefining a variable with a new type is known as 'shadowing'
        let guess: u32 = guess.trim().parse().expect("Please type a number!");

        // Use curly bracket syntax to insert variables into strings
        println!("You guessed: {}", guess);

        // `guess.cmp` returns an `Ordering` enum of either the `Less`, `Greater` or `Equal` variant
        // `match` defines the behaviour to take when each of the variants is received
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
