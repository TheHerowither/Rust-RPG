pub mod main_funcs;

use main_funcs::*;
use std::{io::Write};


fn Input(name : &str) -> String {
    let mut line : String = String::new();
    print!("{}", name);

    std::io::stdout().flush().unwrap();
    std::io::stdin().read_line(&mut line).expect("Error: Could not read a line");


    return line.trim().to_string();
}