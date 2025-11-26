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

        let tokens = command.split_whitespace().collect::<Vec<&str>>();
        // println!("{:?}",iter.next());

        match tokens[0]{
            "exit" => break,
            "echo" => println!("{}", tokens[1..].join(" ")),
            _ =>  println!("{}: command not found",command)
        } 
       
    }


}
