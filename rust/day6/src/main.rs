fn main() {
    println!("{:?}", divide(10, 2));
    println!("{:?}", divide(10, 0));
    println!("{:?}", divide(10, -2));
}

#[derive(Debug)]
struct DivisionError {
    message: String,
}

impl std::error::Error for DivisionError {}

impl std::fmt::Display for DivisionError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

fn divide(a: i64, b: i64) -> Result<i64, DivisionError> {
    if b == 0 {
        Err(DivisionError {
            message: String::from("Error: Division by zero"),
        })
    } else {
        Ok(a / b)
    }
}
