use std::num::ParseIntError;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();

    let result = input.trim().parse::<i32>();
    match result {
        Ok(num) => println!("ok: {}", num),
        ParseIntError => println!("error: not a number"),
    }
}
