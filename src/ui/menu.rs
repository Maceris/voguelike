use crate::{constants::{self, NAME_MAX_LENGTH}, tabletop::Stats};

type FocusIndex = u16;

pub struct Dropdown {
    pub choices: Vec<String>,
    pub editing: bool,
    focus_index: FocusIndex,
    pub label: String,
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

impl MenuItem for Dropdown {
    fn get_focus_index(&self) -> FocusIndex {
        return self.focus_index;
    }
}

pub trait MenuItem {
    fn get_focus_index(&self) -> FocusIndex;
}

pub trait FocusTracking {
    fn get_current_focus_index(&self) -> FocusIndex;
    fn get_max_focus_index(&self) -> FocusIndex;
    fn next_focus(&mut self);
    fn previous_focus(&mut self);
    fn wraps_focus() -> bool;
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MenuType {
    Character,
    Main,
    NewCharacter,
    Pause,
    TestMenu,
}

pub trait MenuNavigation {
    fn navigate_menu_down(&mut self);
    fn navigate_menu_in(&mut self);
    fn navigate_menu_out(&mut self);
    fn navigate_menu_up(&mut self);
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

pub struct TextField {
    pub editing: bool,
    focus_index: FocusIndex,
    pub label: String,
    pub max_length: u16,
    pub value: String,
}

impl MenuItem for TextField {
    fn get_focus_index(&self) -> FocusIndex {
        return self.focus_index;
    }
}

pub struct CharacterCreation {
    pub alignment: Dropdown,
    pub class: Dropdown,
    pub name: TextField,
    pub race: Dropdown,
    pub stat_points: u8,
    pub stats: Stats,
}

impl CharacterCreation {
    pub fn new() -> Self {
        Self {
            alignment: Dropdown {
                choices: vec![
                    "Lawful Good".to_string(),
                    "Neutral Good".to_string(),
                    "Chaotic Good".to_string(),
                    "Lawful Neutral".to_string(),
                    "Neutral".to_string(),
                    "Chaotic Neutral".to_string(),
                    "Lawful Evil".to_string(),
                    "Neutral Evil".to_string(),
                    "Chaotic Evil".to_string(), 
                ],
                editing: false,
                focus_index: 3,
                label: "Alignment".to_string(),
                selected_item: 0,
                size: 0,
            },
            class: Dropdown {
                choices: vec![
                    "Barbarian".to_string(),
                    "Bard".to_string(),
                    "Cleric".to_string(),
                    "Druid".to_string(),
                    "Fighter".to_string(),
                    "Monk".to_string(),
                    "Paladin".to_string(),
                    "Ranger".to_string(),
                    "Rogue".to_string(),
                    "Sorcerer".to_string(),
                    "Warlock".to_string(),
                    "Wizard".to_string(),
                ],
                editing: false,
                focus_index: 1,
                label: "Class".to_string(),
                selected_item: 0,
                size: 0,
            },
            name: TextField {
                editing: false,
                focus_index: 0,
                label: "Name".to_string(),
                max_length: constants::NAME_MAX_LENGTH,
                value: String::with_capacity(NAME_MAX_LENGTH as usize),
            },
            race: Dropdown {
                choices: vec![
                    "Dragonborn".to_string(),
                    "Dwarf".to_string(),
                    "Elf".to_string(),
                    "Gnome".to_string(),
                    "HalfElf".to_string(),
                    "HalfOrc".to_string(),
                    "Halfling".to_string(),
                    "Human".to_string(),
                    "Tiefling".to_string(),
                ],
                editing: false,
                focus_index: 2,
                label: "Race".to_string(),
                selected_item: 0,
                size: 0,
            },
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

impl FocusTracking for CharacterCreation {
    fn get_current_focus_index(&self) -> FocusIndex {
        //TODO(ches) implement this.
        return 0;
    }

    fn get_max_focus_index(&self) -> FocusIndex {
        //TODO(ches) implement this.
        return 0;
    }

    fn next_focus(&mut self) {
        //TODO(ches) implement this.
    }

    fn previous_focus(&mut self) {
        //TODO(ches) implement this.
    }

    fn wraps_focus() -> bool {
        return false;
    }
}

impl MenuNavigation for CharacterCreation {
    fn navigate_menu_down(&mut self) {
        //TODO(ches) implement this
    }
    
    fn navigate_menu_in(&mut self) {
        //TODO(ches) implement this
    }

    fn navigate_menu_out(&mut self) {
        //TODO(ches) implement this
    }

    fn navigate_menu_up(&mut self) {
        //TODO(ches) implement this
    }
}

pub struct TestMenu {
    pub dropdown: Dropdown,
    focus_index: u16,
    pub text_field: TextField,
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
                focus_index: 0,
                label: "Dropdown".to_string(),
                selected_item: 0,
                size: 0,
            },
            focus_index: 0,
            text_field: TextField {
                editing: false,
                focus_index: 1,
                label: "Player Name".to_string(),
                max_length: constants::NAME_MAX_LENGTH,
                value: String::with_capacity(NAME_MAX_LENGTH as usize),
            }
        };
        result.dropdown.recalculate_size();
        return result;
    }
    
    pub fn editing_anything(&self) -> bool {
        // TODO(ches) Generalize this
        return self.dropdown.editing 
            || self.text_field.editing;
    }
}

impl FocusTracking for TestMenu {
    fn get_current_focus_index(&self) -> FocusIndex {
        return self.focus_index;
    }
    
    fn get_max_focus_index(&self) -> FocusIndex {
        return 1;   
    }
    
    fn next_focus(&mut self) {
        if self.get_current_focus_index() < self.get_max_focus_index() {
            self.focus_index += 1;
        }
        else if TestMenu::wraps_focus() && self.get_current_focus_index() >= self.get_max_focus_index() {
            self.focus_index = 0;
        }
    }
    
    fn previous_focus(&mut self) {
        if self.get_current_focus_index() > 0 {
            self.focus_index -= 1;
        }
        else if TestMenu::wraps_focus() && self.get_current_focus_index() == 0 {
            self.focus_index = self.get_max_focus_index();
        }
    }

    fn wraps_focus() -> bool {
        return false;
    }
}

impl MenuNavigation for TestMenu {
    fn navigate_menu_down(&mut self) {
        if self.focus_index == self.dropdown.get_focus_index() && self.dropdown.editing {
            if self.dropdown.selected_item < self.dropdown.choices.len() - 1 {
                self.dropdown.selected_item += 1;
            }
        }
        else {
            self.next_focus();
        }
    }
    
    fn navigate_menu_in(&mut self) {
        if self.focus_index == self.dropdown.get_focus_index() {
            self.dropdown.editing = !self.dropdown.editing;
        }
        else if self.focus_index == self.text_field.get_focus_index() {
            self.text_field.editing = !self.text_field.editing;
        }
    }
    
    fn navigate_menu_out(&mut self) {
        if self.focus_index == self.dropdown.get_focus_index() {
            self.dropdown.editing = false;
        }
        else if self.focus_index == self.text_field.get_focus_index() {
            self.text_field.editing = false;
        }
    }

    fn navigate_menu_up(&mut self) {
        if self.focus_index == self.dropdown.get_focus_index() && self.dropdown.editing {
            if self.dropdown.selected_item > 0 {
                self.dropdown.selected_item -= 1;
            }
        }
        else {
            self.previous_focus();
        }
    }
}