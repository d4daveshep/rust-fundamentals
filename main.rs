use std::io::Read;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    let trimmed = input.trim();
    let upper = trimmed.to_uppercase();

    println!("{}", upper);
}
