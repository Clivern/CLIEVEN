fn main() {
    let msg = "Hello, World!";
    println!("{}", msg);
    println!("{}", reverse_string(msg));
}

fn reverse_string(input: &str) -> String {
    String::from(input).chars().rev().collect()
}
