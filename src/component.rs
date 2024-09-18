use std::{sync::atomic::{AtomicU64, Ordering}};

use crate::{entity::EntityID, map::MapID, tabletop::{Alignment, Class, Race, Size, Stats}};

pub struct Creature {
    alignment: Alignment,
    size: Size,
    race: Race,
    stats: Stats,
}

impl Creature {
    pub fn new() -> Self {
        Self {
            alignment: Alignment::Neutral,
            size: Size::Medium,
            race: Race::Human,
            stats: Stats {
                charisma: 10,
                constitution: 10,
                dexterity: 10,
                intelligence: 10,
                strength: 10,
                wisdom: 10
            },
        }
    }
}

pub struct Character {
    class: Class,
}

impl Character {
    pub fn new() -> Self {
        Self {
            class: Class::Fighter
        }
    }
}

pub struct MapLocation {
    pub map: MapID,
}

impl MapLocation {
    pub fn new() -> Self {
        Self {
            map: 0
        }
    }
}

pub struct Position {
    pub pos_x: u16,
    pub pos_y: u16,
}

impl Position {
    pub fn new() -> Self {
        Self {
            pos_x: 0,
            pos_y: 0
        }
    }
}

pub enum EntityType {
    Character,
    Meta,
    Monster,
    Object,
}

const TYPE_BITMASK: EntityID           = 0b11 << 62;
const TYPE_BITMASK_META: EntityID      = 0b00 << 62;
const TYPE_BITMASK_CHARACTER: EntityID = 0b01 << 62;
const TYPE_BITMASK_MONSTER: EntityID   = 0b10 << 62;
const TYPE_BITMASK_OBJECT: EntityID    = 0b11 << 62;

const FIRST_ID_META: EntityID = TYPE_BITMASK_META;
const FIRST_ID_CHARACTER: EntityID = TYPE_BITMASK_CHARACTER;
const FIRST_ID_MONSTER: EntityID = TYPE_BITMASK_MONSTER;
const FIRST_ID_OBJECT: EntityID = TYPE_BITMASK_OBJECT;

const LAST_ID_META: EntityID = TYPE_BITMASK_META | !TYPE_BITMASK;
const LAST_ID_CHARACTER: EntityID = TYPE_BITMASK_CHARACTER | !TYPE_BITMASK;
const LAST_ID_MONSTER: EntityID = TYPE_BITMASK_MONSTER | !TYPE_BITMASK;
const LAST_ID_OBJECT: EntityID = TYPE_BITMASK_OBJECT | !TYPE_BITMASK;

const DEFAULT_META_COMPONENT_COUNT: usize = 100;
const DEFAULT_CHARACTER_COMPONENT_COUNT: usize = 1000;
const DEFAULT_MONSTER_COMPONENT_COUNT: usize = 1000;
const DEFAULT_OBJECT_COMPONENT_COUNT: usize = 1000;

struct CharacterComponents {
    next_id: AtomicU64,
    pub character: Vec<Character>,
    pub creature: Vec<Creature>,
    pub map_location: Vec<MapLocation>,
    pub position: Vec<Position>,
}

impl CharacterComponents {
    pub fn new() -> Self {
        Self {
            next_id: AtomicU64::new(FIRST_ID_CHARACTER),
            character: Vec::with_capacity(DEFAULT_CHARACTER_COMPONENT_COUNT),
            creature: Vec::with_capacity(DEFAULT_CHARACTER_COMPONENT_COUNT),
            map_location: Vec::with_capacity(DEFAULT_CHARACTER_COMPONENT_COUNT),
            position: Vec::with_capacity(DEFAULT_CHARACTER_COMPONENT_COUNT),
        }
    }

    fn create_entity(&mut self) -> EntityID {
        let id: EntityID = self.next_id.fetch_add(1, Ordering::Relaxed);

        self.character.push(Character::new());
        self.creature.push(Creature::new());
        self.map_location.push(MapLocation::new());
        self.position.push(Position::new());

        return id;
    }
}

struct MetaComponents {
    next_id: AtomicU64,
}

impl MetaComponents {
    pub fn new() -> Self {
        Self {
            next_id: AtomicU64::new(FIRST_ID_META)
        }
    }

    fn create_entity(&mut self) -> EntityID {
        let id: EntityID = self.next_id.fetch_add(1, Ordering::Relaxed);
        return id;
    }
}

struct MonsterComponents {
    next_id: AtomicU64,
    pub creature: Vec<Creature>,
    pub map_location: Vec<MapLocation>,
    pub position: Vec<Position>,
}

impl MonsterComponents {
    pub fn new() -> Self {
        Self {
            next_id: AtomicU64::new(FIRST_ID_MONSTER),
            creature: Vec::with_capacity(DEFAULT_MONSTER_COMPONENT_COUNT),
            map_location: Vec::with_capacity(DEFAULT_MONSTER_COMPONENT_COUNT),
            position: Vec::with_capacity(DEFAULT_MONSTER_COMPONENT_COUNT),
        }
    }

    fn create_entity(&mut self) -> EntityID {
        let id: EntityID = self.next_id.fetch_add(1, Ordering::Relaxed);

        self.creature.push(Creature::new());
        self.map_location.push(MapLocation::new());
        self.position.push(Position::new());

        return id;
    }
}

struct ObjectComponents {
    next_id: AtomicU64,
    pub map_location: Vec<MapLocation>,
    pub position: Vec<Position>,
}

impl ObjectComponents {
    pub fn new() -> Self {
        Self {
            next_id: AtomicU64::new(FIRST_ID_OBJECT),
            map_location: Vec::with_capacity(DEFAULT_OBJECT_COMPONENT_COUNT),
            position: Vec::with_capacity(DEFAULT_OBJECT_COMPONENT_COUNT),
        }
    }

    fn create_entity(&mut self) -> EntityID {
        let id: EntityID = self.next_id.fetch_add(1, Ordering::Relaxed);

        self.map_location.push(MapLocation::new());
        self.position.push(Position::new());

        return id;
    }
}

pub struct Components {
    character_components: CharacterComponents,
    meta_components: MetaComponents,
    monster_components: MonsterComponents,
    object_components: ObjectComponents,
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

    pub fn create_entity(&mut self, entity_type: EntityType) -> EntityID {
        match entity_type {
            EntityType::Character => self.character_components.create_entity(),
            EntityType::Meta => self.meta_components.create_entity(),
            EntityType::Monster => self.monster_components.create_entity(),
            EntityType::Object => self.object_components.create_entity(),
        }
    }
}