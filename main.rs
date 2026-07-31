use std::io::Read;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let mut lines = input.lines();

    // let name = lines.next().unwrap().trim();
    // let age = lines.next().unwrap().trim();
    let num = lines.next().unwrap().parse::<i32>().unwrap();

    // if num % 15 == 0 {
    //     println!("FizzBuzz")
    // } else if num % 3 == 0 {
    //     println!("Fizz")
    // } else if num % 5 == 0 {
    //     println!("Buzz")
    // } else {
    //     println!("{num}")
    // }

    match (num % 3 == 0, num % 5 == 0) {
        (true, true) => println!("FizzBuzz"),
        (true, false) => println!("Fizz"),
        (false, true) => println!("Buzz"),
        (false, false) => println!("{num}"),
    }
}
