use std::collections::VecDeque;

use ringbuffer::AllocRingBuffer;

use crate::{action::ActionRequest, component::Components, entity::Entity, map::{GameMap, TileMap}, material::MaterialMap, tag::TagMap};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GameState {
    Menu,
    Paused,
    Running,
    QuitRequested
}

pub struct DebugInfo {
    pub fps_history: AllocRingBuffer<u32>
}

pub struct DataTables {
    pub material_map: MaterialMap,
    pub tag_map: TagMap,
    pub tile_map: TileMap,
}

pub struct Game {
    pub action_queue: VecDeque<ActionRequest>,
    pub components: Components,
    pub current_map: Box<GameMap>,
    pub data_tables: DataTables,
    pub debug_info: DebugInfo,
    pub player: Entity,
    pub state: GameState,
}