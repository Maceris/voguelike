use crate::ui::menu::MenuType;

pub struct Offset {
    pub x: u16,
    pub y: u16,
}

impl Offset {
    pub const fn new(x: u16, y: u16) -> Self {
        Self { x, y }
    }
}

pub const UNKNOWN_OFFSET: Offset = Offset::new(0, 0);

pub mod new_character {
    use crate::ui::menu_focus;
    use super::{Offset, UNKNOWN_OFFSET};

    pub fn get_offset(index: usize) -> Offset {
        match index {
            menu_focus::new_character::NAME => NAME,
            menu_focus::new_character::CLASS => CLASS,
            menu_focus::new_character::RACE => RACE,
            menu_focus::new_character::ALIGNMENT => ALIGNMENT,
            menu_focus::new_character::STATS => STATS,
            _ => UNKNOWN_OFFSET
        }
    }

    pub const NAME: Offset = Offset::new(2, 1);
    pub const CLASS: Offset = Offset::new(2, 2);
    pub const RACE: Offset = Offset::new(2, 3);
    pub const ALIGNMENT: Offset = Offset::new(2, 4);
    pub const STATS: Offset = Offset::new(2, 5);
}

pub mod test_window {
    use crate::ui::menu_focus;
    use super::{Offset, UNKNOWN_OFFSET};

    pub fn get_offset(index: usize) -> Offset {
        match index {
            menu_focus::test_window::DROPDOWN => DROPDOWN,
            menu_focus::test_window::TEXT_FIELD => TEXT_FIELD,
            menu_focus::test_window::POINT_BUY => POINT_BUY,
            _ => UNKNOWN_OFFSET
        }
    }

    pub const DROPDOWN: Offset = Offset::new(2, 2);
    pub const TEXT_FIELD: Offset = Offset::new(2, 3);
    pub const POINT_BUY: Offset = Offset::new(2, 4);
}

pub fn get_offset(menu_type: MenuType, index: usize) -> Offset {
    match menu_type {
        MenuType::Character => UNKNOWN_OFFSET,
        MenuType::Main => UNKNOWN_OFFSET,
        MenuType::NewCharacter => new_character::get_offset(index),
        MenuType::Pause => UNKNOWN_OFFSET,
        MenuType::TestMenu => test_window::get_offset(index),
    }
}
