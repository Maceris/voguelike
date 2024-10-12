
pub struct Offset {
    pub x: u16,
    pub y: u16,
}

impl Offset {
    pub const fn new(x: u16, y: u16) -> Self {
        Self { x, y }
    }
}

pub mod test_window {
    use super::Offset;

    pub const DROPDOWN: Offset = Offset::new(2, 2);
    pub const TEXT_FIELD: Offset = Offset::new(2, 3);
    pub const POINT_BUY: Offset = Offset::new(0, 4);
}