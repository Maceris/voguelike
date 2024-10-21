use enum_map::{Enum, EnumMap, enum_map};

use crate::{entity::EntityID, tag::Tag};

pub struct Item {
    pub item_type: ItemType,
    pub location: ItemLocation,
}

pub enum ItemLocation {
    Entity(EntityID),
    World,
}

#[derive(Clone, Copy, Debug, Enum)]
pub enum ItemType {
    Abacus,
    Amulet,
    Arrow,
    ArrowBundle,
    Backpack,
    Bagpipes,
    BallBearing,
    BallBearingBundle,
    Barrel,
    Basket,
    BatteringRam,
    Battleaxe,
    Bedroll,
    Bell,
    Blanket,
    BlockAndTackle,
    Blowgun,
    BlowgunNeedle,
    BlowgunNeedleBundle,
    Book,
    Bottle,
    BreastplateArmor,
    Bucket,
    Caltrop,
    CaltropBundle,
    Candle,
    Chain,
    ChainMailArmor,
    ChainShirtArmor,
    Chalk,
    Chest,
    ClimbersKit,
    ClothesCommon,
    ClothesCostume,
    ClothesFine,
    ClothesTraveler,
    Club,
    Coin,
    ComponentPouch,
    CrossbowBolt,
    CrossbowBoltBundle,
    CrossbowBoltCase,
    Crowbar,
    Crystal,
    Dagger,
    Dart,
    Drum,
    Emblem,
    FishingTackle,
    Flail,
    Flask,
    Flute,
    Glaive,
    GrapplingHook,
    Greataxe,
    Greatclub,
    Greatsword,
    Halberd,
    HalfPlateArmor,
    Hammer,
    Handaxe,
    HandCrossbow,
    HealersKit,
    HeavyCrossbow,
    HideArmor,
    Horn,
    Hourglass,
    HuntingTrap,
    Ink,
    Javelin,
    Jug,
    Ladder,
    Lamp,
    Lance,
    Lantern,
    LeatherArmor,
    LightCrossbow,
    LightHammer,
    Lock,
    Longbow,
    Longsword,
    Lute,
    Lyre,
    Mace,
    MagnifyingGlass,
    Manacles,
    MapCase,
    Maul,
    MessKit,
    Mirror,
    Morningstar,
    Net,
    Orb,
    PaddedArmor,
    PanFlute,
    Paper,
    Parchment,
    Pen,
    Perfume,
    Pickaxe,
    Pike,
    Piton,
    PlateArmor,
    Pole,
    Pot,
    Pouch,
    Quarterstaff,
    Quiver,
    Rapier,
    Rations,
    Reliquary,
    Ring,
    RingMailArmor,
    Robes,
    Rod,
    Rope,
    Sack,
    Scale,
    ScaleMailArmor,
    Scimitar,
    Shield,
    Shortbow,
    Shortsword,
    Shovel,
    Sickle,
    Sledgehammer,
    Sling,
    SlingBullet,
    SlingBulletBundle,
    Soap,
    Spear,
    Spellbook,
    Spike,
    SpikeBundle,
    SplintArmor,
    Spyglass,
    Staff,
    StuddedLeatherArmor,
    Tent,
    Tinderbox,
    Torch,
    Totem,
    Trident,
    Vial,
    Violin,
    Wand,
    Warhammer,
    WarPick,
    Waterskin,
    Wax,
    Whetstone,
    Whip,
    Whistle,
}

pub enum CoinType {
    Copper,
    Silver,
    Electrum,
    Gold,
    Platinum
}

pub struct CurrencyAmount {
    amount: u32,
    coin_type: CoinType,
}

impl CurrencyAmount {
    pub const fn new(amount: u32, coin_type: CoinType) -> Self {
        Self {amount, coin_type}
    }
    pub const fn zero() -> Self {
        Self {
            amount: 0, 
            coin_type: CoinType::Copper
        }
    }
}

pub const fn get_cost(item_type: ItemType) -> CurrencyAmount {
    match item_type {
        ItemType::Abacus => CurrencyAmount::new(2, CoinType::Gold),
        ItemType::Amulet => CurrencyAmount::new(5, CoinType::Gold),
        ItemType::Arrow => CurrencyAmount::new(5, CoinType::Copper),
        ItemType::ArrowBundle => CurrencyAmount::new(1, CoinType::Gold),
        ItemType::Backpack => CurrencyAmount::new(2, CoinType::Gold),
        ItemType::Bagpipes => CurrencyAmount::new(30, CoinType::Gold),
        ItemType::BallBearing => CurrencyAmount::zero(),
        ItemType::BallBearingBundle => CurrencyAmount::new(1, CoinType::Gold),
        ItemType::Barrel => CurrencyAmount::new(2, CoinType::Gold),
        ItemType::Basket => CurrencyAmount::new(4, CoinType::Silver),
        ItemType::BatteringRam => CurrencyAmount::new(4, CoinType::Gold),
        ItemType::Battleaxe => CurrencyAmount::new(10, CoinType::Gold),
        ItemType::Bedroll => CurrencyAmount::new(1, CoinType::Gold),
        ItemType::Bell => CurrencyAmount::new(1, CoinType::Gold),
        ItemType::Blanket => CurrencyAmount::new(5, CoinType::Silver),
        ItemType::BlockAndTackle => CurrencyAmount::new(1, CoinType::Gold),
        ItemType::Blowgun => CurrencyAmount::new(10, CoinType::Gold),
        ItemType::BlowgunNeedle => CurrencyAmount::new(2, CoinType::Copper),
        ItemType::BlowgunNeedleBundle => CurrencyAmount::new(1, CoinType::Gold),
        ItemType::Book => CurrencyAmount::new(25, CoinType::Gold),
        ItemType::Bottle => CurrencyAmount::new(2, CoinType::Gold),
        ItemType::BreastplateArmor => CurrencyAmount::new(400, CoinType::Gold),
        ItemType::Bucket => CurrencyAmount::new(5, CoinType::Copper),
        ItemType::Caltrop => CurrencyAmount::new(5, CoinType::Copper),
        ItemType::CaltropBundle => CurrencyAmount::new(1, CoinType::Gold),
        ItemType::Candle => CurrencyAmount::new(1, CoinType::Copper),
        ItemType::Chain => CurrencyAmount::new(5, CoinType::Gold),
        ItemType::ChainMailArmor => CurrencyAmount::new(75, CoinType::Gold),
        ItemType::ChainShirtArmor => CurrencyAmount::new(50, CoinType::Gold),
        ItemType::Chalk => CurrencyAmount::new(1, CoinType::Copper),
        ItemType::Chest => CurrencyAmount::new(5, CoinType::Gold),
        ItemType::ClimbersKit => CurrencyAmount::new(25, CoinType::Gold),
        ItemType::ClothesCommon => CurrencyAmount::new(5, CoinType::Silver),
        ItemType::ClothesCostume => CurrencyAmount::new(5, CoinType::Gold),
        ItemType::ClothesFine => CurrencyAmount::new(15, CoinType::Gold),
        ItemType::ClothesTraveler => CurrencyAmount::new(2, CoinType::Gold),
        ItemType::Club => CurrencyAmount::new(1, CoinType::Silver),
        ItemType::Coin => CurrencyAmount::zero(),
        ItemType::ComponentPouch => CurrencyAmount::new(25, CoinType::Gold),
        ItemType::CrossbowBolt => CurrencyAmount::new(5, CoinType::Copper),
        ItemType::CrossbowBoltBundle => CurrencyAmount::new(1, CoinType::Gold),
        ItemType::CrossbowBoltCase => CurrencyAmount::new(1, CoinType::Gold),
        ItemType::Crowbar => CurrencyAmount::new(2, CoinType::Gold),
        ItemType::Crystal => CurrencyAmount::new(10, CoinType::Gold),
        ItemType::Dagger => CurrencyAmount::new(2, CoinType::Gold),
        ItemType::Dart => CurrencyAmount::new(5, CoinType::Copper),
        ItemType::Drum => CurrencyAmount::new(6, CoinType::Gold),
        ItemType::Emblem => CurrencyAmount::new(5, CoinType::Gold),
        ItemType::FishingTackle => CurrencyAmount::new(1, CoinType::Gold),
        ItemType::Flail => CurrencyAmount::new(10, CoinType::Gold),
        ItemType::Flask => CurrencyAmount::new(2, CoinType::Copper),
        ItemType::Flute => CurrencyAmount::new(2, CoinType::Gold),
        ItemType::Glaive => CurrencyAmount::new(20, CoinType::Gold),
        ItemType::GrapplingHook => CurrencyAmount::new(2, CoinType::Gold),
        ItemType::Greataxe => CurrencyAmount::new(30, CoinType::Gold),
        ItemType::Greatclub => CurrencyAmount::new(2, CoinType::Silver),
        ItemType::Greatsword => CurrencyAmount::new(50, CoinType::Gold),
        ItemType::Halberd => CurrencyAmount::new(20, CoinType::Gold),
        ItemType::HalfPlateArmor => CurrencyAmount::new(750, CoinType::Gold),
        ItemType::Hammer => CurrencyAmount::new(1, CoinType::Gold),
        ItemType::Handaxe => CurrencyAmount::new(5, CoinType::Gold),
        ItemType::HandCrossbow => CurrencyAmount::new(75, CoinType::Gold),
        ItemType::HealersKit => CurrencyAmount::new(5, CoinType::Gold),
        ItemType::HeavyCrossbow => CurrencyAmount::new(50, CoinType::Gold),
        ItemType::HideArmor => CurrencyAmount::new(10, CoinType::Gold),
        ItemType::Horn => CurrencyAmount::new(3, CoinType::Gold),
        ItemType::Hourglass => CurrencyAmount::new(25, CoinType::Gold),
        ItemType::HuntingTrap => CurrencyAmount::new(5, CoinType::Gold),
        ItemType::Ink => CurrencyAmount::new(10, CoinType::Gold),
        ItemType::Javelin => CurrencyAmount::new(5, CoinType::Silver),
        ItemType::Jug => CurrencyAmount::new(2, CoinType::Copper),
        ItemType::Ladder => CurrencyAmount::new(1, CoinType::Silver),
        ItemType::Lamp => CurrencyAmount::new(5, CoinType::Silver),
        ItemType::Lance => CurrencyAmount::new(10, CoinType::Gold),
        ItemType::Lantern => CurrencyAmount::new(5, CoinType::Gold),
        ItemType::LeatherArmor => CurrencyAmount::new(10, CoinType::Gold),
        ItemType::LightCrossbow => CurrencyAmount::new(25, CoinType::Gold),
        ItemType::LightHammer => CurrencyAmount::new(2, CoinType::Gold),
        ItemType::Lock => CurrencyAmount::new(10, CoinType::Gold),
        ItemType::Longbow => CurrencyAmount::new(50, CoinType::Gold),
        ItemType::Longsword => CurrencyAmount::new(15, CoinType::Gold),
        ItemType::Lute => CurrencyAmount::new(35, CoinType::Gold),
        ItemType::Lyre => CurrencyAmount::new(30, CoinType::Gold),
        ItemType::Mace => CurrencyAmount::new(5, CoinType::Gold),
        ItemType::MagnifyingGlass => CurrencyAmount::new(100, CoinType::Gold),
        ItemType::Manacles => CurrencyAmount::new(2, CoinType::Gold),
        ItemType::MapCase => CurrencyAmount::new(1, CoinType::Gold),
        ItemType::Maul => CurrencyAmount::new(10, CoinType::Gold),
        ItemType::MessKit => CurrencyAmount::new(2, CoinType::Silver),
        ItemType::Mirror => CurrencyAmount::new(5, CoinType::Gold),
        ItemType::Morningstar => CurrencyAmount::new(15, CoinType::Gold),
        ItemType::Net => CurrencyAmount::new(1, CoinType::Gold),
        ItemType::Orb => CurrencyAmount::new(20, CoinType::Gold),
        ItemType::PaddedArmor => CurrencyAmount::new(5, CoinType::Gold),
        ItemType::PanFlute => CurrencyAmount::new(12, CoinType::Gold),
        ItemType::Paper => CurrencyAmount::new(2, CoinType::Silver),
        ItemType::Parchment => CurrencyAmount::new(1, CoinType::Silver),
        ItemType::Pen => CurrencyAmount::new(2, CoinType::Copper),
        ItemType::Perfume => CurrencyAmount::new(5, CoinType::Gold),
        ItemType::Pickaxe => CurrencyAmount::new(2, CoinType::Gold),
        ItemType::Pike => CurrencyAmount::new(5, CoinType::Gold),
        ItemType::Piton => CurrencyAmount::new(5, CoinType::Copper),
        ItemType::PlateArmor => CurrencyAmount::new(1500, CoinType::Gold),
        ItemType::Pole => CurrencyAmount::new(5, CoinType::Copper),
        ItemType::Pot => CurrencyAmount::new(2, CoinType::Gold),
        ItemType::Pouch => CurrencyAmount::new(5, CoinType::Silver),
        ItemType::Quarterstaff => CurrencyAmount::new(2, CoinType::Silver),
        ItemType::Quiver => CurrencyAmount::new(1, CoinType::Gold),
        ItemType::Rapier => CurrencyAmount::new(25, CoinType::Gold),
        ItemType::Rations => CurrencyAmount::new(5, CoinType::Silver),
        ItemType::Reliquary => CurrencyAmount::new(5, CoinType::Gold),
        ItemType::Ring => CurrencyAmount::new(5, CoinType::Gold),
        ItemType::RingMailArmor => CurrencyAmount::new(30, CoinType::Gold),
        ItemType::Robes => CurrencyAmount::new(1, CoinType::Gold),
        ItemType::Rod => CurrencyAmount::new(10, CoinType::Gold),
        ItemType::Rope => CurrencyAmount::new(1, CoinType::Gold),
        ItemType::Sack => CurrencyAmount::new(1, CoinType::Copper),
        ItemType::Scale => CurrencyAmount::new(5, CoinType::Gold),
        ItemType::ScaleMailArmor => CurrencyAmount::new(50, CoinType::Gold),
        ItemType::Scimitar => CurrencyAmount::new(25, CoinType::Gold),
        ItemType::Shield => CurrencyAmount::new(10, CoinType::Gold),
        ItemType::Shortbow => CurrencyAmount::new(25, CoinType::Gold),
        ItemType::Shortsword => CurrencyAmount::new(10, CoinType::Gold),
        ItemType::Shovel => CurrencyAmount::new(2, CoinType::Gold),
        ItemType::Sickle => CurrencyAmount::new(1, CoinType::Gold),
        ItemType::Sledgehammer => CurrencyAmount::new(2, CoinType::Gold),
        ItemType::Sling => CurrencyAmount::new(1, CoinType::Silver),
        ItemType::SlingBullet => CurrencyAmount::zero(),
        ItemType::SlingBulletBundle => CurrencyAmount::new(4, CoinType::Copper),
        ItemType::Soap => CurrencyAmount::new(2, CoinType::Copper),
        ItemType::Spear => CurrencyAmount::new(1, CoinType::Gold),
        ItemType::Spellbook => CurrencyAmount::new(50, CoinType::Gold),
        ItemType::Spike => CurrencyAmount::new(10, CoinType::Copper),
        ItemType::SpikeBundle => CurrencyAmount::new(1, CoinType::Gold),
        ItemType::SplintArmor => CurrencyAmount::new(200, CoinType::Gold),
        ItemType::Spyglass => CurrencyAmount::new(1000, CoinType::Gold),
        ItemType::Staff => CurrencyAmount::new(5, CoinType::Gold),
        ItemType::StuddedLeatherArmor => CurrencyAmount::new(45, CoinType::Gold),
        ItemType::Tent => CurrencyAmount::new(2, CoinType::Gold),
        ItemType::Tinderbox => CurrencyAmount::new(5, CoinType::Silver),
        ItemType::Torch => CurrencyAmount::new(1, CoinType::Copper),
        ItemType::Totem => CurrencyAmount::new(1, CoinType::Gold),
        ItemType::Trident => CurrencyAmount::new(5, CoinType::Gold),
        ItemType::Vial => CurrencyAmount::new(1, CoinType::Gold),
        ItemType::Violin => CurrencyAmount::new(30, CoinType::Gold),
        ItemType::Wand => CurrencyAmount::new(10, CoinType::Gold),
        ItemType::Warhammer => CurrencyAmount::new(15, CoinType::Gold),
        ItemType::WarPick => CurrencyAmount::new(5, CoinType::Gold),
        ItemType::Waterskin => CurrencyAmount::new(2, CoinType::Silver),
        ItemType::Wax => CurrencyAmount::new(5, CoinType::Silver),
        ItemType::Whetstone => CurrencyAmount::new(1, CoinType::Copper),
        ItemType::Whip => CurrencyAmount::new(2, CoinType::Gold),
        ItemType::Whistle => CurrencyAmount::new(5, CoinType::Copper),
    }
}

pub type ItemTagMap = EnumMap<ItemType, Vec<Tag>>;

pub fn generate_item_tag_map() -> ItemTagMap {
    enum_map! {
        ItemType::Abacus => vec!(),
        ItemType::Amulet => vec!(),
        ItemType::Arrow => vec!(Tag::Ammunition),
        ItemType::ArrowBundle => vec!(),
        ItemType::Backpack => vec!(),
        ItemType::Bagpipes => vec!(Tag::Instrument),
        ItemType::BallBearing => vec!(),
        ItemType::BallBearingBundle => vec!(),
        ItemType::Barrel => vec!(),
        ItemType::Basket => vec!(),
        ItemType::BatteringRam => vec!(),
        ItemType::Battleaxe => vec!(),
        ItemType::Bedroll => vec!(),
        ItemType::Bell => vec!(),
        ItemType::Blanket => vec!(),
        ItemType::BlockAndTackle => vec!(),
        ItemType::Blowgun => vec!(),
        ItemType::BlowgunNeedle => vec!(),
        ItemType::BlowgunNeedleBundle => vec!(),
        ItemType::Book => vec!(),
        ItemType::Bottle => vec!(),
        ItemType::BreastplateArmor => vec!(),
        ItemType::Bucket => vec!(),
        ItemType::Caltrop => vec!(),
        ItemType::CaltropBundle => vec!(),
        ItemType::Candle => vec!(),
        ItemType::Chain => vec!(),
        ItemType::ChainMailArmor => vec!(),
        ItemType::ChainShirtArmor => vec!(),
        ItemType::Chalk => vec!(),
        ItemType::Chest => vec!(),
        ItemType::ClimbersKit => vec!(),
        ItemType::ClothesCommon => vec!(),
        ItemType::ClothesCostume => vec!(),
        ItemType::ClothesFine => vec!(),
        ItemType::ClothesTraveler => vec!(),
        ItemType::Club => vec!(),
        ItemType::Coin => vec!(),
        ItemType::ComponentPouch => vec!(),
        ItemType::CrossbowBolt => vec!(),
        ItemType::CrossbowBoltBundle => vec!(),
        ItemType::CrossbowBoltCase => vec!(),
        ItemType::Crowbar => vec!(),
        ItemType::Crystal => vec!(),
        ItemType::Dagger => vec!(),
        ItemType::Dart => vec!(),
        ItemType::Drum => vec!(),
        ItemType::Emblem => vec!(),
        ItemType::FishingTackle => vec!(),
        ItemType::Flail => vec!(),
        ItemType::Flask => vec!(),
        ItemType::Flute => vec!(),
        ItemType::Glaive => vec!(),
        ItemType::GrapplingHook => vec!(),
        ItemType::Greataxe => vec!(),
        ItemType::Greatclub => vec!(),
        ItemType::Greatsword => vec!(),
        ItemType::Halberd => vec!(),
        ItemType::HalfPlateArmor => vec!(),
        ItemType::Hammer => vec!(),
        ItemType::Handaxe => vec!(),
        ItemType::HandCrossbow => vec!(),
        ItemType::HealersKit => vec!(),
        ItemType::HeavyCrossbow => vec!(),
        ItemType::HideArmor => vec!(),
        ItemType::Horn => vec!(),
        ItemType::Hourglass => vec!(),
        ItemType::HuntingTrap => vec!(),
        ItemType::Ink => vec!(),
        ItemType::Javelin => vec!(),
        ItemType::Jug => vec!(),
        ItemType::Ladder => vec!(),
        ItemType::Lamp => vec!(),
        ItemType::Lance => vec!(),
        ItemType::Lantern => vec!(),
        ItemType::LeatherArmor => vec!(),
        ItemType::LightCrossbow => vec!(),
        ItemType::LightHammer => vec!(),
        ItemType::Lock => vec!(),
        ItemType::Longbow => vec!(),
        ItemType::Longsword => vec!(),
        ItemType::Lute => vec!(),
        ItemType::Lyre => vec!(),
        ItemType::Mace => vec!(),
        ItemType::MagnifyingGlass => vec!(),
        ItemType::Manacles => vec!(),
        ItemType::MapCase => vec!(),
        ItemType::Maul => vec!(),
        ItemType::MessKit => vec!(),
        ItemType::Mirror => vec!(),
        ItemType::Morningstar => vec!(),
        ItemType::Net => vec!(),
        ItemType::Orb => vec!(),
        ItemType::PaddedArmor => vec!(),
        ItemType::PanFlute => vec!(),
        ItemType::Paper => vec!(),
        ItemType::Parchment => vec!(),
        ItemType::Pen => vec!(),
        ItemType::Perfume => vec!(),
        ItemType::Pickaxe => vec!(),
        ItemType::Pike => vec!(),
        ItemType::Piton => vec!(),
        ItemType::PlateArmor => vec!(),
        ItemType::Pole => vec!(),
        ItemType::Pot => vec!(),
        ItemType::Pouch => vec!(),
        ItemType::Quarterstaff => vec!(),
        ItemType::Quiver => vec!(),
        ItemType::Rapier => vec!(),
        ItemType::Rations => vec!(),
        ItemType::Reliquary => vec!(),
        ItemType::Ring => vec!(),
        ItemType::RingMailArmor => vec!(),
        ItemType::Robes => vec!(),
        ItemType::Rod => vec!(),
        ItemType::Rope => vec!(),
        ItemType::Sack => vec!(),
        ItemType::Scale => vec!(),
        ItemType::ScaleMailArmor => vec!(),
        ItemType::Scimitar => vec!(),
        ItemType::Shield => vec!(),
        ItemType::Shortbow => vec!(),
        ItemType::Shortsword => vec!(),
        ItemType::Shovel => vec!(),
        ItemType::Sickle => vec!(),
        ItemType::Sledgehammer => vec!(),
        ItemType::Sling => vec!(),
        ItemType::SlingBullet => vec!(),
        ItemType::SlingBulletBundle => vec!(),
        ItemType::Soap => vec!(),
        ItemType::Spear => vec!(),
        ItemType::Spellbook => vec!(),
        ItemType::Spike => vec!(),
        ItemType::SpikeBundle => vec!(),
        ItemType::SplintArmor => vec!(),
        ItemType::Spyglass => vec!(),
        ItemType::Staff => vec!(),
        ItemType::StuddedLeatherArmor => vec!(),
        ItemType::Tent => vec!(),
        ItemType::Tinderbox => vec!(),
        ItemType::Torch => vec!(),
        ItemType::Totem => vec!(),
        ItemType::Trident => vec!(),
        ItemType::Vial => vec!(),
        ItemType::Violin => vec!(),
        ItemType::Wand => vec!(),
        ItemType::Warhammer => vec!(),
        ItemType::WarPick => vec!(),
        ItemType::Waterskin => vec!(),
        ItemType::Wax => vec!(),
        ItemType::Whetstone => vec!(),
        ItemType::Whip => vec!(),
        ItemType::Whistle => vec!(),
    }
}

//TODO(ches) Weights
