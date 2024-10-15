pub type FocusIndex = u16;

pub mod new_character {
    pub const NAME: usize = 0;
    pub const CLASS: usize = 1;
    pub const RACE: usize = 2;
    pub const ALIGNMENT: usize = 3;
    pub const STATS: usize = 4;

    pub const FOCUS_INDEX_SIZE: usize = 5;
}

pub mod test_window {
    pub const DROPDOWN: usize = 0;
    pub const TEXT_FIELD: usize = 1;
    pub const POINT_BUY: usize = 2;

    pub const FOCUS_INDEX_SIZE: usize = 3;

}