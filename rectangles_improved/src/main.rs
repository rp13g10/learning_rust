// Previously, we defined Rectangle as a simple struct
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// However, it makes more sense to take the area function from
// the `rectangles` exercise and make it a method of the Rectangle
// struct, rather than a separate function
impl Rectangle {
    // Everything in this implementation block will be associated with the
    // Rectangle type

    // As in Python, the first argument of a method should be a reference to self
    // &self is shorthand for `self: &Self`, where the Self type is made available
    // inside implementation blocks
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // You could pass self without the & if you wanted, but as this would take
    // ownership of the object it's not a common use case.

    // Note that a method can have the same name as a param, Rust will fetch the
    // correct item based on the presence/absence of parentheses
    fn width(&self) -> bool {
        self.width > 0
    }

    // Methods can take more than one argument
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    // We can also create constructors which return a new instance of the Struct,
    // these are accessed using Rectangle:: rather than `instance.` notation.
    // Presumably this behaviour is triggered for methods which return the Self
    // type?
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size
        }
    }
}

// Note that a struct can have more than one implementation block, some use cases
// for this will be covered later in the book

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );

    // Demonstrate references to methods & params based on parentheses
    if rect1.width() {
        println!("The rectangle has a nonzero width; it is {}", rect1.width);
    }

    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    let sq = Rectangle::square(3);
}