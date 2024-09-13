use enum_map::{enum_map, Enum, EnumMap};

#[derive(Clone, Copy, Debug, Enum)]
pub enum Tag {
    Beverage,
    Burning,
    Cold,
    Conductive,
    Consumable,
    Container,
    Explosive,
    Flammable,
    Food,
    Frozen,
    Gas,
    GasContainer,
    Hot,
    Indestructible,
    Liquid,
    LiquidContainer,
    Lockable,
    Organic,
    Openable,
    Poisonous,
    Powder,
    Solid,
    SolidContainer,
    Transparent,
    Wettable
}

pub type TagMap = EnumMap<Tag, Option<Tag>>;

pub fn generate_tag_map() -> TagMap {
    let result: TagMap = enum_map! {
        Tag::Beverage => Some(Tag::Consumable),
        Tag::Burning => Some(Tag::Hot),
        Tag::Cold => None,
        Tag::Conductive => None,
        Tag::Consumable => None,
        Tag::Container => None,
        Tag::Explosive => None,
        Tag::Flammable => None,
        Tag::Food => Some(Tag::Consumable),
        Tag::Frozen => Some(Tag::Cold),
        Tag::Gas => None,
        Tag::GasContainer => Some(Tag::Container),
        Tag::Hot => None,
        Tag::Indestructible => None,
        Tag::Liquid => None,
        Tag::LiquidContainer => Some(Tag::Container),
        Tag::Lockable => None,
        Tag::Organic => None,
        Tag::Openable => None,
        Tag::Poisonous => None,
        Tag::Powder => Some(Tag::Solid),
        Tag::Solid => None,
        Tag::SolidContainer => Some(Tag::Container),
        Tag::Transparent => None,
        Tag::Wettable => None
    };
    return result;
}

