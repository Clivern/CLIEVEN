fn main() {
    let msg = "Hello, World!";
    println!("{}", msg);
    println!("{}", reverse_string(msg));
    println!("{}", hello());
}

fn reverse_string(input: &str) -> String {
    String::from(input).chars().rev().collect()
}

fn hello() -> &'static str {
    "Hello, World!"
}

#[cfg(test)]
mod test {

    use crate::hello;
    use crate::reverse_string;

    #[test]
    fn test_hello() {
        assert_eq!(hello(), "Hello, World!");
    }

    #[test]
    fn test_reverse_string() {
        assert_eq!(reverse_string("Hello"), String::from("olleH"));
    }
}
