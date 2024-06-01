#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    print!("$ ");
    io::stdout().flush().unwrap();

    // Wait for user input
    let stdin = io::stdin();
    let mut input = String::new();
    match stdin.read_line(&mut input) {
        Ok(n) => {
            println!("{input}: command not found");
        }
        Err(error) => println!("error: {error}"),
    }

}
