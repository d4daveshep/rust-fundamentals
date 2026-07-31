use std::io::Read;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let mut lines = input.lines();

    let name = lines.next().unwrap().trim();
    let age = lines.next().unwrap().trim();

    println!("Hi, {}! You are {} years old.", name, age);
}
