fn main() {
    let mut s = String::from("hello world");

    let word = first_word(&s); // word is set to 5

    s.clear(); // s goes back to an empty string

    // At this point, the value of word has no meaning as the associated
    // string was cleared

    // We can use string slices to handle this problem
    let hello = &s[0..5];
    let world = &s[6..11];

    // In general, .. is the Rust syntax for a range
    // The variables above are refernces to a slice of s

    // Using this, we can redefine our function;
    let word = the_cooler_first_word(&s);

    // With this, we gain some more insight into string literals
    let hw = "Hello, world!";
    // hw is an immutable reference to a string of fixed length

    let my_string = String::from("hello world");
    let my_string_literal = "hello world";

    // Out improved type hinting means we can call the same function on
    // slices of string...
    let word = the_cooler_first_word(&my_string[..]);

    // And references to Strings...
    let word = the_cooler_first_word(&my_string);

    // And slices of string literals
    let word = the_cooler_first_word(&my_string_literal[..]);

    // And string literals directly, because they're already references
    let word = the_cooler_first_word(my_string_literal)
}

// Note: usize is essentially an integer which is exactly large enough to
// store the size of an object in memory in bytes
fn first_word(s: &String) -> usize {
    // Objective: Return the length of the first word in the provided string

    // First, convert to an array of bytes
    let bytes = s.as_bytes();

    // Iterate over each byte, using enumerate to easily track
    // the index as we go
    for (i, &item) in bytes.iter().enumerate() { // Tuple unpacking
        if item == b' ' { // byte literal syntax
            return i; // Return the position of the first space
        }
    }

    s.len() // If no spaces found, return the string length
}

// Note the different type hint, which allows us to pass both &String and
// &str
fn the_cooler_first_word(s: &str) -> &str {
    // If we use this function, we will be unable to clear s afterwards
    // as the compiler knows there are some existing references to the
    // underlying data. Using clear requires a mutable reference, which
    // cannot be created while the slices (immutable references) are still
    // in scope
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}