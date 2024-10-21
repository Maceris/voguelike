use crate::{item::{Item, ItemLocation, ItemType}, map::{self, GameMap}};

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

    map.get_items_mut(2, 1).push(Item{item_type: ItemType::Abacus, location: ItemLocation::World});
    map.get_items_mut(7, 1).push(Item{item_type: ItemType::LightCrossbow, location: ItemLocation::World});
    map.get_items_mut(5, 1).push(Item{item_type: ItemType::Crystal, location: ItemLocation::World});
    map.get_items_mut(8, 2).push(Item{item_type: ItemType::Coin, location: ItemLocation::World});
    map.get_items_mut(5, 2).push(Item{item_type: ItemType::Waterskin, location: ItemLocation::World});
    map.get_items_mut(2, 3).push(Item{item_type: ItemType::Lute, location: ItemLocation::World});
    map.get_items_mut(3, 4).push(Item{item_type: ItemType::Dagger, location: ItemLocation::World});
    map.get_items_mut(10, 8).push(Item{item_type: ItemType::CrossbowBolt, location: ItemLocation::World});

}