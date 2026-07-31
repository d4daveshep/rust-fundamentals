fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();

    let result = parse_int(input.trim());

    match result {
        Ok(num) => println!("ok: {}", num),
        Err(msg) => println!("error: {}", msg),
    }
}

fn parse_int(s: &str) -> Result<i32, String> {
    let result = s
        .trim()
        .parse::<i32>()
        .map_err(|_| String::from("not a number"));
    result
}
