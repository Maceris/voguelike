use crossterm::style::Color;

use crate::{map::Tile, tabletop::Race};

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

pub fn tile_color(tile: &Tile) -> Color {
    match tile {
        Tile::Air => Color::DarkGrey,
        Tile::Altar => Color::DarkGrey,
        Tile::Building => Color::DarkGrey,
        Tile::DoorClosed => Color::DarkGrey,
        Tile::DoorOpen => Color::DarkGrey,
        Tile::Entrance => Color::DarkGrey,
        Tile::Floor => Color::DarkGrey,
        Tile::Forest => Color::DarkGrey,
        Tile::Forge => Color::DarkGrey,
        Tile::Gate => Color::DarkGrey,
        Tile::Graveyard => Color::DarkGrey,
        Tile::Herbs => Color::DarkGrey,
        Tile::Hills => Color::DarkGrey,
        Tile::Hive => Color::DarkGrey,
        Tile::Hole => Color::DarkGrey,
        Tile::Lever => Color::DarkGrey,
        Tile::Magma => Color::DarkGrey,
        Tile::Mountain => Color::DarkGrey,
        Tile::Passage => Color::DarkGrey,
        Tile::Plains => Color::DarkGrey,
        Tile::Pool => Color::DarkGrey,
        Tile::Road => Color::DarkGrey,
        Tile::StairDown => Color::DarkGrey,
        Tile::StairUp => Color::DarkGrey,
        Tile::Statue => Color::DarkGrey,
        Tile::Swamp => Color::DarkGrey,
        Tile::Tombstone => Color::DarkGrey,
        Tile::TrapKnown => Color::DarkGrey,
        Tile::Tree => Color::DarkGrey,
        Tile::Tunnel => Color::DarkGrey,
        Tile::Wall => Color::DarkGrey,
        Tile::Water => Color::DarkGrey,
        Tile::Web => Color::DarkGrey,
    }
}

pub fn tile_icon(tile: &Tile) -> char {
    match tile {
        Tile::Air => ' ',
        Tile::Altar => '_',
        Tile::Building => 'o',
        Tile::DoorClosed => '+',
        Tile::DoorOpen => '/',
        Tile::Entrance => '*',
        Tile::Floor => '.',
        Tile::Forest => '&',
        Tile::Forge => '&',
        Tile::Gate => '8',
        Tile::Graveyard => '+',
        Tile::Herbs => '"',
        Tile::Hills => '~',
        Tile::Hive => '0',
        Tile::Hole => '*',
        Tile::Lever => '!',
        Tile::Magma => '=',
        Tile::Mountain => '^',
        Tile::Passage => '.',
        Tile::Plains => '"',
        Tile::Pool => '0',
        Tile::Road => '.',
        Tile::StairDown => '>',
        Tile::StairUp => '<',
        Tile::Statue => '&',
        Tile::Swamp => '"',
        Tile::Tombstone => '+',
        Tile::TrapKnown => '^',
        Tile::Tree => 'T',
        Tile::Tunnel => '.',
        Tile::Wall => '#',
        Tile::Water => '=',
        Tile::Web => '|',
    }
}

