pub type FocusIndex = u16;

pub trait FocusTracking {
    fn get_current_focus_index(&self) -> FocusIndex;
    fn get_max_focus_index(&self) -> FocusIndex;
    fn next_focus(&mut self);
    fn previous_focus(&mut self);
    fn wraps_focus() -> bool;
}

pub mod test_window {
    pub const DROPDOWN: usize = 0;
    pub const TEXT_FIELD: usize = 1; 
}