fn main() {
    // Using unwrap()
    let number1 = 20;
    let result1 = validate_positive_number(number1).unwrap(); // This will panic if the number is not positive
    println!("The number {} is positive: {}", number1, result1);

    // Using expect()
    let number2 = 15;
    let result2 = validate_positive_number(number2).expect("Expected a positive number"); // Custom panic message
    println!("The number {} is positive: {}", number2, result2);

    // Using match
    let number3 = -5;
    match validate_positive_number(number3) {
        Ok(is_positive) => println!("The number {} is positive: {}", number3, is_positive),
        Err(e) => println!("Error: {}", e), // Handle the error gracefully
    }
}

fn validate_positive_number(num: i64) -> Result<bool, String> {
    if num > 0 {
        Ok(true)
    } else {
        Err(format!("Number {} is less than or equal to 0", num))
    }
}
