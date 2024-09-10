use crate::entity::Player;

pub struct Map {
    pub player: Player
}

pub trait Location {
    fn get_x(&self) -> u16;
    fn get_y(&self) -> u16;
}
