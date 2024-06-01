#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    print!("$ ");
    io::stdout().flush().unwrap();

    // Wait for user input
    let stdin = io::stdin();
    let mut input = String::new();
    loop {
        match stdin.read_line(&mut input) {
        Ok(n) => {
            let output = &input[..n-1];
            print!("{output}: command not found\n");
        }
        Err(error) => println!("error: {error}"),
    }
    }
}
