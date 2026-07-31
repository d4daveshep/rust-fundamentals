enum Light {
    Red,
    Yellow,
    Green,
}

impl Light {
    fn next(&self) -> Self {
        match self {
            Light::Red => Light::Green,
            Light::Yellow => Light::Red,
            Light::Green => Light::Yellow,
        }
    }

    fn name(&self) -> &str {
        match self {
            Light::Red => "red",
            Light::Yellow => "yellow",
            Light::Green => "green",
        }
    }
}

fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();

    let line = input.trim();

    let current = match line {
        "red" => Light::Red,
        "yellow" => Light::Yellow,
        "green" => Light::Green,
        _ => Light::Green,
    };

    let next = current.next();

    println!("{}", next.name());
}
