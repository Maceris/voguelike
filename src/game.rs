use crate::{entity::Player, map::{GameMap, TileMap}};

pub enum GameState {
    Menu,
    Paused,
    Running
}

pub struct Game {
    pub state: GameState,
    pub player: Player,
    pub current_map: Option<Box<GameMap>>,
    pub tile_map: TileMap,
}