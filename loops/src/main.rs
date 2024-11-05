fn main() {
    simple_loop();
    nested_loop();
    while_loop();
    loop_using_index();
    loop_over_elements();
    reverse_loop();
}

fn simple_loop() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            // This syntax allows us to set the value which we want to be
            // returned by the loop when it exits
            break counter * 2;
        }
    };

    println!("The result is {result}");
}

fn nested_loop() {
    let mut count = 0;
    // Single quote syntax allows us to define an alias for a loop
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                // Inner loop is broken here
                break;
            }
            if count == 2{
                // Outer loop is broken here on remaining == 10, count == 2
                break 'counting_up;
            }
            remaining -= 1;
        }

        count+= 1;
    }
    println!("End count = {count}");
}

fn while_loop() {
    let mut number = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("LIFTOFF!!!");
}

fn loop_using_index() {
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    // This works, but is not optimal as the code must check at runtime that
    // the index is inside the bounds of the arra
    while index < 5 {
        println!("The array value is: {}", a[index]);

        index += 1;
    }
}

fn loop_over_elements() {
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("The array value is {element}")
    }
}

fn reverse_loop() {
    // Shorthand for defining a range, as with python the upper limit is not
    // included (i.e. this starts at 3, not 4)
    for number in (1..4).rev() {
        println!("{number}");
    }
    println!("LIFTOFF!!!")
}