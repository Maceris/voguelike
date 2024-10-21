use enum_map::Enum;

use crate::item::Item;

#[derive(Clone, Copy, Debug, Enum)]
pub enum Tile {
    Air,
    Altar,
    Building,
    DoorClosed,
    DoorOpen,
    Entrance,
    Floor,
    Forest,
    Forge,
    Gate,
    Graveyard,
    Herbs,
    Hills,
    Hive,
    Hole,
    Lever,
    Magma,
    Mountain,
    Passage,
    Plains,
    Pool,
    Road,
    StairDown,
    StairUp,
    Statue,
    Swamp,
    Tombstone,
    TrapKnown,
    Tree,
    Tunnel,
    Wall,
    Water,
    Web,
}

pub type MapID = u32;

pub struct GameMap {
    pub id: MapID,
    pub width: u16,
    pub height: u16,
    pub items: Vec<Vec<Item>>,
    pub tiles: Vec<Tile>,
}

impl GameMap {
    pub fn new(id: MapID, width: u16, height: u16) -> Self {
        let mut result = Self {
            id,
            width,
            height,
            items: Vec::with_capacity((width * height).into()),
            tiles: Vec::with_capacity((width * height).into()),
        };

        for _ in 0..width*height {
            result.tiles.push(Tile::Air);
            result.items.push(Vec::new());
        }

        return result;
    }

    pub fn coordinates_to_index(&self, x: u16, y: u16) -> usize {
        return (y * self.width + x).into();
    }

    pub fn index_to_coordinates(&self, index: usize) -> (u16, u16) {
        let y: usize = index / self.width as usize;
        let x: usize = index % self.width as usize;
        return (x as u16, y as u16);
    }
    
    pub fn get_items(&self, x: u16, y: u16) -> &Vec<Item> {
        let index: usize = self.coordinates_to_index(x, y);
        return &self.items[index];
    }

    pub fn get_items_mut(&mut self, x: u16, y: u16) -> &mut Vec<Item> {
        let index: usize = self.coordinates_to_index(x, y);
        return &mut self.items[index];
    }

    pub fn get_tile(&self, x: u16, y: u16) -> &Tile {
        let index: usize = self.coordinates_to_index(x, y);
        return &self.tiles[index];
    }

    pub fn set_tile(&mut self, x: u16, y: u16, tile: Tile) {
        let index: usize = self.coordinates_to_index(x, y);
        self.tiles[index] = tile;
    }

    pub fn empty_map() -> Self {
        Self {
            id: 0,
            width: 0,
            height: 0,
            items: Vec::new(),
            tiles: Vec::new(),
        }
    }
}
