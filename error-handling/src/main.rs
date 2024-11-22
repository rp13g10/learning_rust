use std::fs::File;
use std::io::{self, ErrorKind, Read};

fn main() {
    // There are many reasons why we might want a programme to keep running
    // even if an error is encountered. For example, when trying to read a
    // file which does not exist. We might just want to let the user type
    // in a new file name.
    let greeting_file_result = File::open("hello.txt");

    // Note that open returns a Result object. The result is of type std::fs::File,
    // and the error is of type std::io::Error. Our code needs to be able to handle
    // both options. We can consider the Result object to be a type of Enum.

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {error:?}"),
    };

    // We might want to build on this behaviour, setting the script to create
    // files which don't exist, but panic if other problems like access issues
    // prevent the file from opening

    let greeting_file_result = File::open("hello.txt");

    let safer_greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {e:?}"),
            },
            other_error => {
                panic!("Problem opening the file: {other_error:?}");
            }
        },
    };

    // We can shorten this a little by using closures, which will be covered in
    // more detail later on in the book. The code below achieves the same effect
    // with closures rather than match statements

    let greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {error:?}");
            })
        } else {
            panic!("Problem opening the file: {error:?}")
        }
    });

    // There are a few other methods available to us when dealing with Results
    // unwrap can be used to either return the Ok value or panic
    let greeting_file = File::open("hello.txt").unwrap();

    // expect also gives us control over the error message
    let greeting_file =
        File::open("hello.txt").expect("hello.txt should be included in this project");

    // In general, expect should be the primary choice for this use case. The message
    // should include information about why this operation is always expected
    // to succeed.

    // Sometimes, we may wish to propagate an error rather than acting on it
    // immediately. We can do this by creating a function which returns a
    // Result object.

    fn read_username_from_file() -> Result<String, io::Error> {
        let username_file_result = File::open("hello.txt");

        let mut username_file = match username_file_result {
            Ok(file) => file,        // If the file opens, store the output as a variable
            Err(e) => return Err(e), // If an error is raised when opening the file, return it
        };

        let mut username = String::new();

        match username_file.read_to_string(&mut username) {
            Ok(_) => Ok(username),
            Err(e) => Err(e),
        }
    }

    // Rust provides a shorthand for this behaviour in the form of the ? operator
    fn read_username_from_file_but_shorter() -> Result<String, io::Error> {
        let mut username_file = File::open("hello.txt")?; // Store value or return error
        let mut username = String::new();
        username_file.read_to_string(&mut username)?; // Store value or return error
        Ok(username) // Encapsulate value in Ok type and return it
    }

    // Note that the ? also coerces any errors encountered to the type declared in
    // the function definition. As such, it can only be used in functions which return
    // a Result object (or an Option), otherwise there's no type to infer from.
    // Using ? on an Option will return None if one is encountered

    // We can combine the ? with method chaining to condense our function even further
    fn read_username_from_file_but_even_shorter_still() -> Result<String, io::Error> {
        let mut username = String::new();

        File::open("hello.txt")?.read_to_string(&mut username)?;

        Ok(username)
    }

    // In general, while our code needs to be able to handle errors, we should aim to
    // avoid the need to implement error handling at every point a value is used. We
    // can achieve this to an extent by defining custom datatypes which implement
    // checks for us, and provide confidence that output values will be usable.
    // For example, in our guessing game we might have added validation of the user
    // input with a custom Guess type

    pub struct Guess {
        value: i32,
    }

    impl Guess {
        pub fn new(value: i32) -> Guess {
            if value < 1 || value > 100 {
                panic!("Guess value must be between 1 and 100, got {value}.");
            }

            Guess { value }
        }

        pub fn value(&self) -> i32 {
            self.value
        }
    }

    // If we were to use the above datatype, any other logic based on the user input
    // could proceed with the assumption that the value would be between 0 and 100
    // without needing to perform the check itself.
}
