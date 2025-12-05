#[allow(unused_imports)]
use std::io::{self, Write};
use std::{
    env::current_exe,
    io::{stderr, stdout},
    os::unix::fs::PermissionsExt,
    process::{Output, Stdio},
};

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

        //our command is sent to the custom function to split our command into array of tokens (it also handles the edge cases of " ", '' , \)
        let mut tokenized_tokens = tokenizer(command);

        //this is for handling > case and after ">" we will have path
        let mut redirect_path: Option<String> = None;
        if let Some(pos) = tokenized_tokens.iter().position(|t| t == ">" || t == "1>") {
            if pos + 1 < tokenized_tokens.len() {
                redirect_path = Some(tokenized_tokens[pos + 1].clone());
                tokenized_tokens.remove(pos + 1);
                tokenized_tokens.remove(pos);
            }
        }

        let tokens = tokenized_tokens
            .iter()
            .map(|s| s.as_str())
            .collect::<Vec<&str>>();

        match tokens[0] {
            "exit" => break,
            "echo" => {
                let content = tokens[1..].join(" ");

                if let Some(path) = redirect_path {
                    std::fs::write(path, format!("{}\n", content));
                } else {
                    println!("{}", content)
                }
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
            "cat" => {
                let mut output = String::new();

                for file_path in &tokens[1..] {
                    match std::fs::read_to_string(file_path) {
                        Ok(content) => output.push_str(&content),
                        Err(_) => eprintln!("cat: {}: No such file or directory", file_path),
                    }
                }

                if let Some(ref new_path) = redirect_path {
                    std::fs::write(new_path, output).unwrap();
                } else {
                    print!("{}", output);
                    io::stdout().flush().unwrap();
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
                        // let mut process = std::process::Command::new(tokens[0])
                        //     .args(args)
                        //     .spawn()
                        //     .unwrap();

                        // //waiting for the process to complete
                        // let _status = process.wait().unwrap();
                        found = true;

                        let mut cmd = std::process::Command::new(tokens[0]);
                        cmd.args(args);
                        cmd.stdout(Stdio::piped());
                        cmd.stderr(Stdio::inherit());

                        let output = cmd.output();

                        match output {
                            Ok(content) => {
                                if let Some(new_path) = redirect_path.as_ref() {
                                    std::fs::write(new_path, &content.stdout).unwrap();
                                } else {
                                    print!("{}", String::from_utf8_lossy(&content.stdout))
                                }
                            }
                            Err(_) => {
                                println!("{}: command not found", tokens[0]);
                            }
                        }

                        break;
                    }
                }

                if !found {
                    println!("{}: command not found", tokens[0]);
                }
            }
        }
    }
}

fn tokenizer(input: &str) -> Vec<String> {
    let mut token = Vec::new();
    let mut current = String::new();
    let mut in_single_quotes = false;
    let mut in_double_quotes = false;
    let mut escaped = false;

    let mut chars = input.chars().peekable();

    while let Some(ch) = chars.next() {
        if escaped {
            //handling of * and \ after \ (we arfe ignoring both two and apart from these two if anything occuers after \ we not use any special proiperty of \ hence we will include \ in our token also)
            if in_double_quotes {
                match ch {
                    '"' | '\\' => current.push(ch),
                    _ => {
                        current.push('\\');
                        current.push(ch);
                    }
                }

                escaped = false;
                continue;
            } else {
                current.push(ch);

                escaped = false;
                continue;
            }
        }

        match ch {
            //this case only added to remove \ and ignore the usecase of special char after the \ in above if statement
            '\\' => {
                if !in_single_quotes {
                    escaped = true;
                    continue;
                } else {
                    current.push(ch);
                }
            }

            //case : if ' then set is single wuotes to true/false
            '\'' => {
                if !in_double_quotes {
                    in_single_quotes = !in_single_quotes;
                } else {
                    current.push(ch);
                }
            }

            '"' => {
                if !in_single_quotes {
                    in_double_quotes = !in_double_quotes;
                } else {
                    current.push(ch);
                }
            }

            //case  : if whitespace and outside ' , then if our curretn string has something then push it in token vec
            c if c.is_whitespace() && (!in_single_quotes && !in_double_quotes) => {
                if !current.is_empty() {
                    token.push(current.clone());
                    current.clear();
                }
            }

            //case  : everything => if noremal words or inside ' regreger rgegreg regege'
            _ => current.push(ch),
        }
    }

    if !current.is_empty() {
        token.push(current);
    }

    return token;
}
