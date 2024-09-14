use crossterm::style::Color;

use crate::{action::{Action, Actor, Examine}, create_drawable, map::Location, terminal_util::Drawable};

pub struct Species;

pub struct Entity;

pub struct Player {
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
    fn before(action: Action) -> bool {
        match action {
            Action::Examine(examine) => {},
            _ => {return true;}
        }
        return true;
    }
    fn after(action: Action) -> bool {
        return true;
    }
    fn react_before(action: Action) -> bool {
        return true;
    }
    fn react_after(action: Action) -> bool {
        return true;
    }
}
