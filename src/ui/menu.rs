use strum::IntoEnumIterator;

use crate::{constants::{self, NAME_MAX_LENGTH}, tabletop::{self, Alignment, Class, Race, Stats}, ui::menu_focus};

use super::menu_focus::{new_character, test_window, FocusIndex, FocusTracking};

pub struct Dropdown {
    pub choices: Vec<String>,
    pub editing: bool,
    pub label: String,
    pub selected_item: usize,
    pub size: usize,
}

impl Dropdown {
    pub fn new(label:String, choices: Vec<String>) -> Self {
        let mut result = Self {
            choices,
            editing: false,
            label,
            selected_item: 0,
            size: 0,
        };
        result.recalculate_size();
        return result;
    }

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

pub enum MenuItem {
    Dropdown(Dropdown),
    PointBuy(PointBuy),
    TextField(TextField),
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
    fn navigate_menu_left(&mut self);
    fn navigate_menu_out(&mut self);
    fn navigate_menu_right(&mut self);
    fn navigate_menu_up(&mut self);
}

pub struct MenuData {
    pub new_character: NewCharacter,
    pub test_menu: TestMenu,
}

impl MenuData {
    pub fn new() -> Self {
        Self {
            new_character: NewCharacter::new(),
            test_menu: TestMenu::new(),
        }
    }
}


pub struct NewCharacter {
    focus_index: FocusIndex,
    pub items: Vec<MenuItem>,
    
}

impl NewCharacter {
    
    pub fn new() -> Self {
        let name = TextField::new(String::from("Name"), constants::NAME_MAX_LENGTH);
        let class = Dropdown::new(String::from("Class"), Class::iter().map(|val| val.to_string()).collect());
        let race = Dropdown::new(String::from("Race"), Race::iter().map(|val| val.to_string()).collect());
        let alignment = Dropdown::new(String::from("Alignment"), Alignment::iter().map(|val| val.to_string()).collect());
        let stats = PointBuy::new();
        
        let mut result = Self {
            focus_index: 0,
            items: Vec::with_capacity(new_character::FOCUS_INDEX_SIZE),
        };

        result.items.push(MenuItem::TextField(name));
        result.items.push(MenuItem::Dropdown(class));
        result.items.push(MenuItem::Dropdown(race));
        result.items.push(MenuItem::Dropdown(alignment));
        result.items.push(MenuItem::PointBuy(stats));

        return result;
    }

    pub fn get_focus_index(&self) -> FocusIndex {
        self.focus_index
    }

    pub fn get_currently_selected_element(&self) -> &MenuItem {
        self.items.get(self.focus_index as usize).unwrap()
    }
}

impl FocusTracking for NewCharacter {
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

impl MenuNavigation for NewCharacter {
    fn navigate_menu_down(&mut self) {
        //TODO(ches) implement this
    }
    
    fn navigate_menu_in(&mut self) {
        //TODO(ches) implement this
    }

    fn navigate_menu_left(&mut self) {
        //TODO(ches) implement this
    }

    fn navigate_menu_out(&mut self) {
        //TODO(ches) implement this
    }

    fn navigate_menu_right(&mut self) {
        //TODO(ches) implement this
    }

    fn navigate_menu_up(&mut self) {
        //TODO(ches) implement this
    }
}

pub struct PointBuy {
    pub internal_focus: FocusIndex,
    pub stat_points: u8,
    pub stats: Stats,
}

impl PointBuy {
    pub fn new() -> Self {
        Self {
            internal_focus: 0,
            stat_points: tabletop::POINT_BUY_POINTS,
            stats: Stats {
                charisma: tabletop::POINT_BUY_MIN,
                constitution: tabletop::POINT_BUY_MIN,
                dexterity: tabletop::POINT_BUY_MIN,
                intelligence: tabletop::POINT_BUY_MIN,
                strength: tabletop::POINT_BUY_MIN,
                wisdom: tabletop::POINT_BUY_MIN
            },
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
    pub label: String,
    pub max_length: u16,
    pub value: String,
}

impl TextField {
    pub fn new(label: String, max_length: u16) -> Self {
        Self {
            editing: false,
            label,
            max_length,
            value: String::with_capacity(max_length as usize)
        }
    }
}

pub struct TestMenu {
    focus_index: FocusIndex,
    pub items: Vec<MenuItem>,
}

impl TestMenu {
    pub fn new() -> Self {

        let dropdown = Dropdown::new("Dropdown".to_string(),  vec![
            "Foo".to_string(), 
            "Bar".to_string(),
            "Double Foo".to_string(),
            "Clown Car".to_string(),
            "Warlock".to_string(),
            "Ternary".to_string(),
            "Baz".to_string(),
            "Neptune".to_string(),
            "Green".to_string()
        ]);

        let text_field = TextField::new(String::from("Player Name"), constants::NAME_MAX_LENGTH);

        let point_buy = PointBuy::new();

        let mut result = Self {
            focus_index: 0,
            items: Vec::with_capacity(test_window::FOCUS_INDEX_SIZE),
        };

        result.items.push(MenuItem::Dropdown(dropdown));
        result.items.push(MenuItem::TextField(text_field));
        result.items.push(MenuItem::PointBuy(point_buy));

        return result;
    }
    
    pub fn editing_anything(&self) -> bool {
        for index in 0..self.items.len() {
            let editing = match self.items.get(index).unwrap() {
                MenuItem::Dropdown(dropdown) => dropdown.editing,
                MenuItem::PointBuy(_) => false,
                MenuItem::TextField(text_field) => text_field.editing,
            };
            if editing {
                return true;
            }
        }
        return false;
    }

    pub fn get_focus_index(&self) -> FocusIndex {
        self.focus_index
    }

    pub fn get_currently_selected_element(&self) -> &MenuItem {
        self.items.get(self.focus_index as usize).unwrap()
    }

    pub fn get_currently_selected_element_mut(&mut self) -> &mut MenuItem {
        self.items.get_mut(self.focus_index as usize).unwrap()
    }

}

impl FocusTracking for TestMenu {
    fn get_current_focus_index(&self) -> FocusIndex {
        return self.focus_index;
    }
    
    fn get_max_focus_index(&self) -> FocusIndex {
        return self.items.len() as FocusIndex - 1;
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
        let item: &mut MenuItem = self.items.get_mut(self.focus_index as usize).unwrap();

        let handled:bool = match item {
            MenuItem::Dropdown(dropdown) => {
                if dropdown.editing {
                    if dropdown.selected_item < dropdown.choices.len() {
                        dropdown.selected_item += 1;
                    }
                }
                dropdown.editing
            },
            MenuItem::PointBuy(point_buy) => {
                let mut result = false;
                if point_buy.internal_focus < tabletop::NUMBER_OF_STATS as u16 - 1 {
                    point_buy.internal_focus += 1;
                    result = true;
                }
                result
            },
            MenuItem::TextField(text_field) => text_field.editing,
        };
        if !handled {
            self.next_focus();
        }
    }
    
    fn navigate_menu_in(&mut self) {
        let item: &mut MenuItem = self.items.get_mut(self.focus_index as usize).unwrap();

        match item {
            MenuItem::Dropdown(dropdown) => dropdown.editing = !dropdown.editing,
            MenuItem::PointBuy(_) => (),
            MenuItem::TextField(text_field) => text_field.editing = !text_field.editing,
        };
    }

    fn navigate_menu_left(&mut self) {
        let item: &mut MenuItem = self.items.get_mut(self.focus_index as usize).unwrap();

        match item {
            MenuItem::PointBuy(point_buy) => {
                let mut fallback: u8 = 0;
                let stat: &mut u8 = match point_buy.internal_focus {
                    0 => &mut point_buy.stats.charisma,
                    1 => &mut point_buy.stats.constitution,
                    2 => &mut point_buy.stats.dexterity,
                    3 => &mut point_buy.stats.intelligence,
                    4 => &mut point_buy.stats.strength,
                    5 => &mut point_buy.stats.wisdom,
                    _ => &mut fallback
                };
                tabletop::point_buy_attempt_decrease(stat, &mut point_buy.stat_points);
            },
            _ => ()
        };
    }
    
    fn navigate_menu_out(&mut self) {
        let item: &mut MenuItem = self.items.get_mut(self.focus_index as usize).unwrap();

        match item {
            MenuItem::Dropdown(dropdown) => dropdown.editing = false,
            MenuItem::PointBuy(_) => (),
            MenuItem::TextField(text_field) => text_field.editing = false,
        };
    }

    fn navigate_menu_right(&mut self) {
        let item: &mut MenuItem = self.items.get_mut(self.focus_index as usize).unwrap();

        match item {
            MenuItem::PointBuy(point_buy) => {
                let mut fallback: u8 = 0;
                let stat: &mut u8 = match point_buy.internal_focus {
                    0 => &mut point_buy.stats.charisma,
                    1 => &mut point_buy.stats.constitution,
                    2 => &mut point_buy.stats.dexterity,
                    3 => &mut point_buy.stats.intelligence,
                    4 => &mut point_buy.stats.strength,
                    5 => &mut point_buy.stats.wisdom,
                    _ => &mut fallback
                };
                tabletop::point_buy_attempt_increase(stat, &mut point_buy.stat_points);
            },
            _ => ()
        };
    }

    fn navigate_menu_up(&mut self) {
        let item: &mut MenuItem = self.items.get_mut(self.focus_index as usize).unwrap();

        let handled:bool = match item {
            MenuItem::Dropdown(dropdown) => {
                if dropdown.editing {
                    if dropdown.selected_item > 0 {
                        dropdown.selected_item -= 1;
                    }
                }
                dropdown.editing
            },
            MenuItem::PointBuy(point_buy) => {
                let mut result = false;
                if point_buy.internal_focus > 0 {
                    point_buy.internal_focus -= 1;
                    result = true;
                }
                result
            },
            MenuItem::TextField(text_field) => text_field.editing,
        };
        if !handled {
            self.previous_focus();
        }
    }
}