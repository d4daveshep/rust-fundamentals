use std::{collections::HashSet, io::Read};

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let mut lines = input.lines();

    // let name = lines.next().unwrap().trim();
    // let age = lines.next().unwrap().trim();
    // let num = lines.next().unwrap().parse::<i32>().unwrap();
    let line = lines.next().unwrap().trim();

    let sum = line
        .split_whitespace()
        .map(|num| num.parse::<i32>().unwrap())
        .filter(|num| num % 2 == 0)
        .map(|num| num * num)
        .sum::<i32>();

    println!("{}", sum);
}
