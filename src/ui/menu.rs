use crate::tabletop::{Alignment, Class, Race, Stats};

pub struct Dropdown {
    pub choices: Vec<String>,
    pub editing: bool,
    pub selected_item: u8,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MenuType {
    Character,
    Main,
    NewCharacter,
    Pause,
    TestMenu,
}

pub struct TabMenu {
    pub tabs: Vec<String>,
    pub selected_tab: u8, 
}

pub struct Table {
    pub headers: Vec<String>,
    pub rows: Vec<TableRow>,
}

pub struct TableRow {
    pub values: Vec<String>,
}

pub struct CharacterCreation {
    pub alignment: Option<Alignment>,
    pub class: Option<Class>,
    pub name: String,
    pub race: Option<Race>,
    pub stat_points: u8,
    pub stats: Stats,
}

impl CharacterCreation {
    pub fn new() -> Self {
        Self {
            alignment: None,
            class: None,
            name: String::new(),
            race: None,
            stat_points: 27,
            stats: Stats {
                charisma: 8,
                constitution: 8,
                dexterity: 8,
                intelligence: 8,
                strength: 8,
                wisdom: 8
            },
        }
    }
}
