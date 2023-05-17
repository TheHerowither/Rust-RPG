pub mod interactions;

//use interactions::main_funcs::*;
use interactions::*;
use interactions::main_funcs::*;

use std::{io::Write};

use crate::shell::interactions::main_funcs::item_to_string;


fn input(name : &str) -> String {
    let mut line : String = String::new();
    print!("{}", name);

    std::io::stdout().flush().unwrap();
    std::io::stdin().read_line(&mut line).expect("Error: Could not read a line");


    return line.trim().to_string();
}

pub fn debug_loop() {
    loop {
        //Get the input from the shell
        let cmd: String = input("> ");
        let split: std::str::Split<&str> = cmd.split(" ");
        let split: Vec<&str> = split.collect();
        //Handle the input:
        if cmd == "exit" {
            break;}
        if cmd == "rng_item" {
            println!("{}", get_random_item());}
        if cmd == "rng_armour" {
            println!("{}", get_random_armour());}
        if split[0] == "get_item_by_id" {
            let r = item_to_string(get_item_by_id(split[1].to_string().parse::<i32>().unwrap()));
            println!("{}", r);
        }
        if split[0] == "get_armour_by_id" {
            let r = armour_to_string(get_armour_by_id(split[1].to_string().parse::<i32>().unwrap()));
            println!("{}", r);
        }
        if cmd == "?" {
            println!("exit - Closes the program");
            println!("rng_item - prints out a random item");
            println!("rng_armour - prints out a random armour");
            println!("get_item_by_id id - prints out the item with the specified id");
            println!("get_armour_by_id id - prints out the armour with the specified id");
            //TODO: Print out all of the valid commands, and what they do
        }
    }
}
pub fn main_loop() {
    
}