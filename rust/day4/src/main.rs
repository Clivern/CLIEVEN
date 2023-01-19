fn main() {
    let a: bool = true;

    // If conditional
    if a {
        println!("a var is true");
    } else {
        println!("a var is false");
    }

    let mut b = 1;

    // while loop
    while b < 100 {
        b += 1
    }

    println!("b is {}", b);

    // loops
    let c = loop {
        b += 1;

        if b > 200 {
            break b;
        }
    };

    println!("c var is {}", c);

    // match
    let d: Result<String, String> = Ok(String::from("Hello"));
    let e: Result<String, String> = Err(String::from("Something wrong"));

    match d {
        Ok(v) => println!("{}", v),
        Err(v) => println!("{}", v),
    };

    match e {
        Ok(v) => println!("{}", v),
        Err(v) => println!("{}", v),
    };

    // match with multiple patterns
    let number = 3;

    match number {
        1 => println!("One"),
        2 => println!("Two"),
        3 => println!("Three"),
        _ => println!("Not one, two, or three"),
    }

    // for loop
    for i in 1..5 {
        println!("i is {}", i);
    }
    // for loop inclusive
    for i in 1..=5 {
        println!("i is {}", i);
    }

    let f: Option<i32> = Some(20);

    if let Some(v) = f {
        println!("f is {}", v);
    } else {
        println!("No value found for f");
    }
}
