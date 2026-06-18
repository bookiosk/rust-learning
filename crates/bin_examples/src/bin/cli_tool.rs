use std::env;

fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}

fn main() {
    let mut args = env::args().skip(1);
    let name = args.next().unwrap_or_else(|| "stranger".to_string());
    println!("{}", greet(&name));
}
