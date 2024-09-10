use crate::entity::Player;

pub struct Map {
    pub player: Player,
    pub width: u16,
    pub height: u16
}

pub trait Location {
    fn get_x(&self) -> u16;
    fn get_y(&self) -> u16;
    fn set_x(&mut self, x: u16);
    fn set_y(&mut self, y: u16);
}
