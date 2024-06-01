#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    // Wait for user input
    let stdin = io::stdin();
    
    loop {
        let mut input = String::new();
        print!("$ ");
        io::stdout().flush().unwrap();
        match stdin.read_line(&mut input) {
        Ok(n) => {
            let output = &input[..n-1];
            print!("{output}: command not found\n");
        }
        Err(error) => println!("error: {error}"),
    }
    }
}
