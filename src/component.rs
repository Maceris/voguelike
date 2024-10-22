use std::sync::atomic::{AtomicUsize, Ordering};

use crate::{entity::EntityID, map::MapID, tabletop::{Alignment, Class, Race, Size, Stats}};

pub struct Alive {
    pub alive: bool
}

impl Alive {
    pub fn new() -> Self {
        Self {
            alive: true
        }
    }
}
macro_rules! impl_get_alive {
    () => {
        fn get_alive(&self, entity: EntityID) -> Option<&Alive> {
            return self.alive.get(to_index(entity));
        }

        fn get_alive_mut(&mut self, entity: EntityID) -> Option<&mut Alive> {
            return self.alive.get_mut(to_index(entity));
        }
    };
}

pub struct Creature {
    pub alignment: Alignment,
    pub size: Size,
    pub race: Race,
    pub stats: Stats,
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
macro_rules! impl_get_creature {
    () => {
        fn get_creature(&self, entity: EntityID) -> Option<&Creature> {
            return self.creature.get(to_index(entity));
        }

        fn get_creature_mut(&mut self, entity: EntityID) -> Option<&mut Creature> {
            return self.creature.get_mut(to_index(entity));
        }
    };
}

pub struct Character {
    pub class: Class,
}

impl Character {
    pub fn new() -> Self {
        Self {
            class: Class::Fighter
        }
    }
}
macro_rules! impl_get_character {
    () => {
        fn get_character(&self, entity: EntityID) -> Option<&Character> {
            return self.character.get(to_index(entity));
        }

        fn get_character_mut(&mut self, entity: EntityID) -> Option<&mut Character> {
            return self.character.get_mut(to_index(entity));
        }
    };
}

pub struct MapIndex {
    pub map: MapID,
}

impl MapIndex {
    pub fn new() -> Self {
        Self {
            map: 0
        }
    }
}
macro_rules! impl_get_map_index {
    () => {
        fn get_map_index(&self, entity: EntityID) -> Option<&MapIndex> {
            return self.map_index.get(to_index(entity));
        }

        fn get_map_index_mut(&mut self, entity: EntityID) -> Option<&mut MapIndex> {
            return self.map_index.get_mut(to_index(entity));
        }
    };
}

pub struct Position {
    pub x: u16,
    pub y: u16,
}

impl Position {
    pub fn new() -> Self {
        Self {
            x: 0,
            y: 0
        }
    }
}
macro_rules! impl_get_position {
    () => {
        fn get_position(&self, entity: EntityID) -> Option<&Position> {
            return self.position.get(to_index(entity));
        }

        fn get_position_mut(&mut self, entity: EntityID) -> Option<&mut Position> {
            return self.position.get_mut(to_index(entity));
        }
    };
}

#[derive(PartialEq)]
pub enum EntityType {
    Character,
    Meta,
    Monster,
    Object,
}

const TYPE_BITMASK: EntityID           = 0b11 << (usize::BITS - 2);
const TYPE_BITMASK_META: EntityID      = 0b00 << (usize::BITS - 2);
const TYPE_BITMASK_CHARACTER: EntityID = 0b01 << (usize::BITS - 2);
const TYPE_BITMASK_MONSTER: EntityID   = 0b10 << (usize::BITS - 2);
const TYPE_BITMASK_OBJECT: EntityID    = 0b11 << (usize::BITS - 2);

const DEFAULT_META_COMPONENT_COUNT: usize = 100;
const DEFAULT_CHARACTER_COMPONENT_COUNT: usize = 1000;
const DEFAULT_MONSTER_COMPONENT_COUNT: usize = 1000;
const DEFAULT_OBJECT_COMPONENT_COUNT: usize = 1000;

fn get_entity_type(entity: EntityID) -> EntityType {
    let masked = entity & TYPE_BITMASK;
    if masked == TYPE_BITMASK_META {
        return EntityType::Meta;
    }
    if masked == TYPE_BITMASK_CHARACTER {
        return EntityType::Character;
    }
    if masked == TYPE_BITMASK_MONSTER {
        return EntityType::Monster;
    }
    if masked == TYPE_BITMASK_OBJECT {
        return EntityType::Object;
    }
    panic!("Unexpected entity type, please check the type bitmasks are right");
}

fn to_index(entity: EntityID) -> usize {
    return entity & !TYPE_BITMASK;
}

pub struct CharacterComponents {
    next_id: AtomicUsize,
    pub alive: Vec<Alive>,
    pub character: Vec<Character>,
    pub creature: Vec<Creature>,
    pub map_index: Vec<MapIndex>,
    pub position: Vec<Position>,
}

impl CharacterComponents {
    pub fn new() -> Self {
        Self {
            next_id: AtomicUsize::new(0),
            alive: Vec::with_capacity(DEFAULT_CHARACTER_COMPONENT_COUNT),
            character: Vec::with_capacity(DEFAULT_CHARACTER_COMPONENT_COUNT),
            creature: Vec::with_capacity(DEFAULT_CHARACTER_COMPONENT_COUNT),
            map_index: Vec::with_capacity(DEFAULT_CHARACTER_COMPONENT_COUNT),
            position: Vec::with_capacity(DEFAULT_CHARACTER_COMPONENT_COUNT),
        }
    }

    fn create_entity(&mut self) -> EntityID {
        if self.next_id.load(Ordering::Relaxed) == usize::MAX {
            panic!("Ran out of character entity IDs!");
        }
        let id: EntityID = self.next_id.fetch_add(1, Ordering::Relaxed);
        
        self.alive.push(Alive::new());
        self.character.push(Character::new());
        self.creature.push(Creature::new());
        self.map_index.push(MapIndex::new());
        self.position.push(Position::new());

        return id | TYPE_BITMASK_CHARACTER;
    }

    pub fn get_size(&self) -> usize {
        return self.next_id.load(Ordering::Relaxed);
    }

    impl_get_alive!();
    impl_get_creature!();
    impl_get_character!();
    impl_get_map_index!();
    impl_get_position!();
}

pub struct MetaComponents {
    next_id: AtomicUsize,
    alive: Vec<Alive>,
}

impl MetaComponents {
    pub fn new() -> Self {
        Self {
            next_id: AtomicUsize::new(0),
            alive: Vec::with_capacity(DEFAULT_META_COMPONENT_COUNT),
        }
    }

    fn create_entity(&mut self) -> EntityID {
        if self.next_id.load(Ordering::Relaxed) == usize::MAX {
            panic!("Ran out of meta entity IDs!");
        }
        let id: EntityID = self.next_id.fetch_add(1, Ordering::Relaxed);
        
        self.alive.push(Alive::new());

        return id | TYPE_BITMASK_META;
    }

    pub fn get_size(&self) -> usize {
        return self.next_id.load(Ordering::Relaxed);
    }

    impl_get_alive!();
}

pub struct MonsterComponents {
    next_id: AtomicUsize,
    pub alive: Vec<Alive>,
    pub creature: Vec<Creature>,
    pub map_index: Vec<MapIndex>,
    pub position: Vec<Position>,
}

impl MonsterComponents {
    pub fn new() -> Self {
        Self {
            next_id: AtomicUsize::new(0),
            alive: Vec::with_capacity(DEFAULT_MONSTER_COMPONENT_COUNT),
            creature: Vec::with_capacity(DEFAULT_MONSTER_COMPONENT_COUNT),
            map_index: Vec::with_capacity(DEFAULT_MONSTER_COMPONENT_COUNT),
            position: Vec::with_capacity(DEFAULT_MONSTER_COMPONENT_COUNT),
        }
    }

    fn create_entity(&mut self) -> EntityID {
        if self.next_id.load(Ordering::Relaxed) == usize::MAX {
            panic!("Ran out of character entity IDs!");
        }
        let id: EntityID = self.next_id.fetch_add(1, Ordering::Relaxed);

        self.alive.push(Alive::new());
        self.creature.push(Creature::new());
        self.map_index.push(MapIndex::new());
        self.position.push(Position::new());

        return id | TYPE_BITMASK_MONSTER;
    }

    pub fn get_size(&self) -> usize {
        return self.next_id.load(Ordering::Relaxed);
    }

    impl_get_alive!();
    impl_get_creature!();
    impl_get_map_index!();
    impl_get_position!();
}

pub struct ObjectComponents {
    next_id: AtomicUsize,
    pub alive: Vec<Alive>,
    pub map_index: Vec<MapIndex>,
    pub parent: Vec<EntityID>,
    pub position: Vec<Position>,
}

impl ObjectComponents {
    pub fn new() -> Self {
        Self {
            next_id: AtomicUsize::new(0),
            alive: Vec::with_capacity(DEFAULT_OBJECT_COMPONENT_COUNT),
            map_index: Vec::with_capacity(DEFAULT_OBJECT_COMPONENT_COUNT),
            parent: Vec::with_capacity(DEFAULT_OBJECT_COMPONENT_COUNT),
            position: Vec::with_capacity(DEFAULT_OBJECT_COMPONENT_COUNT),
        }
    }

    fn create_entity(&mut self) -> EntityID {
        if self.next_id.load(Ordering::Relaxed) == usize::MAX {
            panic!("Ran out of object entity IDs!");
        }
        let id: EntityID = self.next_id.fetch_add(1, Ordering::Relaxed);

        self.alive.push(Alive::new());
        self.map_index.push(MapIndex::new());
        self.position.push(Position::new());

        return id | TYPE_BITMASK_OBJECT;
    }

    pub fn get_size(&self) -> usize {
        return self.next_id.load(Ordering::Relaxed);
    }

    impl_get_alive!();
    impl_get_map_index!();
    impl_get_position!();
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

    pub fn get_character_components(&self) -> &CharacterComponents {
        return &self.character_components;
    }

    pub fn get_meta_components(&self) -> &MetaComponents {
        return &self.meta_components;
    }

    pub fn get_monster_components(&self) -> &MonsterComponents {
        return &self.monster_components;
    }

    pub fn get_object_components(&self) -> &ObjectComponents {
        return &self.object_components;
    }
    
    fn get_alive(&self, entity: EntityID) -> Option<&Alive> {
        return match get_entity_type(entity) {
            EntityType::Character => self.character_components.get_alive(entity),
            EntityType::Monster => self.monster_components.get_alive(entity),
            EntityType::Meta => self.meta_components.get_alive(entity),
            EntityType::Object => self.object_components.get_alive(entity),
        };
    }

    fn get_alive_mut(&mut self, entity: EntityID) -> Option<&mut Alive> {
        return match get_entity_type(entity) {
            EntityType::Character => self.character_components.get_alive_mut(entity),
            EntityType::Monster => self.monster_components.get_alive_mut(entity),
            EntityType::Meta => self.meta_components.get_alive_mut(entity),
            EntityType::Object => self.object_components.get_alive_mut(entity),
        };
    }

    pub fn get_creature(&self, entity: EntityID) -> Option<&Creature> {
        return match get_entity_type(entity) {
            EntityType::Character => self.character_components.get_creature(entity),
            EntityType::Monster => self.monster_components.get_creature(entity),
            _ => None,
        };
    }

    pub fn get_creature_mut(&mut self, entity: EntityID) -> Option<&mut Creature> {
        return match get_entity_type(entity) {
            EntityType::Character => self.character_components.get_creature_mut(entity),
            EntityType::Monster => self.monster_components.get_creature_mut(entity),
            _ => None,
        };
    }

    pub fn get_character(&self, entity: EntityID) -> Option<&Character> {
        return match get_entity_type(entity) {
            EntityType::Character => self.character_components.get_character(entity),
            _ => None,
        };
    }

    pub fn get_character_mut(&mut self, entity: EntityID) -> Option<&mut Character> {
        return match get_entity_type(entity) {
            EntityType::Character => self.character_components.get_character_mut(entity),
            _ => None,
        };
    }

    pub fn get_map_index(&self, entity: EntityID) -> Option<&MapIndex> {
        return match get_entity_type(entity) {
            EntityType::Character => self.character_components.get_map_index(entity),
            EntityType::Monster => self.monster_components.get_map_index(entity),
            EntityType::Object => self.object_components.get_map_index(entity),
            _ => None,
        };
    }

    pub fn get_map_index_mut(&mut self, entity: EntityID) -> Option<&mut MapIndex> {
        return match get_entity_type(entity) {
            EntityType::Character => self.character_components.get_map_index_mut(entity),
            EntityType::Monster => self.monster_components.get_map_index_mut(entity),
            EntityType::Object => self.object_components.get_map_index_mut(entity),
            _ => None,
        };
    }

    pub fn get_position(&self, entity: EntityID) -> Option<&Position> {
        return match get_entity_type(entity) {
            EntityType::Character => self.character_components.get_position(entity),
            EntityType::Monster => self.monster_components.get_position(entity),
            EntityType::Object => self.object_components.get_position(entity),
            _ => None,
        };
    }

    pub fn get_position_mut(&mut self, entity: EntityID) -> Option<&mut Position> {
        return match get_entity_type(entity) {
            EntityType::Character => self.character_components.get_position_mut(entity),
            EntityType::Monster => self.monster_components.get_position_mut(entity),
            EntityType::Object => self.object_components.get_position_mut(entity),
            _ => None,
        };
    }

}