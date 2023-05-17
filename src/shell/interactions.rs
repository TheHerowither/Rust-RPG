pub mod main_funcs;

use main_funcs::*;
use rand::Rng;

const ITEMS : [Item<'static>; 2] = [Item{name : "Fires edge", id : 0, descr : "A fiery blade, that pierces its foes\nwith fiery slashes", stats : [1.3, 0.2, 0.0, 0.0], damage_type : "Fire"}, Item{name : "Herring the red", id : 1, descr : "A red herring", stats : [0.0, 0.0, 0.0, 0.0], damage_type : ""}];
const ARMOURS : [Armour<'static>; 1] = [Armour{name : "Firestone Helmet", id : 0, descr : "A helmet, that looks\nto be made of molten rock", armour : 1.0, stats : [0.3, 0.6], damage_resistance : "Fire", damage_resistance_addition : 0.7}];

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