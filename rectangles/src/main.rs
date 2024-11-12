fn main() {
    let scale = 2;
    // The dbg! macro pretty-prints a value then returns it
    let rect1 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };

    dbg!(&rect1); // If we need to retain ownership of rect1, we must pass it as a reference
    
    // To print using debug formatting, note the use of the :? format spec
    println!("rect1 is {:?}", rect1);

    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1)
    );
}

// We can link function arguments together as a tuple to make it clear that
// width and height are not independent
// fn area(width: u32, height: u32) -> u32 {
//     width * height
// }

// fn area(dimensions: (u32, u32)) -> u32 {
//     dimensions.0 * dimensions.1
// }

// But this still isn't ideal, as index notation isn't the most readable here

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

// Using a struct improves readability and defines the linkage between height and width

// ABy default, the Rectangle struct cannot be printed as it does not implement
// the std::fmt::display trait. We can inherit that behaviour using the derive
// outer attribute. Debug implements the Display trait, so we can set our Struct to
// derive from Debug