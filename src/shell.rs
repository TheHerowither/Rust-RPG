pub mod interactions;

//use interactions::main_funcs::*;
use interactions::*;

use std::{io::Write};


fn input(name : &str) -> String {
    let mut line : String = String::new();
    print!("{}", name);

    std::io::stdout().flush().unwrap();
    std::io::stdin().read_line(&mut line).expect("Error: Could not read a line");


    return line.trim().to_string();
}

pub fn main_loop() {
    loop {
        //Get the input from the shell
        let cmd = input("> ");
        //Handle the input:
        if cmd == "exit" {
            break;}
        if cmd == "rng_item" {
            println!("{}", get_random_item());}
        if cmd == "?" {
            println!("exit - Closes the program");
            //TODO: Print out all of the valid commands, and what they do
        }
    }
}