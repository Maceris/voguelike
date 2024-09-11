use crossterm::style::Color;
use enum_map::{enum_map, Enum, EnumMap};

use crate::{terminal_util::DrawInfo};

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

pub struct TileInfo {
    pub draw_info: DrawInfo
}

pub struct Map {
    pub width: u16,
    pub height: u16,
    pub tiles: Vec<Tile>
}

impl Map {
    pub fn new(width: u16, height: u16) -> Self {
        let mut result = Self {
            width,
            height,
            tiles: Vec::with_capacity((width * height).into())
        };

        for _ in 0..width*height {
            result.tiles.push(Tile::Air);
        }

        return result;
    }

    pub fn get(&self, x: u16, y: u16) -> Tile {
        let index: usize = (y * self.width + x).into();
        return self.tiles[index];
    }

    pub fn set(&mut self, x: u16, y: u16, tile: Tile) {
        let index: usize = (y * self.width + x).into();
        self.tiles[index] = tile;
    }
}

pub trait Location {
    fn get_x(&self) -> u16;
    fn get_y(&self) -> u16;
    fn set_x(&mut self, x: u16);
    fn set_y(&mut self, y: u16);
}

pub type TileMap = EnumMap<Tile, TileInfo>;

macro_rules! draw_info {
    ($color:expr, $icon:expr) => {
        DrawInfo{ color: $color, icon: $icon }
    };
}

pub fn generate_tile_map() -> TileMap {
    let result: EnumMap<Tile, TileInfo> = enum_map! {
        Tile::Air => TileInfo{
            draw_info: draw_info!(Color::Grey, ' ')
        },
        Tile::Altar => TileInfo{
            draw_info: draw_info!(Color::Grey, '_')
        },
        Tile::Building => TileInfo{
            draw_info: draw_info!(Color::Grey, 'o')
        },
        Tile::DoorClosed => TileInfo{
            draw_info: draw_info!(Color::Grey, '+')
        },
        Tile::DoorOpen => TileInfo{
            draw_info: draw_info!(Color::Grey, '/')
        },
        Tile::Entrance => TileInfo{
            draw_info: draw_info!(Color::Grey, '*')
        },
        Tile::Floor => TileInfo{
            draw_info: draw_info!(Color::Grey, '.')
        },
        Tile::Forest => TileInfo{
            draw_info: draw_info!(Color::Grey, '&')
        },
        Tile::Forge => TileInfo{
            draw_info: draw_info!(Color::Grey, '&')
        },
        Tile::Gate => TileInfo{
            draw_info: draw_info!(Color::Grey, '8')
        },
        Tile::Graveyard => TileInfo{
            draw_info: draw_info!(Color::Grey, '+')
        },
        Tile::Herbs => TileInfo{
            draw_info: draw_info!(Color::Grey, '"')
        },
        Tile::Hills => TileInfo{
            draw_info: draw_info!(Color::Grey, '~')
        },
        Tile::Hive => TileInfo{
            draw_info: draw_info!(Color::Grey, '0')
        },
        Tile::Hole => TileInfo{
            draw_info: draw_info!(Color::Grey, '*')
        },
        Tile::Lever => TileInfo{
            draw_info: draw_info!(Color::Grey, '!')
        },
        Tile::Magma => TileInfo{
            draw_info: draw_info!(Color::Grey, '=')
        },
        Tile::Mountain => TileInfo{
            draw_info: draw_info!(Color::Grey, '^')
        },
        Tile::Passage => TileInfo{
            draw_info: draw_info!(Color::Grey, '.')
        },
        Tile::Plains => TileInfo{
            draw_info: draw_info!(Color::Grey, '"')
        },
        Tile::Pool => TileInfo{
            draw_info: draw_info!(Color::Grey, '0')
        },
        Tile::Road => TileInfo{
            draw_info: draw_info!(Color::Grey, '.')
        },
        Tile::StairDown => TileInfo{
            draw_info: draw_info!(Color::Grey, '>')
        },
        Tile::StairUp => TileInfo{
            draw_info: draw_info!(Color::Grey, '<')
        },
        Tile::Statue => TileInfo{
            draw_info: draw_info!(Color::Grey, '&')
        },
        Tile::Swamp => TileInfo{
            draw_info: draw_info!(Color::Grey, '"')
        },
        Tile::Tombstone => TileInfo{
            draw_info: draw_info!(Color::Grey, '+')
        },
        Tile::TrapKnown => TileInfo{
            draw_info: draw_info!(Color::Grey, '^')
        },
        Tile::Tree => TileInfo{
            draw_info: draw_info!(Color::Grey, 'T')
        },
        Tile::Tunnel => TileInfo{
            draw_info: draw_info!(Color::Grey, '.')
        },
        Tile::Wall => TileInfo{
            draw_info: draw_info!(Color::Grey, '#')
        },
        Tile::Water => TileInfo{
            draw_info: draw_info!(Color::Grey, '=')
        },
        Tile::Web => TileInfo{
            draw_info: draw_info!(Color::Grey, '|')
        },
    };
    return result;
}
