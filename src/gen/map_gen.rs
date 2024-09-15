use crate::map::{self, GameMap};


pub fn populate_map(map: &mut GameMap) {
    let map_height = map.height;
    let map_width = map.width;

    let y_max: u16 = map_height - 1;
    let x_max: u16 = map_width - 1;
    for y in 0..map_height {
        for x in 0..map_width {
            if y == 0 || y == y_max || x == 0 || x == x_max {
                map.set(x, y, map::Tile::Wall);
            }
            else {
                map.set(x, y, map::Tile::Floor);
            }
        }
    }
}