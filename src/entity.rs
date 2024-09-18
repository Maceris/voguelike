use crossterm::style::Color;

use crate::{action::{Action, Actor}, map::Location, stub_actor, tabletop::Race, terminal_util::Drawable};

pub struct Entity {
    pub id: EntityID,
    pub race: Race,
    pub pos_x: u16,
    pub pos_y: u16
}

pub struct North;
pub struct East;
pub struct South;
pub struct West;
pub struct NorthEast;
pub struct NorthWest;
pub struct SouthEast;
pub struct SouthWest;
pub struct Up;
pub struct Down;

pub type EntityID = u64;

pub const ID_NO_ENTITY: EntityID = 0;
pub const ID_WORLD: EntityID = 1;
pub const ID_PLAYER: EntityID = 2;

impl Location for Entity {
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

impl Drawable for Entity {
    fn get_color(&self) -> Color {
        return match self.race {
            Race::Dragonborn => Color::White,
            Race::Dwarf => Color::White,
            Race::Elf => Color::White,
            Race::Gnome => Color::White,
            Race::HalfElf => Color::White,
            Race::HalfOrc => Color::White,
            Race::Halfling => Color::White,
            Race::Human => Color::White,
            Race::Tiefling => Color::White,
        };
    }
    fn get_icon(&self) -> char {
        return match self.race {
            Race::Dragonborn => '@',
            Race::Dwarf => '@',
            Race::Elf => '@',
            Race::Gnome => '@',
            Race::HalfElf => '@',
            Race::HalfOrc => '@',
            Race::Halfling => '@',
            Race::Human => '@',
            Race::Tiefling => '@',
        };
    }
}


impl Actor for Entity {
    stub_actor!();
}

impl Actor for North {
    stub_actor!();
}
impl Actor for East {
    stub_actor!();
}
impl Actor for South {
    stub_actor!();
}
impl Actor for West {
    stub_actor!();
}
impl Actor for NorthEast {
    stub_actor!();
}
impl Actor for NorthWest {
    stub_actor!();
}
impl Actor for SouthEast {
    stub_actor!();
}
impl Actor for SouthWest {
    stub_actor!();
}
impl Actor for Up {
    stub_actor!();
}
impl Actor for Down {
    stub_actor!();
}
