use std::env;
use rt_common::greet;

fn main() {
    let mut args = env::args().skip(1);
    let name = args.next().unwrap_or_else(|| "stranger".to_string());
    println!("{}", greet(&name));
}
