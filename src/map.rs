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
    pub items: Vec<Item>,
    pub tiles: Vec<Tile>,
}

impl GameMap {
    pub fn new(id: MapID, width: u16, height: u16) -> Self {
        let mut result = Self {
            id,
            width,
            height,
            items: Vec::new(),
            tiles: Vec::with_capacity((width * height).into()),
        };

        for _ in 0..width*height {
            result.tiles.push(Tile::Air);
        }

        return result;
    }

    pub fn get(&self, x: u16, y: u16) -> &Tile {
        let index: usize = (y * self.width + x).into();
        return &self.tiles[index];
    }

    pub fn set(&mut self, x: u16, y: u16, tile: Tile) {
        let index: usize = (y * self.width + x).into();
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
