fn main() {
    let mut a: Vec<i32> = vec![1, 2, 3];
    a.push(4);
    a.push(5);
    a.pop();

    for b in &a {
        println!("value is {}", b);
    }

    println!("{:?}", a);
}
