use crossterm::style::Color;

use crate::{action::{Action, Actor}, create_drawable, map::Location, terminal_util::Drawable};

pub struct Species;

pub struct Entity;

pub type EntityID = u128;

pub const ID_NO_ENTITY: EntityID = 0;
pub const ID_WORLD: EntityID = 1;
pub const ID_PLAYER: EntityID = 2;

pub struct Player {
    pub id: EntityID,
    pub pos_x: u16,
    pub pos_y: u16
}

impl Location for Player {
    fn get_x(&self) -> u16 {
        return self.pos_x;
    }

    fn get_y(&self) -> u16 {
        return self.pos_y;
    }
    
    fn set_x(&mut self, x: u16) {
        self.pos_x = x;
    }
    
    fn set_y(&mut self, y: u16) {
        self.pos_y = y;
    }
    
}

create_drawable!(Player, Color::White, '@');

pub struct Monster {
    species: Species
}

impl Actor for Entity {
    fn before(&self, action: Action) -> bool {
        match action {
            Action::Examine(examine) => {},
            _ => {return false;}
        }
        return false;
    }
    fn after(&self, _action: Action) -> bool {
        return false;
    }
    fn react_before(&self, _action: Action) -> bool {
        return false;
    }
    fn react_after(&self, _action: Action) -> bool {
        return false;
    }
}


