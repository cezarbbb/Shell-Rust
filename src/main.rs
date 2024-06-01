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
            ["exit", _code] => break,
            ["echo", ..] => print!("{}", args[1..].join(" ")),
            ["type", arg, ..] => {
                match arg.trim() {
                    "exit" | "echo" => print!("{} is a shell builtin\n", arg.trim()),
                    _ => print!("{} not found\n", arg.trim()),
                }
            }
            _ => print!("{}: command not found\n", input.trim()),
        }
    }
}
