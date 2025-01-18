#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    // Uncomment this block to pass the first stage
    print!("$ ");
    io::stdout().flush().unwrap();

    // Wait for user input
    let stdin: io::Stdin = io::stdin();
    let mut input = String::new();
    stdin.read_line(&mut input).unwrap();
    println!("{}: command not found", input.trim());

    // match input {
    //     _ => println!("{}: command not found", input.trim()),
    //     _err => println!("errrr!!!!"),
    // };

    let out = io::stdout().write_all(b"Hello world");
}
