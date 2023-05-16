use std::any::type_name;

//Structs
//  Item struct, made for saving item information
pub struct Item<'a> {
    //Name: The name of the item
    pub name : &'a str,
    //Id: The itemID for easier randomization
    pub id : i32,
    //Descr: Description, the description of the item
    pub descr : &'a str,
    //Stats are a list of floats, the indexes represents these stats:
    //1: Damage
    //2: Attack speed
    //3: Mining power
    //4: Mining speed
    pub stats : [f64; 4],
}
//  Armour struct, made for saving Armour information
pub struct Armour<'a> {
    //Name: The name of the item
    pub name : &'a str,
    //Id: The itemID for easier randomization
    pub id : i32,
    //Descr: Description, the description of the item
    pub descr : &'a str,
    //Stats are a list of floats, the indexes represents these stats:
    //1: Strength
    //2: Speed
    pub stats : [f64; 2],
}
//  Inventory struct, basically just a list, but its possible to add more fields to it
pub struct Inventory {
    //List for storing an item inventory
    pub item_inventory_list : Vec<Item<'static>>,
    //Variable to save the equipped item from Item inventory. This is saved as 
    pub equipped_item : usize,

    //List for storing armour inventory
    pub armour_inventory_list : Vec<Armour<'static>>,
    //A set size list of equipped armour pieces
    pub equipped_armour : [usize; 4]
}
//  Player struct, made for saving playerstats and make it easier to save into a file
pub struct Player {
    //Inventory: The inventory of the player
    pub inventory : Inventory,

    //HEALTH
    //Health: Decudes how much health the player has left
    pub health : i32,
    //Max Health: Decides the max amount the player can heal to
    pub max_health : i32,

    //STATS
    //Strength: Influences the players damage and hit chance
    pub strength : f64,
    //Speed: Influences the players dodging power, and action completion speed
    pub speed : f64,
    
}
// Enemy type struct, used to define an enemy type
//  This struct includes the stats, but the health will be decided in the enemy struct
pub struct EnemyType {
    //Name: Stores the name of the enemy
    pub name : String,

    //STATS
    //Speed: Influences the enemy's attack speed and dodging chance
    pub speed : f64,
    //Strength: Influences the enemy's attack damage and hit chance
    pub strength : f64,
}
//Enemy struct, saves information about one particular enemy
pub struct Enemy {
    //Type: The EnemyType this enemy inherits from
    pub self_type : EnemyType,

    //HEALTH
    //Health: Decides how much health the enemy has left
    pub health : i32,
    //Max Health: Decides the max amount the enemy can heal to
    pub max_health : i32,

    //STATS
    //THESE ARE OPTIONAL ALL SHALL NOT BE FILLED
    //Speed: Influences the enemy's attack speed and dodging chance
    pub speed : Option<f64>,
    //Strength: Influences the enemy's attack damage and hit chance
    pub strength : Option<f64>,
}



//Struct impls
impl Player {
    pub fn add_to_item_inventory(&mut self, object : Item<'static>) {
        self.inventory.item_inventory_list.push(object);
    }
    pub fn add_to_armour_inventory(&mut self, object : Armour<'static>) {
        self.inventory.armour_inventory_list.push(object);
    }
    pub fn change_stat(&mut self, stat : i32, value : f64) {
        if stat == 1 {
            self.strength = value;}
        if stat == 2 {
            self.speed = value;}
    }
}
impl EnemyType {
    pub fn change_stat(&mut self, stat : i32, value : f64) {
        if stat == 1 {
            self.strength = value;}
        if stat == 2 {
            self.speed = value;}
    }
}
impl Enemy {
    pub fn init(&mut self) {
        self.strength = Some(self.self_type.strength);
        self.speed = Some(self.self_type.speed);
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

pub fn item_to_string(item : &Item<'static>) -> String {
    let mut return_val : String = format!("{}\nItem ID: {}\n\n{}\n\nSTATS:", item.name, item.id, item.descr);
    if item.stats[0] != 0.0{
        let r : String = format!("\n  Damage: {}\n  Attack speed: {}", item.stats[0], item.stats[1]);
        return_val.push_str(&r);
    }
    if item.stats[2] != 0.0{
        let r : String = format!("\n  Mining power: {}\n  Mining speed: {}\n", item.stats[2], item.stats[3]);
        return_val.push_str(&r);
    }
    

    return return_val;
}