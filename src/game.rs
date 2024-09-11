use crate::{entity::Player, map::TileMap};

pub enum GameState {
    Menu,
    Paused,
    Running
}

pub struct Game {
    pub state: GameState,
    pub player: Player,
    pub tile_map: TileMap,
}