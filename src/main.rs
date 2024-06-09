#[allow(unused_imports)]
use std::io::{self, Write};
use std::process::Command;
use std::env;

fn main() {
    // Wait for user input
    let stdin = io::stdin();
    let path_env = match env::var("PATH") {
        Ok(path) => path,
        Err(_) => "/bin".to_owned(),
    };
    let home_env = match env::var("HOME") {
        Ok(path) => path,
        Err(_) => "/home/user".to_owned(),
    };
    
    loop {
        let mut input = String::new();
        print!("$ ");
        io::stdout().flush().unwrap();
        stdin.read_line(&mut input).unwrap();
        let args: Vec<_> = input.split_whitespace().collect();
        match args[..] {
            ["exit", _code] => break,
            ["echo", ..] => println!("{}", args[1..].join(" ")),
            ["type", arg, ..] => {
                match arg {
                    "exit" | "echo" | "type" => print!("{} is a shell builtin\n", arg.trim()),
                    _ => {
                        let mut path_split = path_env.split(':');
                        if let Some(path) = path_split.find(|path| std::fs::metadata(format!("{}/{}", path, arg)).is_ok()) {
                            println!("{arg} is {path}/{arg}");
                        } else {
                            println!("{arg} not found");
                        }
                    }
                }
            },
            ["pwd"] => {
                let dir = env::current_dir().unwrap();
                println!("{}", dir.display());
            },
            ["cd", path] => {
                if path == "~" {
                    if env::set_current_dir(&home_env).is_err() {
                        println!("cd: {}: No such file or directory", path);
                    }
                }
                if env::set_current_dir(path).is_err() {
                    println!("cd: {}: No such file or directory", path);
                }
            },
            _ => {
                for path in env::split_paths(&path_env) {
                    let exec_path = path.join(args[0]);
                    if exec_path.is_file() {
                        Command::new(exec_path).args(&args[1..]).status().expect("failed to execute process");
                        break;
                    } else {
                        println!("{}: command not found", input.trim());
                        break;
                    }
                }
            },
        }
    }
}
