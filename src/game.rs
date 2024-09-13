use ringbuffer::AllocRingBuffer;

use crate::{entity::Player, map::{GameMap, TileMap}, material::MaterialMap, tag::TagMap};

pub enum GameState {
    Menu,
    Paused,
    Running
}

pub struct DebugInfo {
    pub fps_history: AllocRingBuffer<u32>
}

pub struct DataTables {
    pub tile_map: TileMap,
    pub tag_map: TagMap,
    pub material_map: MaterialMap,
}

pub struct Game {
    pub state: GameState,
    pub player: Player,
    pub current_map: Option<Box<GameMap>>,
    pub data_tables: DataTables,
    pub debug_info: DebugInfo
}