use crossterm::style::Color;
use enum_map::{enum_map, Enum, EnumMap};

use crate::ui::terminal::terminal_util::DrawInfo;

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

pub type MapID = u32;

pub struct GameMap {
    pub width: u16,
    pub height: u16,
    pub tiles: Vec<Tile>
}

impl GameMap {
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

    pub fn empty_map() -> Self {
        Self {
            width: 0,
            height: 0,
            tiles: Vec::new()
        }
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
            draw_info: draw_info!(Color::DarkGrey, ' ')
        },
        Tile::Altar => TileInfo{
            draw_info: draw_info!(Color::DarkGrey, '_')
        },
        Tile::Building => TileInfo{
            draw_info: draw_info!(Color::DarkGrey, 'o')
        },
        Tile::DoorClosed => TileInfo{
            draw_info: draw_info!(Color::DarkGrey, '+')
        },
        Tile::DoorOpen => TileInfo{
            draw_info: draw_info!(Color::DarkGrey, '/')
        },
        Tile::Entrance => TileInfo{
            draw_info: draw_info!(Color::DarkGrey, '*')
        },
        Tile::Floor => TileInfo{
            draw_info: draw_info!(Color::DarkGrey, '.')
        },
        Tile::Forest => TileInfo{
            draw_info: draw_info!(Color::DarkGrey, '&')
        },
        Tile::Forge => TileInfo{
            draw_info: draw_info!(Color::DarkGrey, '&')
        },
        Tile::Gate => TileInfo{
            draw_info: draw_info!(Color::DarkGrey, '8')
        },
        Tile::Graveyard => TileInfo{
            draw_info: draw_info!(Color::DarkGrey, '+')
        },
        Tile::Herbs => TileInfo{
            draw_info: draw_info!(Color::DarkGrey, '"')
        },
        Tile::Hills => TileInfo{
            draw_info: draw_info!(Color::DarkGrey, '~')
        },
        Tile::Hive => TileInfo{
            draw_info: draw_info!(Color::DarkGrey, '0')
        },
        Tile::Hole => TileInfo{
            draw_info: draw_info!(Color::DarkGrey, '*')
        },
        Tile::Lever => TileInfo{
            draw_info: draw_info!(Color::DarkGrey, '!')
        },
        Tile::Magma => TileInfo{
            draw_info: draw_info!(Color::DarkGrey, '=')
        },
        Tile::Mountain => TileInfo{
            draw_info: draw_info!(Color::DarkGrey, '^')
        },
        Tile::Passage => TileInfo{
            draw_info: draw_info!(Color::DarkGrey, '.')
        },
        Tile::Plains => TileInfo{
            draw_info: draw_info!(Color::DarkGrey, '"')
        },
        Tile::Pool => TileInfo{
            draw_info: draw_info!(Color::DarkGrey, '0')
        },
        Tile::Road => TileInfo{
            draw_info: draw_info!(Color::DarkGrey, '.')
        },
        Tile::StairDown => TileInfo{
            draw_info: draw_info!(Color::DarkGrey, '>')
        },
        Tile::StairUp => TileInfo{
            draw_info: draw_info!(Color::DarkGrey, '<')
        },
        Tile::Statue => TileInfo{
            draw_info: draw_info!(Color::DarkGrey, '&')
        },
        Tile::Swamp => TileInfo{
            draw_info: draw_info!(Color::DarkGrey, '"')
        },
        Tile::Tombstone => TileInfo{
            draw_info: draw_info!(Color::DarkGrey, '+')
        },
        Tile::TrapKnown => TileInfo{
            draw_info: draw_info!(Color::DarkGrey, '^')
        },
        Tile::Tree => TileInfo{
            draw_info: draw_info!(Color::DarkGrey, 'T')
        },
        Tile::Tunnel => TileInfo{
            draw_info: draw_info!(Color::DarkGrey, '.')
        },
        Tile::Wall => TileInfo{
            draw_info: draw_info!(Color::DarkGrey, '#')
        },
        Tile::Water => TileInfo{
            draw_info: draw_info!(Color::DarkGrey, '=')
        },
        Tile::Web => TileInfo{
            draw_info: draw_info!(Color::DarkGrey, '|')
        },
    };
    return result;
}
