use crate::{item::{Item, ItemType}, map::{self, GameMap}};

pub fn populate_map(map: &mut GameMap) {
    let map_height = map.height;
    let map_width = map.width;

    let y_max: u16 = map_height - 1;
    let x_max: u16 = map_width - 1;
    for y in 0..map_height {
        for x in 0..map_width {
            if y == 0 || y == y_max || x == 0 || x == x_max {
                map.set_tile(x, y, map::Tile::Wall);
            }
            else {
                map.set_tile(x, y, map::Tile::Floor);
            }
        }
    }

    //TODO(ches) remove the random items
    //TODO(ches) store items in components instead?

    map.get_items_mut(2, 1).push(Item{item_type: ItemType::Abacus});
    map.get_items_mut(7, 1).push(Item{item_type: ItemType::LightCrossbow});
    map.get_items_mut(5, 1).push(Item{item_type: ItemType::Crystal});
    map.get_items_mut(8, 2).push(Item{item_type: ItemType::Coin});
    map.get_items_mut(5, 2).push(Item{item_type: ItemType::Waterskin});
    map.get_items_mut(2, 3).push(Item{item_type: ItemType::Lute});
    map.get_items_mut(3, 4).push(Item{item_type: ItemType::Dagger});
    map.get_items_mut(10, 8).push(Item{item_type: ItemType::CrossbowBolt});

}