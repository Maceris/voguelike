pub type FocusIndex = u16;

pub trait FocusTracking {
    fn get_current_focus_index(&self) -> FocusIndex;
    fn get_max_focus_index(&self) -> FocusIndex;
    fn next_focus(&mut self);
    fn previous_focus(&mut self);
    fn wraps_focus() -> bool;
}

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