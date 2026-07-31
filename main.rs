use std::io::Read;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    let mut lines = input.lines();
    let width = lines.next().unwrap().parse::<i32>().unwrap();
    let height = lines.next().unwrap().parse::<i32>().unwrap();

    println!("{}", width * height);
}
