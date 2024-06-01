#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    // Wait for user input
    let stdin = io::stdin();
    
    loop {
        let mut input = String::new();
        print!("$ ");
        io::stdout().flush().unwrap();
        stdin.read_line(&mut input).unwrap();
        let args: Vec<_> = input.split(' ').collect();
        match args[..] {
            ["exit", code] => std::process::exit(code.parse::<i32>().unwrap()),
            ["echo", ..] => print!("{}", args[1..].join(" ")),
            _ => print!("{}: command not found\n", input.trim()),
        }
    }
}
