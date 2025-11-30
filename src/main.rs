#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    // TODO: Uncomment the code below to pass the first stage

    loop{

        
        print!("$ ");
        io::stdout().flush().unwrap();
        
        let mut command = String::new();
        io::stdin().read_line(&mut command).unwrap();

      

        let command  = command.trim();


        //this will break the command into arrya of tokens
        let tokens = command.split_whitespace().collect::<Vec<&str>>();

        match tokens[0]{
            "exit" => break,
            "echo" => println!("{}", tokens[1..].join(" ")),
            // "type" => println!("{} is a shell builtin", tokens[1..].join(" ")),
            "type" => {
                let arg = tokens[1];
                match arg {
                    "exit"|"echo"|"type" =>{
                        println!("{} is a shell builtin",arg);
                    },
                    _ => println!("{}: not found",tokens[1..].join(" "))
                }
            }
            _ =>  println!("{}: command not found",command)
        } 
       
    }


}
