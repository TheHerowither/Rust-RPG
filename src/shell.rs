pub mod interactions;

//use interactions::main_funcs::*;
use interactions::*;
use interactions::main_funcs::*;
use rand::Rng;


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
    let fire_pool : ItemPool = ItemPool { pool_item_ids: vec![0], pool_armour_ids: vec![0] };
    let valid_pools : Vec<String> = vec!["fire_pool".to_string()];
    loop {
        //Get the input from the shell
        let cmd: String = input("RPG - debug shell> ");
        let split: std::str::Split<&str> = cmd.split(" ");
        let split: Vec<&str> = split.collect();
        println!();
        //Handle the input:
        if cmd == "exit" {
            break;}
        else if cmd == "rng_item" {
            println!("{}", get_random_item());}
        else if cmd == "rng_armour" {
            println!("{}", get_random_armour());}
        else if split[0] == "get_item_by_id" {
            let item: &'static Item = get_item_by_id(split[1].to_string().parse::<i32>().unwrap());
            if item.name != "Invalid"{
                let r = item_to_string(item);
                println!("{}", r)  
            }
            else {
                //println!("Item ID: {} is invalid", split[1]);
            }
        }
        else if split[0] == "get_armour_by_id" {
            let item: &'static Armour = get_armour_by_id(split[1].to_string().parse::<i32>().unwrap());
            if item.name != "Invalid"{
                let r = armour_to_string(item);
                println!("{}", r);
            }
            else {
                //println!("Item ID: {} is invalid", split[1]);
            }
        }
        else if split[0] == "get_item_ids"{
            println!("Item ID range: 0 - {}", TOTALITEMID);}
        else if split[0] == "get_armour_ids"{
            println!("Armour ID range: 0 - {}", TOTALARMOURSID);}
        else if split[0] == "get_random_item_pool_item" {
            let mut rng = rand::thread_rng();
            if split[1] == "fire_pool" {
                let i = &vec![item_to_string(get_random_item_from_pool(&fire_pool)), armour_to_string(get_random_armour_from_pool(&fire_pool))][rng.gen_range(0..2)];
                println!("{}", i);
            }
        }
        else if split[0] == "get_all_pools" {
            println!("{:?}", valid_pools);
        }
        
        else if cmd == "?" {
            //Print out all of the valid commands, and what they do
            println!("exit - Closes the program");
            println!("rng_item - prints out a random item");
            println!("rng_armour - prints out a random armour");
            println!("get_item_by_id id - prints out the item with the specified id");
            println!("get_armour_by_id id - prints out the armour with the specified id");
            println!("get_item_ids - prints out max number of item id's");
            println!("get_armour_ids - prints out max number of armour id's");
            println!("get_random_item_pool_item ItemPool - prints out a random item from the specified pool");
            println!("get_all_pools - print out all valid item pools");
        }
        else {
            println!("Invalid command '{}'\nTry writing '?' to see valid commands.\nRemember: it IS context sensitive!", cmd);
        }
        println!();
    }
}
pub fn main_loop() {
    loop {
        let cmd : String = input("> ");
        let split: std::str::Split<&str> = cmd.split(" ");
        let split: Vec<&str> = split.collect();

        //Handle the input:
        if split[0] == "exit" {
            break;}
        else if split[0] == "?"{
            //Print out all of the possible commands and what they do
            println!("exit - Closes the program");
        }
        else {
            println!("Invalid command '{}'\nTry writing '?' to see the commands\nRemember: it IS context sensitive", cmd);
        }
        println!();
    }
}