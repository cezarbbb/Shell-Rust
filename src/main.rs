#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    // Wait for user input
    let stdin = io::stdin();
    
    loop {
        let mut input = String::new();
        print!("$ ");
        io::stdout().flush().unwrap();
        let _ = stdin.read_line(&mut input);
        match input.trim().to_lowercase().as_str() {
            "exit 0" => break,
            _ => print!("{}: command not found\n", input.trim()),
        }
    }
}
