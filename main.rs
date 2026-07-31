use std::io::Read;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let mut lines = input.lines();

    // let name = lines.next().unwrap().trim();
    // let age = lines.next().unwrap().trim();
    // let num = lines.next().unwrap().parse::<i32>().unwrap();
    let line = lines.next().unwrap().trim();

    let nums: Vec<_> = line
        .split_whitespace()
        .map(|num| num.parse::<i32>().unwrap())
        .collect();

    let mut max = i32::MIN;
    for num in nums {
        if num > max {
            max = num;
        }
    }

    println!("{}", max);
}
