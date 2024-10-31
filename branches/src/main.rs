fn main() {
    let number: i32 = 3;

    if number < 5{
        println!("condition was true");
    } else {
        println!{"condition was false"};
    };

    // Note: Unlike other languages, if statements will only accept
    // boolean inputs. Something like number = 3; if number... will not
    // work. We must instead do this:

    if number != 0 {
        println!("number was not equal to zero")
    };

    // We can test for multiple conditions using else if

    let number2: i32 = 6;

    if number2 % 4 == 0 {
        println!("number2 is divisible by 4");
    } else if number2 % 3 == 0 {
        println!("number2 is divisble by 3");
    } else if number2 % 2 == 0 {
        println!("number2 is divisible by 2");
    } else {
        println!("number2 is not divisible by 4, 3, or 2");
    };

    // As if is an expression, we can assign it to a variable
    let condition: bool = true;
    let number3 = if condition {5} else {6};

    // When assigning the output of an if statement in this manner, it
    // is important to ensure that the output type of each branch is
    // the same

    println!("The value of number3 is: {number3}");
}

