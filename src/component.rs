use std::fmt::Alignment;

use crate::{entity::EntityID, map::MapID, tabletop::{Class, Race, Size, Stats}};

pub struct Creature {
    alignment: Alignment,
    size: Size,
    race: Race,
    stats: Stats,
}

pub struct Character {
    class: Class,
}

pub struct MapLocation {
    pub map: MapID,
}

pub struct Position {
    pub pos_x: u16,
    pub pos_y: u16,
}

pub enum EntityType {
    Character,
    Meta,
    Monster,
    Object,
}

const TYPE_BITMASK: EntityID           = 0b11 << 62;
const TYPE_BITMASK_CHARACTER: EntityID = 0b00 << 62;
const TYPE_BITMASK_META: EntityID      = 0b01 << 62;
const TYPE_BITMASK_MONSTER: EntityID   = 0b10 << 62;
const TYPE_BITMASK_OBJECT: EntityID    = 0b11 << 62;

const FIRST_ID_CHARACTER: EntityID = TYPE_BITMASK_CHARACTER;
const FIRST_ID_META: EntityID = TYPE_BITMASK_META;
const FIRST_ID_MONSTER: EntityID = TYPE_BITMASK_MONSTER;
const FIRST_ID_OBJECT: EntityID = TYPE_BITMASK_OBJECT;

const LAST_ID_CHARACTER: EntityID = TYPE_BITMASK_CHARACTER | !TYPE_BITMASK;
const LAST_ID_META: EntityID = TYPE_BITMASK_META | !TYPE_BITMASK;
const LAST_ID_MONSTER: EntityID = TYPE_BITMASK_MONSTER | !TYPE_BITMASK;
const LAST_ID_OBJECT: EntityID = TYPE_BITMASK_OBJECT | !TYPE_BITMASK;

struct CharacterComponents {
    pub next_id: EntityID,
    pub character: Vec<Character>,
    pub creature: Vec<Creature>,
    pub map_location: Vec<MapLocation>,
    pub position: Vec<Position>,
}

struct MetaComponents {
    pub next_id: EntityID,
}

struct MonsterComponents {
    pub next_id: EntityID,
    pub creature: Vec<Creature>,
    pub map_location: Vec<MapLocation>,
    pub position: Vec<Position>,
}

struct ObjectComponents {
    pub next_id: EntityID,
    pub map_location: Vec<MapLocation>,
    pub position: Vec<Position>,
}

pub struct Components {
    character_components: CharacterComponents,
    meta_components: MetaComponents,
    monster_components: MonsterComponents,
    object_components: ObjectComponents,
}

impl CharacterComponents {
    pub fn new() -> Self {
        Self {
            next_id: FIRST_ID_CHARACTER,
            character: Vec::new(),
            creature: Vec::new(),
            map_location: Vec::new(),
            position: Vec::new(),
        }
    }
}

impl MetaComponents {
    pub fn new() -> Self {
        Self {
            next_id: FIRST_ID_META
        }
    }
}

impl MonsterComponents {
    pub fn new() -> Self {
        Self {
            next_id: FIRST_ID_MONSTER,
            creature: Vec::new(),
            map_location: Vec::new(),
            position: Vec::new(),
        }
    }
}

impl ObjectComponents {
    pub fn new() -> Self {
        Self {
            next_id: FIRST_ID_OBJECT,
            map_location: Vec::new(),
            position: Vec::new(),
        }
    }
}

impl Components {
    pub fn new() -> Self {
        Self {
            character_components: CharacterComponents::new(),
            meta_components: MetaComponents::new(),
            monster_components: MonsterComponents::new(),
            object_components: ObjectComponents::new(),
        }
    }
}