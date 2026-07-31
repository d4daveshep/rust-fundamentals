use std::{collections::HashSet, io::Read};

struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn distance_sq(&self, other: &Self) -> i32 {
        (self.x - other.x).pow(2) + (self.y - other.y).pow(2)
    }
}

fn main() {
    let lines: Vec<_> = (0..4)
        .map(|_| {
            let mut s = String::new();
            std::io::stdin().read_line(&mut s).unwrap();
            s.trim().parse::<i32>().unwrap()
        })
        .collect();
    let a = Point {
        x: lines[0],
        y: lines[1],
    };
    let b = Point {
        x: lines[2],
        y: lines[3],
    };
    println!("{}", a.distance_sq(&b));
}
