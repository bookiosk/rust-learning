use std::env;
use rt_common::greet;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() <= 1 {
        eprintln!("Usage: cargo run -p cli_tool -- <name>");
        std::process::exit(2);
    }
    let name = &args[1];
    println!("{}", greet(name));
}
