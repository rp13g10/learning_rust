// Note: All of the collections covered here use the Heap memory
fn main() {
    
    // ########################################################################
    // # Vectors                                                              #
    // ########################################################################

    // Vectors contain multiple values which are stored next to each other in
    // memory. They can only contain one datatype.

    // To create an empty vector, use the Vec::new function
    let mut v1: Vec<i32> = Vec::new(); // Note that we must provide a type for empty vectors

    // To create a vector with initial data, the syntax is simpler
    let v2 = vec![1, 2, 3, 4, 5]; // Note that rust can infer the datatype as values are provided

    // To add items to a vector, we can use the push method. Recall that the
    // vector must be mutable for this to work.
    v1.push(5);
    v1.push(6);
    v1.push(7);
    v1.push(8);

    // There are a few ways to acces an item from a vector

    // Indexing syntax will error out if the value is missing
    let third: &i32 = &v1[2];
    println!("The third element is {third}");

    let third: Option<&i32> = v2.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element.")
    }

    // If a reference to an element in a vector is created, the vector will be
    // locked for editing. In order to add a value, Rust needs to borrow a
    // mutable reference to the vector, but creating the slice creates an
    // immutable reference which would be invalidated if the push were to be
    // allowed.
    // This code snippet will result in an error.
    let mut v3 = vec![1, 2, 3, 4, 5];
    let first = &v3[0];
    // v3.push(6); // This would throw an error

    println!("The first element is: {first}");

    // We can iterate over values using the usual syntax
    let v4 = vec![100, 32, 57];
    for i in &v4 {
        println!("{i}");
    }

    // If we need to alter values, the mut keyword lets us do this
    let mut v5 = vec![100, 32, 57];
    for i in &mut v5 {
        // the * symbol is known as the dereference operator, and is covered in later chapters
        *i += 50;
    }

    {
        // While a vector can only store one type, if we make that one type an enum
        // we gain a lot of flexibility
        enum SpreadsheetCell {
            Int(i32),
            Float(f64),
            Text(String),
        }

        let row = vec![
            SpreadsheetCell::Int(3),
            SpreadsheetCell::Text(String::from("blue")),
            SpreadsheetCell::Float(10.12)
        ];
    }

    // When a vector is dropped, so is its content. The cells above are cleared
    // from the heap once `row` goes out of scope

    // ########################################################################
    // # Strings                                                              #
    // ########################################################################

    // In the core language, only the `str` datatype exists. The `String` type
    // is provided by the standard library. Behind the scenes, a `String` is a
    // vector of bytes with some additional restrictions.

    // We can create a String in the same was as a vector
    let mut s1 = String::new();

    // Or convert a `str` into one, there are a few ways to do this
    let str_data = "initial contents";
    let s2 = str_data.to_string();

    let s3 = "initial contents".to_string();

    let s4 = String::from("initial contents");

    // Strings are encoded using utf8, so they work with any properly encoded
    // character

    let hello = String::from("السلام عليكم");
    let hello = String::from("Dobrý den");
    let hello = String::from("Hello");
    let hello = String::from("שלום");
    let hello = String::from("नमस्ते");
    let hello = String::from("こんにちは");
    let hello = String::from("안녕하세요");
    let hello = String::from("你好");
    let hello = String::from("Olá");
    let hello = String::from("Здравствуйте");
    let hello = String::from("Hola");

    // We can add data to a string in a similar way to a vector
    let mut s5 = String::from("foo");
    s5.push_str("bar");

    // Note that push_str accepts a string slice, so the appended value can
    // still be referenced further on in the code if needed
    let mut s6 = String::from("foo");
    let s7 = "bar";
    s6.push_str(s7);
    println!("s7 is {s7}");

    // We can also use `push` to append a single character to a String
    let mut s8 = String::from("lo");
    s8.push('l');

    // Strings can be concatenated with the + operator
    let s9 = String::from("Hello, ");
    let s10 = String::from("world!");
    let s11 = s9 + &s10; // Note that this moves s9, invalidating the reference. s10 can still be used however.

    // Behind the scenes, the + operator calls the `add` function, which defines this behaviour

    // If we have lots of strings to combine, the format! macro is generally more user friendly
    let s12 = String::from("tic");
    let s13 = String::from("tac");
    let s14 = String::from("toe");

    let s15 = format!("{s12}-{s13}-{s14}");

    // It is important to note that strings cannot be indexed like vectors
    let s16 = String::from("hello");
    // let h = s16[0]; // This would thrown an error

    // This is because some characters take up more than one byte in memory.
    // If the character boundaries are known, providing a slice is valid syntax
    // but should be done with caution.
    let hello = "Здравствуйте";
    let first_char = &hello[0..4];

    // If the slice doesn't match up with a character boundary, an error will
    // be thrown.

    // We can iterate over strings, but need to specify how we want to do it
    for c in "Зд".chars() {
        println!("{c}");
    } // prints 2 characters

    for b in "Зд".bytes() {
        println!("{b}");
    } // prints 4 bytes
    

    // ########################################################################
    // # Hash Maps                                                            #
    // ########################################################################

    // HashMaps are analagous to the dict type in Python, and are used to look
    // up values based on a key of any type, rather than an index

    use std::collections::HashMap;

    // Create a HashMap with the usual syntax
    let mut scores = HashMap::new();

    // Add some data
    scores.insert(String::from("Blue"), 10); // Blue team start with 10 points
    scores.insert(String::from("Yellow"), 50); // Yellow starts with 50

    // Retrieve it again
    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);

    // .get returns None if the key is not present in the hash map, so we use
    // unwrap_or to ensure we always retrieve a numeric value

    // We can also iterate over a hash map
    for (key, value) in &scores {
        println!("{key}: {value}");
    }

    // If a datatype implements the Copy trait, the value will be copied into the
    // hash map rather than moved. For types like String which do not implement this
    // trait, the value will be moved and the original reference invalidated after
    // the insert
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // At this point field_name and field_value can no longer be used

    // Note that subsequent `insert` calls with the same key will simply overwrite
    // any existing data, rather than throwing an exception. We can use `or_insert`
    // if we only want to add data when the key is missing.
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    println!("{scores:?}"); // Yellow gives 50, Blue gives 10

    // Using the dereference operator again, it's possible to update a value
    // in a hash map using the existing vaue
    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        // Retrieve the count for each word, initializing with value 0 for words
        // not yet encountered
        let count = map.entry(word).or_insert(0);
        // Add 1 to the count, this reflects in the hash map as or_insert
        // returns a mutable reference
        *count += 1;
    }

    println!("{map:?}");

    // Note that the default hashing algorithm for HashMap is SipHash, which
    // is secure but slower than some other approaches. Other hashers are
    // available if SipHash proves to be too slow

}
