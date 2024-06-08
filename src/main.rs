#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    // Wait for user input
    let stdin = io::stdin();
    let path_env = match std::env::var("PATH") {
        Ok(path) => path,
        Err(_) => "/bin".to_owned(),
    };
    
    loop {
        let mut input = String::new();
        print!("$ ");
        io::stdout().flush().unwrap();
        stdin.read_line(&mut input).unwrap();
        let args: Vec<_> = input.split_whitespace().collect();
        match args[..] {
            ["exit", _code] => break,
            ["echo", ..] => print!("{}", args[1..].join(" ")),
            ["type", arg, ..] => {
                // match arg.trim() {
                //     "exit" | "echo" | "type" => print!("{} is a shell builtin\n", arg.trim()),
                //     _ => print!("{} not found\n", arg.trim()),
                // }
                let mut path_split = path_env.split(':');
                if let Some(path) = path_split.find(|path| std::fs::metadata(format!("{}/{}", path, arg)).is_ok()) {
                    print!("{arg} is {path}/{arg}");
                } else {
                    print!("missing_cmd: {arg} not found");
                }
            }
            _ => print!("{input}: command not found\n"),
        }
    }
}
