pub mod main_funcs;

use main_funcs::*;
use rand::Rng;

const ITEMS : [Item<'static>; 1] = [Item{name : "Fires edge", id : 0, descr : "A fiery blade, that pierces its foes\nwith fiery slashes", stats : [1.3, 0.2, 0.0, 0.0]}];

pub fn get_random_item() -> String {
    let items = ITEMS;
    let mut rng = rand::thread_rng();
    let index : usize = rng.gen_range(0..ITEMS.len());
    let item: &Item = &items[index];

    return item_to_string(item);
}