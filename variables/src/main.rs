fn main() {
    // Mutable variables
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6; // This errors out if x is not declared as mutable
    println!("The value of x is: {x}");

    // Shadowing
    let y = 5;
    let y = y + 1; // We're defining a new variable here, so mut is not required

    {
        // Inner scope is created with braces
        let y = y * 2;
        println!("The value of y in the inner scope is: {y}");
    }

    println!("The value of y in the outer scope is: {y}");

    // Floating points
    let a = 2.0; // Defaults to f64
    let b: f32 = 3.0; // Explicitly set to f32

    // Numeric operations
    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // i32/i32 => i32, be careful!

    // remainder
    let remainder = 43 % 5;

    // Booleans
    let t = true; // implicit
    let f: bool = false; // explicit

    // Characters
    // IMPORTANT: single quotes => char, double quotes => string
    // char is 4 bytes, supports any unicode character
    let c = 'z'; // implicit
    let d: char = 'z'; // explicit
    let heart_eyed_cat = '😻'; // Emoji support

    // Compound Types
    // tuple
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (t1, t2, t3) = tup; // destructuring/unpacking
    println!("The value of t2 is: {t2}");

    // element access with dot notation
    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let one = tup.2;

    // Arrays
    // IMPORTANT: Arrays are structures of fixed type and size
    let arr: [i32; 5] = [1, 2, 3, 4, 5];

    // Arrays support indexing as they are of known size
    let first = arr[0];
    let second = arr[1];

    // Attempting to access an invalid array will result in a runtime
    // error. Note that in some other low-level languages, this check
    // is not performed and invalid memory may instead be accessed

    // Arrays can be created as N occurences of a single value
    let array_of_threes = [3; 5];
}
