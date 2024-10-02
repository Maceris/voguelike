use crate::{constants, tabletop::{Alignment, Class, Race, Stats}};

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

impl Focusable for Dropdown {
    fn get_focus_index(&self) -> FocusIndex {
        return self.focus_index;
    }
}

pub trait MenuItem {
    fn get_focus_index(&self) -> FocusIndex;
}

pub trait Focusable {
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

impl Focusable for TextField {
    fn get_focus_index(&self) -> FocusIndex {
        return self.focus_index;
    }
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
                value: String::new(),
            }
        };
        result.dropdown.recalculate_size();
        return result;
    }
}

impl FocusTracking for TestMenu {
    fn get_current_focus_index(&self) -> u16 {
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
    }
    
    fn navigate_menu_out(&mut self) {
        if self.focus_index == self.dropdown.get_focus_index() {
            self.dropdown.editing = false;
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