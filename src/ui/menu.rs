use crate::tabletop::{Alignment, Class, Race, Stats};

pub struct Dropdown {
    pub choices: Vec<String>,
    pub editing: bool,
    pub selected_item: usize,
    pub size: usize,
}

impl Dropdown {
    pub fn recalculate_size(&mut self) {
        let mut max: usize = 0; 
        for str in self.choices.iter() {
            let len: usize = str.len();
            if len > max {
                max = len;
            }
        }
        self.size = max;
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MenuType {
    Character,
    Main,
    NewCharacter,
    Pause,
    TestMenu,
}

pub struct MenuData {
    pub character_creation: CharacterCreation,
    pub test_menu: TestMenu,
}

impl MenuData {
    pub fn new() -> Self {
        Self {
            character_creation: CharacterCreation::new(),
            test_menu: TestMenu::new(),
        }
    }
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

pub struct TestMenu {
    pub dropdown: Dropdown,
}

impl TestMenu {
    pub fn new() -> Self {
        let mut result = Self {
            dropdown: Dropdown {
                choices: vec!["Foo".to_string(), 
                    "Bar".to_string(),
                    "Double Foo".to_string(),
                    "Clown Car".to_string(),
                    "Warlock".to_string(),
                    "Ternary".to_string(),
                    "Baz".to_string(),
                    "Neptune".to_string(),
                    "Green".to_string()
                ],
                editing: false,
                selected_item: 0,
                size: 0,
            }
        };
        result.dropdown.recalculate_size();
        return result;
    }
}
