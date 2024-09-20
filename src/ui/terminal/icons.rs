use crossterm::style::Color;

use crate::tabletop::Race;

pub fn creature_color(race: &Race) -> Color {
    match race {
        Race::Dragonborn => Color::White,
        Race::Dwarf => Color::White,
        Race::Elf => Color::White,
        Race::Gnome => Color::White,
        Race::HalfElf => Color::White,
        Race::HalfOrc => Color::White,
        Race::Halfling => Color::White,
        Race::Human => Color::White,
        Race::Tiefling => Color::White,
    }
}

pub fn creature_icon(race: &Race) -> char {
    match race {
        Race::Dragonborn => '@',
        Race::Dwarf => '@',
        Race::Elf => '@',
        Race::Gnome => '@',
        Race::HalfElf => '@',
        Race::HalfOrc => '@',
        Race::Halfling => '@',
        Race::Human => '@',
        Race::Tiefling => '@',
    }
}
