pub mod main_funcs;

use main_funcs::*;
fn main() {
    let testweapon : Item = Item{name : "Test weapon".to_string(), id : 0, descr : "A test weapon".to_string(), stats : [1.0,1.0,0.0,0.0]};
    let testminer : Item = Item{name : "Test mining tool".to_string(), id : 1, descr : "A test mining tool".to_string(), stats : [0.0,0.0,1.0,1.0]};
    print_item(testweapon, true);
    println!("\n\n\n");
    print_item(testminer, true);

}
