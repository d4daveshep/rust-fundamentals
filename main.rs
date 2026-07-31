use std::io::Read;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let mut lines = input.lines();

    // let name = lines.next().unwrap().trim();
    // let age = lines.next().unwrap().trim();
    // let num = lines.next().unwrap().parse::<i32>().unwrap();
    let line = lines.next().unwrap().trim();

    let length = count(line);

    println!("{}", length);
}

fn count(input: &str) -> usize {
    input.len()
}
