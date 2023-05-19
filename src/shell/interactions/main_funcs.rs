use std::any::type_name;
use rand::Rng;

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
    //Damage type: The type of damage the item deals, set to "" if item has no damage type
    pub damage_type : &'a str,
}
//  Armour struct, made for saving an entire Armour set's information
pub struct Armour<'a> {
    //Name: The name of the item
    pub name : &'a str,
    //Id: The itemID for easier randomization
    pub id : i32,
    //Descr: Description, the description of the item
    pub descr : &'a str,

    //Armour: the armour that this armourpiece gives
    pub armour : f32,

    //STATS
    //Stats are a list of floats, the indexes represents these stats:
    //1: Strength
    //2: Speed
    pub stats : [f64; 2],
    //Damage resistance: the armourpiece has increased resistance against that type of damage set to "" if none
    pub damage_resistance : &'a str,
    //Damage resistance addition: the added armour against the damage resistance type
    pub damage_resistance_addition : f32,
}
//  Inventory struct, basically just a list, but its possible to add more fields to it
pub struct Inventory {
    //List for storing an item inventory
    pub item_inventory_list : Vec<&'static Item<'static>>,
    //Variable to save the equipped item from Item inventory. This is saved as 
    pub equipped_item : usize,

    //List for storing armour inventory
    pub armour_inventory_list : Vec<&'static Armour<'static>>,
    //A set size list of equipped armour pieces
    pub equipped_armour : usize
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
    //Strength: Damage multiplier
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
    //Strength: Damage multiplier
    pub strength : f64,
}
//Enemy struct, saves information about one particular enemy
pub struct Enemy {
    //Type: The EnemyType this enemy inherits from
    pub self_type : EnemyType,

    //Inventory: stores items held by the enemy
    pub inventory : Inventory,

    //HEALTH
    //Health: Decides how much health the enemy has left
    pub health : i32,
    //Max Health: Decides the max amount the enemy can heal to
    pub max_health : i32,

    //STATS
    //THESE ARE TO BE SET TO 0
    //Speed: Influences the enemy's attack speed and dodging chance
    pub speed : f64,
    //Strength: Influences the enemy's attack damage and hit chance
    pub strength : f64,

    //Can Drop: A ItemPool that stores the items/armours this enemy can drop on death
    pub can_drop : ItemPool,
}
//Item pool struct, stores a item pool
pub struct ItemPool {
    //PoolItemIDs: List of item ID's for items in this pool
    pub pool_item_ids : Vec<i32>,
    //PoolArmourIDs: List of armour ID's for items in this pool
    pub pool_armour_ids : Vec<i32>,
}



//Struct impls
impl Player {
    pub fn add_to_item_inventory(&mut self, object : &'static Item<'static>) {
        self.inventory.item_inventory_list.push(object);
    }
    pub fn add_to_armour_inventory(&mut self, object : &'static Armour<'static>) {
        self.inventory.armour_inventory_list.push(object);
    }
    pub fn change_stat(&mut self, stat : i32, value : f64) {
        if stat == 1 {
            self.strength = value;}
        if stat == 2 {
            self.speed = value;}
    }
    pub fn damage(&mut self, value : i32) {
        self.health -= value;
    }
    pub fn heal(&mut self, value : i32) {
        if self.health >= self.max_health - value {
            self.health += value;
        }
        else if self.health > self.max_health {
            self.health += self.max_health - self.health;
        }
        
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
        self.strength = self.self_type.strength;
        self.speed = self.self_type.speed;
    }
    pub fn damage(&mut self, value : i32) {
        self.health -= value;
    }
    pub fn heal(&mut self, value : i32) {
        if self.health >= self.max_health - value {
            self.health += value;
        }
        else if self.health > self.max_health {
            self.health += self.max_health - self.health;
        }
        
    }
    pub fn death(&mut self) -> i32{
        let mut rng: rand::rngs::ThreadRng = rand::thread_rng();
        let armour_index : usize = rng.gen_range(0..self.can_drop.pool_armour_ids.len());
        let item_index : usize = rng.gen_range(0..self.can_drop.pool_item_ids.len());

        let items: [i32; 2] = [self.can_drop.pool_armour_ids[armour_index], self.can_drop.pool_item_ids[item_index]];
        return items[rng.gen_range(0..1)];
    }
}


pub fn type_of<T>(_: T) -> &'static str {
    type_name::<T>()
}

//Funcs
pub fn item_to_string(item : &Item<'static>) -> String {
    let mut return_val : String = format!("{}\nItem ID: {}\n\n{}\n\nSTATS:", item.name, item.id, item.descr);
    if item.stats[0] != 0.0{
        let r : String = format!("\n  Damage: {}\n  Attack speed: {}", item.stats[0], item.stats[1]);
        return_val.push_str(&r);
    }
    else if item.stats[2] != 0.0{
        let r : String = format!("\n  Mining power: {}\n  Mining speed: {}\n", item.stats[2], item.stats[3]);
        return_val.push_str(&r);
    }
    else {
        return_val.push_str(&"\nNo stat bonuses");
    }

    if item.damage_type != "" {
        let r : String = format!("\n  Damage type: {}", item.damage_type);
        return_val.push_str(&r)
    }
    

    return return_val;
}
pub fn armour_to_string(item : &Armour<'static>) -> String {
    let mut return_val : String = format!("{}\nArmour ID: {}\n\n{}\n\nSTATS:", item.name, item.id, item.descr);
    if item.stats[0] != 0.0{
        let r : String = format!("\n  Strength: {}", item.stats[0]);
        return_val.push_str(&r);
    }
    else if item.stats[1] != 0.0{
        let r : String = format!("\n  Speed: {}", item.stats[1]);
        return_val.push_str(&r);
    }
    else {
        return_val.push_str(&"\nNo stat bonuses");
    }

    if item.damage_resistance != "" {
        let r : String = format!("\n  Damage resistance: {} - {}", item.damage_resistance, item.damage_resistance_addition);
        return_val.push_str(&r)
    }
    

    return return_val;
}