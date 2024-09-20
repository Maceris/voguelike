use std::collections::VecDeque;

use ringbuffer::AllocRingBuffer;

use crate::{action::ActionRequest, component::{Components, EntityType}, entity::EntityID, map::{self, GameMap, TileMap}, material::{self, MaterialMap}, tag::{self, TagMap}};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GameState {
    Menu,
    Paused,
    Running,
    QuitRequested
}

pub struct DebugInfo {
    pub fps_history: AllocRingBuffer<u32>
}

pub struct DataTables {
    pub material_map: MaterialMap,
    pub tag_map: TagMap,
    pub tile_map: TileMap,
}

pub struct SpecialEntities {
    pub down: EntityID,
    pub east: EntityID,
    pub inside: EntityID,
    pub north: EntityID,
    pub north_east: EntityID,
    pub north_west: EntityID,
    pub outside: EntityID,
    pub player: EntityID,
    pub south: EntityID,
    pub south_east: EntityID,
    pub south_west: EntityID,
    pub west: EntityID,
    pub world: EntityID,
    pub up: EntityID,
}

pub struct Game {
    pub action_queue: VecDeque<ActionRequest>,
    pub components: Components,
    pub current_map: Box<GameMap>,
    pub data_tables: DataTables,
    pub debug_info: DebugInfo,
    pub special_entities: SpecialEntities,
    pub state: GameState,
}

impl SpecialEntities {
    pub fn new() -> Self {
        Self {
            down: 0,
            east: 0,
            inside: 0,
            north: 0,
            north_east: 0,
            north_west: 0,
            outside: 0,
            player: 0,
            south: 0,
            south_east: 0,
            south_west: 0,
            west: 0,
            world: 0,
            up: 0,
        }
    }
}

impl Game {
    pub fn new() -> Self {
        let mut result = Self {
            action_queue: VecDeque::with_capacity(1000),
            components: Components::new(),
            current_map: Box::new(GameMap::empty_map()),
            data_tables: DataTables {
                tile_map: map::generate_tile_map(),
                tag_map: tag::generate_tag_map(),
                material_map: material::generate_material_map(),
            },
            debug_info: DebugInfo{fps_history: AllocRingBuffer::new(100)},
            special_entities: SpecialEntities::new(),
            state: GameState::Menu,
        };
        set_up_special_entities(&mut result.special_entities, &mut result.components);
        return result;
    }
}

fn set_up_special_entities(special_entities: &mut SpecialEntities, components: &mut Components) {
    special_entities.player = components.create_entity(EntityType::Character);
    special_entities.world = components.create_entity(EntityType::Meta);
    special_entities.down = components.create_entity(EntityType::Meta);
    special_entities.east = components.create_entity(EntityType::Meta);
    special_entities.inside = components.create_entity(EntityType::Meta);
    special_entities.north = components.create_entity(EntityType::Meta);
    special_entities.north_east = components.create_entity(EntityType::Meta);
    special_entities.north_west = components.create_entity(EntityType::Meta);
    special_entities.outside = components.create_entity(EntityType::Meta);
    special_entities.south = components.create_entity(EntityType::Meta);
    special_entities.south_east = components.create_entity(EntityType::Meta);
    special_entities.south_west = components.create_entity(EntityType::Meta);
    special_entities.west = components.create_entity(EntityType::Meta);
    special_entities.up = components.create_entity(EntityType::Meta);
}
