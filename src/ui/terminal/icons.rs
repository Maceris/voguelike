use crossterm::style::Color;

use crate::{item::{Item, ItemType}, map::Tile, tabletop::Race};

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

pub fn item_icon(item: &Item) -> char {
    match item.item_type {
        ItemType::Abacus => '[', //TODO(ches) set the actual icon
        ItemType::Amulet => '[', //TODO(ches) set the actual icon
        ItemType::Arrow => '[', //TODO(ches) set the actual icon
        ItemType::ArrowBundle => '[', //TODO(ches) set the actual icon
        ItemType::Backpack => '[', //TODO(ches) set the actual icon
        ItemType::Bagpipes => '[', //TODO(ches) set the actual icon
        ItemType::BallBearing => '[', //TODO(ches) set the actual icon
        ItemType::BallBearingBundle => '[', //TODO(ches) set the actual icon
        ItemType::Barrel => '[', //TODO(ches) set the actual icon
        ItemType::Basket => '[', //TODO(ches) set the actual icon
        ItemType::BatteringRam => '[', //TODO(ches) set the actual icon
        ItemType::Battleaxe => '[', //TODO(ches) set the actual icon
        ItemType::Bedroll => '[', //TODO(ches) set the actual icon
        ItemType::Bell => '[', //TODO(ches) set the actual icon
        ItemType::Blanket => '[', //TODO(ches) set the actual icon
        ItemType::BlockAndTackle => '[', //TODO(ches) set the actual icon
        ItemType::Blowgun => '[', //TODO(ches) set the actual icon
        ItemType::BlowgunNeedle => '[', //TODO(ches) set the actual icon
        ItemType::BlowgunNeedleBundle => '[', //TODO(ches) set the actual icon
        ItemType::Book => '[', //TODO(ches) set the actual icon
        ItemType::Bottle => '[', //TODO(ches) set the actual icon
        ItemType::BreastplateArmor => '[', //TODO(ches) set the actual icon
        ItemType::Bucket => '[', //TODO(ches) set the actual icon
        ItemType::Caltrop => '[', //TODO(ches) set the actual icon
        ItemType::CaltropBundle => '[', //TODO(ches) set the actual icon
        ItemType::Candle => '[', //TODO(ches) set the actual icon
        ItemType::Chain => '[', //TODO(ches) set the actual icon
        ItemType::ChainMailArmor => '[', //TODO(ches) set the actual icon
        ItemType::ChainShirtArmor => '[', //TODO(ches) set the actual icon
        ItemType::Chalk => '[', //TODO(ches) set the actual icon
        ItemType::Chest => '[', //TODO(ches) set the actual icon
        ItemType::ClimbersKit => '[', //TODO(ches) set the actual icon
        ItemType::ClothesCommon => '[', //TODO(ches) set the actual icon
        ItemType::ClothesCostume => '[', //TODO(ches) set the actual icon
        ItemType::ClothesFine => '[', //TODO(ches) set the actual icon
        ItemType::ClothesTraveler => '[', //TODO(ches) set the actual icon
        ItemType::Club => '[', //TODO(ches) set the actual icon
        ItemType::Coin => '[', //TODO(ches) set the actual icon
        ItemType::ComponentPouch => '[', //TODO(ches) set the actual icon
        ItemType::CrossbowBolt => '[', //TODO(ches) set the actual icon
        ItemType::CrossbowBoltBundle => '[', //TODO(ches) set the actual icon
        ItemType::CrossbowBoltCase => '[', //TODO(ches) set the actual icon
        ItemType::Crowbar => '[', //TODO(ches) set the actual icon
        ItemType::Crystal => '[', //TODO(ches) set the actual icon
        ItemType::Dagger => '[', //TODO(ches) set the actual icon
        ItemType::Dart => '[', //TODO(ches) set the actual icon
        ItemType::Drum => '[', //TODO(ches) set the actual icon
        ItemType::Emblem => '[', //TODO(ches) set the actual icon
        ItemType::FishingTackle => '[', //TODO(ches) set the actual icon
        ItemType::Flail => '[', //TODO(ches) set the actual icon
        ItemType::Flask => '[', //TODO(ches) set the actual icon
        ItemType::Flute => '[', //TODO(ches) set the actual icon
        ItemType::Glaive => '[', //TODO(ches) set the actual icon
        ItemType::GrapplingHook => '[', //TODO(ches) set the actual icon
        ItemType::Greataxe => '[', //TODO(ches) set the actual icon
        ItemType::Greatclub => '[', //TODO(ches) set the actual icon
        ItemType::Greatsword => '[', //TODO(ches) set the actual icon
        ItemType::Halberd => '[', //TODO(ches) set the actual icon
        ItemType::HalfPlateArmor => '[', //TODO(ches) set the actual icon
        ItemType::Hammer => '[', //TODO(ches) set the actual icon
        ItemType::Handaxe => '[', //TODO(ches) set the actual icon
        ItemType::HandCrossbow => '[', //TODO(ches) set the actual icon
        ItemType::HealersKit => '[', //TODO(ches) set the actual icon
        ItemType::HeavyCrossbow => '[', //TODO(ches) set the actual icon
        ItemType::HideArmor => '[', //TODO(ches) set the actual icon
        ItemType::Horn => '[', //TODO(ches) set the actual icon
        ItemType::Hourglass => '[', //TODO(ches) set the actual icon
        ItemType::HuntingTrap => '[', //TODO(ches) set the actual icon
        ItemType::Ink => '[', //TODO(ches) set the actual icon
        ItemType::Javelin => '[', //TODO(ches) set the actual icon
        ItemType::Jug => '[', //TODO(ches) set the actual icon
        ItemType::Ladder => '[', //TODO(ches) set the actual icon
        ItemType::Lamp => '[', //TODO(ches) set the actual icon
        ItemType::Lance => '[', //TODO(ches) set the actual icon
        ItemType::Lantern => '[', //TODO(ches) set the actual icon
        ItemType::LeatherArmor => '[', //TODO(ches) set the actual icon
        ItemType::LightCrossbow => '[', //TODO(ches) set the actual icon
        ItemType::LightHammer => '[', //TODO(ches) set the actual icon
        ItemType::Lock => '[', //TODO(ches) set the actual icon
        ItemType::Longbow => '[', //TODO(ches) set the actual icon
        ItemType::Longsword => '[', //TODO(ches) set the actual icon
        ItemType::Lute => '[', //TODO(ches) set the actual icon
        ItemType::Lyre => '[', //TODO(ches) set the actual icon
        ItemType::Mace => '[', //TODO(ches) set the actual icon
        ItemType::MagnifyingGlass => '[', //TODO(ches) set the actual icon
        ItemType::Manacles => '[', //TODO(ches) set the actual icon
        ItemType::MapCase => '[', //TODO(ches) set the actual icon
        ItemType::Maul => '[', //TODO(ches) set the actual icon
        ItemType::MessKit => '[', //TODO(ches) set the actual icon
        ItemType::Mirror => '[', //TODO(ches) set the actual icon
        ItemType::Morningstar => '[', //TODO(ches) set the actual icon
        ItemType::Net => '[', //TODO(ches) set the actual icon
        ItemType::Orb => '[', //TODO(ches) set the actual icon
        ItemType::PaddedArmor => '[', //TODO(ches) set the actual icon
        ItemType::PanFlute => '[', //TODO(ches) set the actual icon
        ItemType::Paper => '[', //TODO(ches) set the actual icon
        ItemType::Parchment => '[', //TODO(ches) set the actual icon
        ItemType::Pen => '[', //TODO(ches) set the actual icon
        ItemType::Perfume => '[', //TODO(ches) set the actual icon
        ItemType::Pickaxe => '[', //TODO(ches) set the actual icon
        ItemType::Pike => '[', //TODO(ches) set the actual icon
        ItemType::Piton => '[', //TODO(ches) set the actual icon
        ItemType::PlateArmor => '[', //TODO(ches) set the actual icon
        ItemType::Pole => '[', //TODO(ches) set the actual icon
        ItemType::Pot => '[', //TODO(ches) set the actual icon
        ItemType::Pouch => '[', //TODO(ches) set the actual icon
        ItemType::Quarterstaff => '[', //TODO(ches) set the actual icon
        ItemType::Quiver => '[', //TODO(ches) set the actual icon
        ItemType::Rapier => '[', //TODO(ches) set the actual icon
        ItemType::Rations => '[', //TODO(ches) set the actual icon
        ItemType::Reliquary => '[', //TODO(ches) set the actual icon
        ItemType::Ring => '[', //TODO(ches) set the actual icon
        ItemType::RingMailArmor => '[', //TODO(ches) set the actual icon
        ItemType::Robes => '[', //TODO(ches) set the actual icon
        ItemType::Rod => '[', //TODO(ches) set the actual icon
        ItemType::Rope => '[', //TODO(ches) set the actual icon
        ItemType::Sack => '[', //TODO(ches) set the actual icon
        ItemType::Scale => '[', //TODO(ches) set the actual icon
        ItemType::ScaleMailArmor => '[', //TODO(ches) set the actual icon
        ItemType::Scimitar => '[', //TODO(ches) set the actual icon
        ItemType::Shield => '[', //TODO(ches) set the actual icon
        ItemType::Shortbow => '[', //TODO(ches) set the actual icon
        ItemType::Shortsword => '[', //TODO(ches) set the actual icon
        ItemType::Shovel => '[', //TODO(ches) set the actual icon
        ItemType::Sickle => '[', //TODO(ches) set the actual icon
        ItemType::Sledgehammer => '[', //TODO(ches) set the actual icon
        ItemType::Sling => '[', //TODO(ches) set the actual icon
        ItemType::SlingBullet => '[', //TODO(ches) set the actual icon
        ItemType::SlingBulletBundle => '[', //TODO(ches) set the actual icon
        ItemType::Soap => '[', //TODO(ches) set the actual icon
        ItemType::Spear => '[', //TODO(ches) set the actual icon
        ItemType::Spellbook => '[', //TODO(ches) set the actual icon
        ItemType::Spike => '[', //TODO(ches) set the actual icon
        ItemType::SpikeBundle => '[', //TODO(ches) set the actual icon
        ItemType::SplintArmor => '[', //TODO(ches) set the actual icon
        ItemType::Spyglass => '[', //TODO(ches) set the actual icon
        ItemType::Staff => '[', //TODO(ches) set the actual icon
        ItemType::StuddedLeatherArmor => '[', //TODO(ches) set the actual icon
        ItemType::Tent => '[', //TODO(ches) set the actual icon
        ItemType::Tinderbox => '[', //TODO(ches) set the actual icon
        ItemType::Torch => '[', //TODO(ches) set the actual icon
        ItemType::Totem => '[', //TODO(ches) set the actual icon
        ItemType::Trident => '[', //TODO(ches) set the actual icon
        ItemType::Vial => '[', //TODO(ches) set the actual icon
        ItemType::Violin => '[', //TODO(ches) set the actual icon
        ItemType::Wand => '[', //TODO(ches) set the actual icon
        ItemType::Warhammer => '[', //TODO(ches) set the actual icon
        ItemType::WarPick => '[', //TODO(ches) set the actual icon
        ItemType::Waterskin => '[', //TODO(ches) set the actual icon
        ItemType::Wax => '[', //TODO(ches) set the actual icon
        ItemType::Whetstone => '[', //TODO(ches) set the actual icon
        ItemType::Whip => '[', //TODO(ches) set the actual icon
        ItemType::Whistle => '[', //TODO(ches) set the actual icon
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

