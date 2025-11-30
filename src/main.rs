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
        // let tokens = command.split_whitespace().collect::<Vec<&str>>();

        let tokenized_tokens = tokenizer(command);
        let tokens = tokenized_tokens
            .iter()
            .map(|s| s.as_str())
            .collect::<Vec<&str>>();

        match tokens[0] {
            "exit" => break,
            "echo" => {
                println!("{}", tokens[1..].join(" "))
            }

            "type" => {
                let arg = tokens[1];
                match arg {
                    "exit" | "echo" | "type" | "pwd" | "cd" => {
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
            }
            "pwd" => {
                let curr_dir = std::env::current_dir().unwrap();
                println!("{}", curr_dir.display());
            }
            "cd" => {
                if tokens.len() < 2 {
                    continue;
                }

                let path = tokens[1];

                //handle the ~ (home) pathing
                if path == "~" {
                    let home = std::env::home_dir().unwrap();
                    std::env::set_current_dir(home);
                    continue;
                }

                // "/" it is for abosolute path
                if path.starts_with('/') {
                    let path_exists = std::path::Path::new(path).exists();
                    let is_path_dir = std::path::Path::new(path).is_dir();

                    if path_exists && is_path_dir {
                        std::env::set_current_dir(path);
                    } else {
                        println!("cd: {}: No such file or directory", path);
                    }

                //else is use to handle the relative pathing
                } else {
                    let curr_dir = std::env::current_dir().unwrap();
                    let new_joined_path = curr_dir.join(path);

                    //this will make the cleaner form of "/foo/test/../test/bar.rs"  -> "/foo/test/bar.rs"
                    let new_path = new_joined_path.canonicalize().unwrap();

                    if new_path.is_dir() {
                        std::env::set_current_dir(new_path);
                    } else {
                        println!("cd: {}: No such file or directory", path);
                    }
                }
            }
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

                        break;
                    }
                }

                if !found {
                    println!("{}: command not found", command);
                }
            }
        }
    }
}

fn tokenizer(input: &str) -> Vec<String> {
    let mut token = Vec::new();
    let mut current = String::new();
    let mut in_single_quotes = false;

    let mut chars = input.chars().peekable();

    while let Some(ch) = chars.next() {
        match ch {
            //case1 : if ' then set is single wuotes to true/false
            '\'' => {
                in_single_quotes = !in_single_quotes;
            }

            //case 2 : if whitespace and outside ' , then if our curretn string has something then push it in token vec
            c if c.is_whitespace() && !in_single_quotes => {
                if !current.is_empty() {
                    token.push(current.clone());
                    current.clear();
                }
            }

            //case 3 : everything => if noremal words or inside ' regreger rgegreg regege'
            _ => current.push(ch),
        }
    }

    if !current.is_empty() {
        token.push(current);
    }

    return token;
}
