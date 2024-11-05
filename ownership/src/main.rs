fn main() {

    // The String datatype is distinct from string, as it is stored
    // in heap memory rather than on the stack. This means it can have
    // variable size, but makes it slower to access as it must be retrieved
    // with a pointer. Stack memory can be accessed directly.
    let _s = String::from("hello");

    // This type of string can be mutated
    let mut s = String::from("hello");

    s.push_str(", world!"); // push_str appends a literal to a String

    println!("{s}");

    // As its size is unknown, heap memory must be requested at runtime
    // In Rust, the allocated memory is returned as soon as the variable
    // goes out of scope.

    // This can have some unexpected side effects. Consider this example
    let x = 5;
    let y = x;

    // This creates two variables which can be pushed to the stack, both
    // have a value of 5. Now to try the same with a heap variable.

    let s1 = String::from("hello");
    let s2 = s1;
    println!("{s1}, world!"); // This will not run/compile

    // In the stack, s1 is represented by a pointer to the heap, and details
    // of its length & capacity
    // When we take a copy, we only copy the data held in the stack. This
    // means we just copy a pointer to the same data on the heap.

    // To ensure memory safety, the variable s1 is invalidated when we define
    // s2. This ensures only one variable ever exists which points to a single
    // piece of data on the heap.

    // If we really need to duplicate the heap memory, we can clone it explicitly
    let s3 = String::from("hello");
    let s4 = s3.clone();
    println!("s3 = {s3}, s4 = {s4}"); // This works fine

    // Behind the scenes, types held only on the stack implement the Copy
    // trait, which allows them to be copied without a move (invalidating the
    // original variable)

    // This holds true when passing Strings into functions, the original
    // variable will be invalidated and cease to be valid after the function
    // has been called. If you need to use it again, the function must
    // return it to transfer ownership back to the outer scope.
}
