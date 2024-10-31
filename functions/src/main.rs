fn main() {
    println!("Hello, world!");

    another_function(5);

    // Recall that as ' denotes a char rather than a str, using " here
    // would result in an error
    print_labeled_measurement(5, 'h');

    // This function returns a value which we can assign
    let x = five();
    println!("The value of x is {x}");

    // This function does the same, but the output is now dynamic
    let y = plus_one(7);
    println!("The value of y is {y}");
}

// Note that a function can be defined below a call in the script
fn another_function(x: i32) {
    println!("The value of x is: {x}.");
}

// Multiple arguments can be defined using commas. Note that the
// provision of a type hint is not optional in rust
fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

// Note
// `fn` and `let` are examples of statements. A statement cannot be assigned
// to a variable. For example, the following would return an error:

// fn main() {
//     let x = (let y = 6);
// }

// The let statement does not return a value, so there's nothing to assign
// to x. This is in contrast to other languages where x = y = 6 may be a
// valid assignment.

// Expressions evaluate to a value, and can be assigned to variables. 5+6
// is an expression. Calling a function is an expression, and scope blocks
// are expressions. The following snippet for example prints 4:

// fn main() {
//     let y = {
//         let x = 3;
//         x + 1
//     };

//     println!("The value of y is: {y}");
// }

// Because there is no semicolon after x + 1, it is considered to be an
// expression. If we were to add a semicolon, the inner scope above would
// not return a value.

// In rust, a function will implicitly return the last value. We can also
// return values early with the `return` keyword.

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

// Neither of these functions would work if we added a semicolon. We have declared
// that they return i32, so a null return value causes an error.