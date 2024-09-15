use enum_map::{enum_map, Enum, EnumMap};

#[derive(Clone, Copy, Debug, Enum)]
pub enum Tag {
    Ammunition,
    Animate,
    Armor,
    Beverage,
    Burning,
    Cold,
    Conductive,
    Consumable,
    Container,
    Explosive,
    FinesseWeapon,
    Flammable,
    Food,
    Frozen,
    Gas,
    GasContainer,
    HeavyArmor,
    HeavyWeapon,
    Hot,
    Indestructible,
    LightArmor,
    LightWeapon,
    Liquid,
    LiquidContainer,
    LoadingWeapon,
    Lockable,
    MartialMeleeWeapon,
    MartialRangedWeapon,
    MediumArmor,
    Organic,
    Openable,
    Poisonous,
    Powder,
    RangedWeapon,
    ReachWeapon,
    Shield,
    SimpleMeleeWeapon,
    SimpleRangedWeapon,
    Solid,
    SolidContainer,
    SpecialWeapon,
    ThrownWeapon,
    Transparent,
    TwoHandedWeapon,
    VersatileWeapon,
    Weapon,
    Wettable
}

pub type TagMap = EnumMap<Tag, Option<Tag>>;

pub fn generate_tag_map() -> TagMap {
    let result: TagMap = enum_map! {
        Tag::Ammunition => Some(Tag::Weapon),
        Tag::Animate => None,
        Tag::Armor => None,
        Tag::Beverage => Some(Tag::Consumable),
        Tag::Burning => Some(Tag::Hot),
        Tag::Cold => None,
        Tag::Conductive => None,
        Tag::Consumable => None,
        Tag::Container => None,
        Tag::Explosive => None,
        Tag::FinesseWeapon => Some(Tag::Weapon),
        Tag::Flammable => None,
        Tag::Food => Some(Tag::Consumable),
        Tag::Frozen => Some(Tag::Cold),
        Tag::Gas => None,
        Tag::GasContainer => Some(Tag::Container),
        Tag::HeavyArmor => Some(Tag::Armor),
        Tag::HeavyWeapon => Some(Tag::Weapon),
        Tag::Hot => None,
        Tag::Indestructible => None,
        Tag::LightArmor => Some(Tag::Armor),
        Tag::LightWeapon => Some(Tag::Weapon),
        Tag::Liquid => None,
        Tag::LiquidContainer => Some(Tag::Container),
        Tag::LoadingWeapon => Some(Tag::Weapon),
        Tag::Lockable => None,
        Tag::MartialMeleeWeapon => Some(Tag::Weapon),
        Tag::MartialRangedWeapon => Some(Tag::Weapon),
        Tag::MediumArmor => Some(Tag::Armor),
        Tag::Organic => None,
        Tag::Openable => None,
        Tag::Poisonous => None,
        Tag::Powder => Some(Tag::Solid),
        Tag::RangedWeapon => Some(Tag::Weapon),
        Tag::ReachWeapon => Some(Tag::Weapon),
        Tag::Shield => Some(Tag::Armor),
        Tag::SimpleMeleeWeapon => Some(Tag::Weapon),
        Tag::SimpleRangedWeapon => Some(Tag::Weapon),
        Tag::Solid => None,
        Tag::SolidContainer => Some(Tag::Container),
        Tag::SpecialWeapon => Some(Tag::Weapon),
        Tag::ThrownWeapon => Some(Tag::Weapon),
        Tag::Transparent => None,
        Tag::TwoHandedWeapon => Some(Tag::Weapon),
        Tag::VersatileWeapon => Some(Tag::Weapon),
        Tag::Weapon => None,
        Tag::Wettable => None
    };
    return result;
}
