#[allow(unused_imports)]
use std::io::{self, Write};
use std::os::unix::fs::PermissionsExt;

fn main() {
    // TODO: Uncomment the code below to pass the first stage

    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let mut command = String::new();
        io::stdin().read_line(&mut command).unwrap();

        let command = command.trim();

        //this will break the command into arrya of tokens
        let tokens = command.split_whitespace().collect::<Vec<&str>>();

        match tokens[0] {
            "exit" => break,
            "echo" => println!("{}", tokens[1..].join(" ")),
            // "type" => println!("{} is a shell builtin", tokens[1..].join(" ")),
            "type" => {
                let arg = tokens[1];
                match arg {
                    "exit" | "echo" | "type" |"pwd"|"cd" => {
                        println!("{} is a shell builtin", arg);
                    }
                    _ => {
                        //displays a long string of path variables like "/home/pratyaksh/.bun/bin:/home/pratyaksh/.local/share/solana/install/active_release/bin:/home/pratyaksh/.local/bin:/home/pratyaksh/bin:/home/etcccccccc"
                        let path_var = std::env::var("PATH").unwrap();
                        let mut found = false;

                        for path_dir in path_var.split(':') {
                            let full_path = std::path::Path::new(path_dir).join(tokens[1]);
                            // println!("{:?}",full_path);
                            if full_path.exists() {
                                if let Ok(metadata) = std::fs::metadata(&full_path) {
                                    if metadata.permissions().mode() & 0o111 != 0 {
                                        println!("{} is {}", tokens[1], full_path.display());
                                        found = true;
                                        break;
                                    }
                                }
                            }
                        }

                        if !found {
                            println!("{}: not found", tokens[1]);
                        }
                    }
                }
            },
            "pwd"=> {
                let curr_dir = std::env::current_dir().unwrap();
                println!("{}",curr_dir.display());
            },
            "cd" => {

                if tokens.len() < 2 {
                    continue;
                }

                let path = tokens[1];

                if path.starts_with('/') {
                    let path_exists = std::path::Path::new(path).exists();
                    let is_path_dir = std::path::Path::new(path).is_dir();

                    if path_exists && is_path_dir {
                        std::env::set_current_dir(path);
                      
                    }else{
                        println!("cd: {}: No such file or directory", path);
                    }
                }else{
                        println!("cd: {}: No such file or directory", path);
                }


            },
            _ => {
                let args = &tokens[1..];

                //if command is not build in , first we will find the command in path and then check if it exits, then execute
                let path = std::env::var("PATH").unwrap();
                let paths = path.split(':');
                let mut found = false;

                for path in paths {
                    let full_path = format!("{}/{}", path, tokens[0]);

                    if std::fs::metadata(&full_path).is_ok() {
                        let mut process = std::process::Command::new(tokens[0])
                            .args(args)
                            .spawn()
                            .unwrap();

                        //waiting for the process to complete
                        let _status = process.wait().unwrap();
                        found = true;
                    }
                }

                if !found {
                    println!("{}: command not found", command);
                }
            }
        }
    }
}
