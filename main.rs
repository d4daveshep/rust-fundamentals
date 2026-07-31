use std::io::Read;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    let mut lines = input.lines();
    let a = lines.next().unwrap().parse::<i32>().unwrap();
    let b = lines.next().unwrap().parse::<i32>().unwrap();

    println!("{}", a + b);
}
