pub mod shell;

use shell::*;
use configparser::ini::*;

fn main() {
    let mut ini = Ini::new();
    let _map = ini.load("opt.ini");
    let debug: String = ini.get("SETTINGS", "DEBUG").unwrap();
    let debug_bool : i16 = debug.parse::<i16>().unwrap();
    
    if debug_bool == 1 {
        debug_loop();}
    else {
        main_loop();}
}
