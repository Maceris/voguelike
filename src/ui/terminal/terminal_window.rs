use crossterm::style::Color;

pub enum Border {
    None,
    Thin,
    Thick
}

pub struct Text<'a> {
    pub color: Color,
    pub text: &'a str
}

pub struct Option<'a> {
    pub text: Text<'a>,
    pub keys: Vec<char>
}

pub struct Window {
    pub width: u16,
    pub height: u16,
    pub x: u16,
    pub y: u16,
    pub z: u16,
    pub border: Border
}

pub struct Menu<'a> {
    pub options: Vec<Text<'a>>,
    pub current_selection: u16
}

pub struct Dialog<'a> {
    pub title: &'a str,
    pub contents: Vec<Text<'a>>
}
