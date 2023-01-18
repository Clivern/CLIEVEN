// Importing standard library modules
use std::collections::HashMap;

// Defining a struct
struct Person {
    name: String,
    age: u32,
}

// Implementing methods for the struct
impl Person {
    fn new(name: String, age: u32) -> Person {
        Person { name, age }
    }

    fn introduce(&self) {
        println!("Hi, I'm {} and I'm {} years old.", self.name, self.age);
    }
}

// Defining an enum
enum Color {
    Red,
    Green,
    Blue,
}

// Main function
fn main() {
    // Printing to console
    println!("Hello, world!");
    println!("Hello, {}!", "world");
    print!("Hello, world!\n");

    let name = "John";
    let game = "football";

    println!("{name} like to play {game}", name = name, game = game);
    println!("{} like to play {}", name, game);
    println!("Binary: {:b} - Hex: {:x} - Octal {:o}", 10, 10, 10);

    // Variables and mutability
    let immutable_var1 = 5;
    let mut mutable_var1 = 10;
    mutable_var1 += 5;

    let mut mutable_float1 = 10.;
    mutable_float1 += 3.;

    println!("mutable_float1 = {}", mutable_float1);

    // Basic types
    let integer: i32 = 42;
    let float: f64 = 3.14;
    let boolean: bool = true;
    let character: char = 'A';

    // Strings
    let string_literal = "Hello";
    let string = String::from("World");

    // Arrays and Vectors
    let array = [1, 2, 3, 4, 5];
    let vector: Vec<i32> = vec![1, 2, 3, 4, 5];

    // Control flow
    if immutable_var1 < 10 {
        println!("Less than 10");
    } else {
        println!("10 or greater");
    }

    // Loops
    for i in 0..5 {
        println!("Loop iteration: {}", i);
    }

    let mut counter = 0;
    while counter < 3 {
        println!("While loop: {}", counter);
        counter += 1;
    }

    // Pattern matching
    let color = Color::Red;
    match color {
        Color::Red => println!("It's red!"),
        Color::Green => println!("It's green!"),
        Color::Blue => println!("It's blue!"),
    }

    // Functions
    fn add(a: i32, b: i32) -> i32 {
        a + b
    }
    let sum = add(5, 7);
    println!("Sum: {}", sum);

    // Closures
    let multiply = |x: i32, y: i32| x * y;
    println!("Product: {}", multiply(3, 4));

    // Using the struct
    let person = Person::new(String::from("Alice"), 30);
    person.introduce();

    // Error handling
    let result: Result<i32, &str> = Ok(42);
    match result {
        Ok(value) => println!("Success: {}", value),
        Err(error) => println!("Error: {}", error),
    }

    // Option type
    let maybe_value: Option<i32> = Some(42);

    if let Some(value) = maybe_value {
        println!("Got a value: {}", value);
    }

    // HashMap
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Red"), 50);

    // Ownership and borrowing
    let owned_string = String::from("Hello");
    let borrowed_string = &owned_string;
    println!("Borrowed: {}", borrowed_string);

    // Slices
    let slice = &array[1..4];
    println!("Slice: {:?}", slice);
}
