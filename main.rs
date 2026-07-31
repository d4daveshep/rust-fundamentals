use std::io::Read;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let mut lines = input.lines();

    // let name = lines.next().unwrap().trim();
    // let age = lines.next().unwrap().trim();
    let num = lines.next().unwrap().parse::<u64>().unwrap();

    let range = 1..=num;
    // let sum = range.sum::<u64>();

    let mut sum = 0_u64;
    for n in range {
        sum += n;
    }
    println!("{}", sum);
}
