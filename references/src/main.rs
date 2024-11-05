fn main() {
    // A function which returns its input is one way around the
    // issue of ownership in Rust. However, a more elegant solution
    // is to use a reference.
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("The length of '{s1}' is {len}.");

    // The ampersand syntax (in both function definition and function call)
    // denotes a reference. These allow functions to refer to values without
    // taking ownership of them.
}


fn calculate_length(s: &String) -> usize { // s is a reference to a String
    s.len()
} // s goes out of scope here, but is not dropped as it's only a reference

// Referencing values in this way is known as borrowing. There are some
// limitations when working with borrowed values. As we don't have
// ownership of them, we are unable to make any changes to their values.

// fn change(some_string: &String) {
//     // This won't work, as some_string is a reference
//     some_string.push_str(", world");
// }

// We can work around this to an extent with mutable references, but only
// one such reference can exist at any given time

fn change(some_string: &mut String) { // &mut creates a mutable reference
    some_string.push_str(", world");
}

// Limiting mutable references in this way prevents data races, which can
// occur when one of >=2 pointers is attempting to write data, while there
// is no mechanism in place to synchronise data between references. These
// errors can be extremely difficult to debug.

// Note that the restriction also applies to non-mutable references. If
// a mutable reference exists, it must be the ONLY reference IN THE CURRENT
// SCOPE