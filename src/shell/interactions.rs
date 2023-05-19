pub mod main_funcs;

use main_funcs::*;
use rand::Rng;
use std::panic as panic;

//A const for saving ALL item's that exists in the game
pub const ITEMS : [Item<'static>; 3] = [Item{name : "Fires edge", id : 0, descr : "A fiery blade, that pierces its foes\nwith fiery slashes", stats : [1.3, 0.2, 0.0, 0.0], damage_type : "Fire"}, 
                    Item{name : "Stonk", id : 1, descr : "A pickaxe that produces STONKS", stats : [0.0, 0.0, 1.0, 3.2], damage_type : ""}, 
                    Item{name : "Herring the red", id : 2, descr : "A red herring", stats : [0.0, 0.0, 0.0, 0.0], damage_type : ""}];
pub const TOTALITEMID : i32 = ITEMS.len() as i32 - 1;
//A const for saving ALL Armour's that exists in the game
pub const ARMOURS : [Armour<'static>; 1] = [Armour{name : "Firestone Helmet", id : 0, descr : "A helmet, that looks\nto be made of molten rock", armour : 1.0, stats : [0.3, 0.6], damage_resistance : "Fire", damage_resistance_addition : 0.7}];
pub const TOTALARMOURSID : i32 = ARMOURS.len() as i32 - 1;

const INVALIDITEM: &Item<'static> = &Item{name : "Invalid", id : 99999, descr : "", stats : [0.0,0.0,0.0,0.0], damage_type : ""};
const INVALIDARMOUR: &Armour<'static> = &Armour{name : "Invalid", id : 99999, descr : "", stats : [0.0,0.0], damage_resistance : "", armour : 0.0, damage_resistance_addition : 0.0};

pub fn get_random_item() -> String {
    let mut rng = rand::thread_rng();
    let index : usize = rng.gen_range(0..ITEMS.len());
    let item: &Item = &ITEMS[index];

    return item_to_string(item);
}
pub fn get_random_armour() -> String {
    let mut rng = rand::thread_rng();
    let index : usize = rng.gen_range(0..ARMOURS.len());
    let item: &Armour = &ARMOURS[index];

    return armour_to_string(item);
}
pub fn get_item_by_id(id : i32) -> &'static Item<'static>{
    let result = panic::catch_unwind(|| {
        let _index: usize = ITEMS.iter().position(|r| r.id == id).unwrap();
    });
    if result.is_err() {
        println!("Item ID: {} is invalid.\nIgnored panick", id);
        return INVALIDITEM;
    }
    else {
        let _index: usize = ITEMS.iter().position(|r| r.id == id).unwrap();
        return &ITEMS[_index];
    }
}
pub fn get_armour_by_id(id : i32) -> &'static Armour<'static>{
    let result = panic::catch_unwind(|| {
        let _index: usize = ARMOURS.iter().position(|r| r.id == id).unwrap();
    });
    if result.is_err() {
        println!("Armour ID: {} is invalid.\nIgnored panick", id);
        return INVALIDARMOUR;
    }
    else {
        let _index: usize = ARMOURS.iter().position(|r| r.id == id).unwrap();
        return &ARMOURS[_index];
    }
    
}
pub fn calculate_damage(enemy : &Enemy, player : &Player) -> [i32; 2] {
    let player_weapon: &Item = &player.inventory.item_inventory_list[player.inventory.equipped_item];
    let player_armour: &Armour = &player.inventory.armour_inventory_list[player.inventory.equipped_armour];
    let player_strength: f64 = player.strength;
    let player_damage_type: &str = player_weapon.damage_type;

    let enemy_strenght: f64 = enemy.strength;
    let enemy_weapon: &Item = &enemy.inventory.item_inventory_list[enemy.inventory.equipped_item];
    let enemy_armour: &Armour = &enemy.inventory.armour_inventory_list[enemy.inventory.equipped_armour];
    let enemy_damage_type: &str = enemy_weapon.damage_type;

    //Calculations for armour classes
    let mut player_armour_class = player_armour.armour;
    if enemy_damage_type == player_armour.damage_resistance {
        player_armour_class += player_armour.damage_resistance_addition;
    }
    let mut enemy_armour_class = enemy_armour.armour;
    if player_damage_type == enemy_armour.damage_resistance {
        enemy_armour_class += enemy_armour.damage_resistance_addition;
    }

    //Calculations for raw damage
    let player_raw_damage: f64 = player_weapon.stats[0] * player_strength;
    let enemy_raw_damage: f64 = enemy_strenght * enemy_weapon.stats[0];

    //Calculations for damage
    let player_damage: f64 = player_raw_damage - enemy_armour_class as f64;
    let enemy_damage: f64 = enemy_raw_damage - player_armour_class as f64;

    return [player_damage as i32, enemy_damage as i32];
}
pub fn get_random_item_from_pool(item_pool : &ItemPool) -> &'static Item<'static>{
    let mut rng: rand::rngs::ThreadRng = rand::thread_rng();
    let index : usize = rng.gen_range(0..item_pool.pool_item_ids.len());
    let id: i32 = item_pool.pool_item_ids[index];

    return get_item_by_id(id);
}
pub fn get_random_armour_from_pool(item_pool : &ItemPool) -> &'static Armour<'static>{
    let mut rng: rand::rngs::ThreadRng = rand::thread_rng();
    let index : usize = rng.gen_range(0..item_pool.pool_armour_ids.len());
    let id: i32 = item_pool.pool_armour_ids[index];

    return get_armour_by_id(id);
}
pub fn enemy_death(mut enemy : Enemy) -> &'static Item<'static>{
    return get_item_by_id(enemy.death());
}
pub fn harvest(mut player : Player) {
    let item: &Item = player.inventory.item_inventory_list[player.inventory.equipped_item];

    player.add_money((item.stats[2] * item.stats[3]) as i64);
}