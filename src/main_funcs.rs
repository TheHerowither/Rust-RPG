use std::any::type_name;

//Structs
//  Item struct, made for saving item information
pub struct Item {
    //Name: The name of the item
    pub name : String,
    //Id: The itemID for easier randomization
    pub id : i32,
    //Descr: Description, the description of the item
    pub descr : String,
    //Stats are a list of floats, the indexes represents these stats:
    //1: Damage
    //2: Attack speed
    //3: Minig power
    //4: Mining speed
    pub stats : [f64; 4],
}
//  Armour struct, made for saving Armour information
pub struct Armour {
    //Name: The name of the item
    pub name : String,
    //Id: The itemID for easier randomization
    pub id : i32,
    //Descr: Description, the description of the item
    pub descr : String,
    //Stats are a list of floats, the indexes represents these stats:
    //1: Strength
    //2: Speed
    pub stats : [f64; 2],
}
//  Inventory struct, basically just a list, but its possible to add more fields to it
pub struct Inventory {
    //List for storing an item inventory
    pub item_inventory_list : Vec<Item>,
    //Variable to save the equipped item from Item inventory. This is saved as 
    pub equipped_item : usize,

    //List for storing armour inventory
    pub armour_inventory_list : Vec<Armour>,
    //A set size list of equipped armour pieces
    pub equipped_armour : [usize; 4]
}
//  Player struct, made for saving playerstats and make it easier to save into a file
pub struct Player {
    //Strength: Influences the players damage
    pub strength : f64,
    //Speed: Influences the players dodging power, and action completion speed
    pub speed : f64,

    //Inventory: The inventory of the player
    pub inventory : Inventory,
    
}



//Struct impls
impl Player {
    pub fn add_to_item_inventory(&mut self, object : Item) {
        self.inventory.item_inventory_list.push(object);
    }
    pub fn add_to_armour_inventory(&mut self, object : Armour) {
        self.inventory.armour_inventory_list.push(object);
    }
}


pub fn type_of<T>(_: T) -> &'static str {
    type_name::<T>()
}

//Funcs
pub fn print_item<'a>(item : Item, debug : bool){
    println!("{}", item.name);
    if debug {println!("Item ID: {}", item.id);}
    println!("\n{}\n\n", item.descr);

    println!("Stats:");
    if item.stats[0] != 0.0{println!("Damage: {}\nAttack speed: {}\n", item.stats[0],item.stats[1]);}
    if item.stats[2] != 0.0{println!("Mining power: {}\nMining speed: {}\n", item.stats[2],item.stats[3]);}
}
pub fn print_armour<'a>(armour : Armour, debug : bool){
    println!("{}", armour.name);
    if debug {println!("Item ID: {}", armour.id);}
    println!("\n{}\n\n", armour.descr);

    println!("Stats:");
    let mut i : usize = 0;

    //Stat name list, a probably temporary list for storing the names of the stats if the "stats" list
    let stat_names : [&str; 2] = ["Strength", "Speed"];
    while i < armour.stats.len() {
        println!("{}: {}", stat_names[i], armour.stats[i]);
        i += 1;
    }
}