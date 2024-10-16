use std::str::FromStr;

use strum_macros::EnumIter;

#[derive(Debug, PartialEq, Eq)]
pub struct FromStringError;

pub enum AdvantageStatus {
    Advantage,
    Normal,
    Disadvantage
}

#[derive(Debug, EnumIter)]
pub enum Alignment {
    ChaoticEvil,
    ChaoticGood,
    ChaoticNeutral,
    LawfulEvil,
    LawfulGood,
    LawfulNeutral,
    Neutral,
    NeutralEvil,
    NeutralGood,
}


impl ToString for Alignment {
    fn to_string(&self) -> String {
        match self {
            Alignment::ChaoticEvil => String::from("Chaotic Evil"),
            Alignment::ChaoticGood => String::from("Chaotic Good"),
            Alignment::ChaoticNeutral => String::from("Chaotic Neutral"),
            Alignment::LawfulEvil => String::from("Lawful Evil"),
            Alignment::LawfulGood => String::from("Lawful Good"),
            Alignment::LawfulNeutral => String::from("Lawful Neutral"),
            Alignment::Neutral => String::from("Neutral"),
            Alignment::NeutralEvil => String::from("Neutral Evil"),
            Alignment::NeutralGood => String::from("Neutral Good"),
        }
    }
}

impl FromStr for Alignment {
    type Err = FromStringError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Chaotic Evil" => Ok(Alignment::ChaoticEvil),
            "Chaotic Good" => Ok(Alignment::ChaoticGood),
            "Chaotic Neutral" => Ok(Alignment::ChaoticNeutral),
            "Lawful Evil" => Ok(Alignment::LawfulEvil),
            "Lawful Good" => Ok(Alignment::LawfulGood),
            "Lawful Neutral" => Ok(Alignment::LawfulNeutral),
            "Neutral" => Ok(Alignment::Neutral),
            "Neutral Evil" => Ok(Alignment::NeutralEvil),
            "Neutral Good" => Ok(Alignment::NeutralGood),
            _ => Err(FromStringError)
        }
    }
}

#[derive(Debug, EnumIter)]
pub enum Class {
    Barbarian,
    Bard,
    Cleric,
    Druid,
    Fighter,
    Monk,
    Paladin,
    Ranger,
    Rogue,
    Sorcerer,
    Warlock,
    Wizard,
}

impl ToString for Class {
    fn to_string(&self) -> String {
        match self {
            Class::Barbarian => String::from("Barbarian"),
            Class::Bard => String::from("Bard"),
            Class::Cleric => String::from("Cleric"),
            Class::Druid => String::from("Druid"),
            Class::Fighter => String::from("Fighter"),
            Class::Monk => String::from("Monk"),
            Class::Paladin => String::from("Paladin"),
            Class::Ranger => String::from("Ranger"),
            Class::Rogue => String::from("Rogue"),
            Class::Sorcerer => String::from("Sorcerer"),
            Class::Warlock => String::from("Warlock"),
            Class::Wizard => String::from("Wizard"),
        }
    }
}

impl FromStr for Class {
    type Err = FromStringError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Barbarian" => Ok(Class::Barbarian),
            "Bard" => Ok(Class::Bard),
            "Cleric" => Ok(Class::Cleric),
            "Druid" => Ok(Class::Druid),
            "Fighter" => Ok(Class::Fighter),
            "Monk" => Ok(Class::Monk),
            "Paladin" => Ok(Class::Paladin),
            "Ranger" => Ok(Class::Ranger),
            "Rogue" => Ok(Class::Rogue),
            "Sorcerer" => Ok(Class::Sorcerer),
            "Warlock" => Ok(Class::Warlock),
            "Wizard" => Ok(Class::Wizard),
            _ => Err(FromStringError)
        }
    }
}

pub enum DamageType {
    Acid,
    Bludgeoning,
    Cold,
    Fire,
    Force,
    Lightning,
    Necrotic,
    Piercing,
    Poison,
    Psychic,
    Radiant,
    Slashing,
    Thunder,
}

#[derive(Debug, EnumIter)]
pub enum Race {
    Dragonborn,
    Dwarf,
    Elf,
    Gnome,
    HalfElf,
    HalfOrc,
    Halfling,
    Human,
    Tiefling
}

impl ToString for Race {
    fn to_string(&self) -> String {
        match self {
            Race::Dragonborn => String::from("Dragonborn"),
            Race::Dwarf => String::from("Dwarf"),
            Race::Elf => String::from("Elf"),
            Race::Gnome => String::from("Gnome"),
            Race::HalfElf => String::from("Half Elf"),
            Race::HalfOrc => String::from("Half Orc"),
            Race::Halfling => String::from("Halfling"),
            Race::Human => String::from("Human"),
            Race::Tiefling => String::from("Tiefling"),
        }
    }
}

impl FromStr for Race {
    type Err = FromStringError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Dragonborn" => Ok(Race::Dragonborn),
            "Dwarf" => Ok(Race::Dwarf),
            "Elf" => Ok(Race::Elf),
            "Gnome" => Ok(Race::Gnome),
            "Half Elf" => Ok(Race::HalfElf),
            "Half Orc" => Ok(Race::HalfOrc),
            "Halfling" => Ok(Race::Halfling),
            "Human" => Ok(Race::Human),
            "Tiefling" => Ok(Race::Tiefling),
            _ => Err(FromStringError)
        }
    }
}

pub enum Size {
    Tiny,
    Small,
    Medium,
    Large,
    Huge,
    Gargantuan
}

pub enum Skill {
    Acrobatics,
    AnimalHandling,
    Arcana,
    Athletics,
    Deception,
    History,
    Insight,
    Intimidation,
    Investigation,
    Medicine,
    Nature,
    Perception,
    Performance,
    Persuasion,
    Religion,
    SleightOfHand,
    Stealth,
    Survival
}

pub enum Stat {
    Charisma,
    Constitution,
    Dexterity,
    Intelligence,
    Strength,
    Wisdom,
}

pub const NUMBER_OF_STATS: usize = 6;

pub struct Stats {
    pub charisma: u8,
    pub constitution: u8,
    pub dexterity: u8,
    pub intelligence: u8,
    pub strength: u8,
    pub wisdom: u8
}

pub fn carrying_capacity(strength: u8, size: Size) -> u16 {
    let total: u16 = strength as u16 * 15;

    return match size {
        Size::Tiny=> total / 2,
        Size::Small => total,
        Size::Medium => total,
        Size::Large => total * 2,
        Size::Huge => total * 4,
        Size::Gargantuan => total * 8
    }
}

pub fn modifier(ability: u8) -> i8 {
    return (ability as i8 / 2) - 5;
}

pub fn move_capacity(strength: u8, size: Size) -> u16 {
    // NOTE(ches) we directly calculate here to prevent truncation issues
    // but it's just 2 * carrying capacity
    let total: u16 = strength as u16 * 15;

    return match size {
        Size::Tiny=> total,
        Size::Small => total * 2,
        Size::Medium => total * 2,
        Size::Large => total * 4,
        Size::Huge => total * 8,
        Size::Gargantuan => total * 16
    }
}

pub fn passive_score(modifiers: i8, advantage: AdvantageStatus) -> i8 {
    let advantage_mod: i8 = match advantage {
        AdvantageStatus::Advantage => 5,
        AdvantageStatus::Normal=> 0,
        AdvantageStatus::Disadvantage => -5
    };

    return 10 + modifiers + advantage_mod;
}

const POINT_BUY_MAX: u8 = 15;
pub const POINT_BUY_MIN: u8 = 8;
pub const POINT_BUY_POINTS: u8 = 27;

fn point_buy_cost(stat: u8) -> u8 {
    return if stat >= 14 { 2 } else { 1 }
}

pub fn point_buy_attempt_decrease(stat: &mut u8, points_left: &mut u8) {
    if *stat <= POINT_BUY_MIN {
        return;
    }
    let cost = point_buy_cost(*stat);
    *points_left += cost;
    *stat -= 1;
}

pub fn point_buy_attempt_increase(stat: &mut u8, points_left: &mut u8) {
    if *stat >= POINT_BUY_MAX {
        return;
    }
    let cost = point_buy_cost(*stat + 1);
    if cost > *points_left {
        return;
    }
    *points_left -= cost;
    *stat += 1;
}

pub fn skill_stat(skill: Skill) -> Stat {
    return match skill {
        Skill::Acrobatics => Stat::Dexterity,
        Skill::AnimalHandling => Stat::Wisdom,
        Skill::Arcana => Stat::Intelligence,
        Skill::Athletics => Stat::Strength,
        Skill::Deception => Stat::Charisma,
        Skill::History => Stat::Intelligence,
        Skill::Insight => Stat::Wisdom,
        Skill::Intimidation => Stat::Charisma,
        Skill::Investigation => Stat::Intelligence,
        Skill::Medicine => Stat::Wisdom,
        Skill::Nature => Stat::Intelligence,
        Skill::Perception => Stat::Wisdom,
        Skill::Performance => Stat::Charisma,
        Skill::Persuasion => Stat::Charisma,
        Skill::Religion => Stat::Intelligence,
        Skill::SleightOfHand => Stat::Dexterity,
        Skill::Stealth => Stat::Dexterity,
        Skill::Survival => Stat::Wisdom,
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_modifier() {
        assert_eq!(modifier(1), -5);
        assert_eq!(modifier(2), -4);
        assert_eq!(modifier(3), -4);
        assert_eq!(modifier(4), -3);
        assert_eq!(modifier(5), -3);
        assert_eq!(modifier(6), -2);
        assert_eq!(modifier(7), -2);
        assert_eq!(modifier(8), -1);
        assert_eq!(modifier(9), -1);
        assert_eq!(modifier(10), 0);
        assert_eq!(modifier(11), 0);
        assert_eq!(modifier(12), 1);
        assert_eq!(modifier(13), 1);
        assert_eq!(modifier(14), 2);
        assert_eq!(modifier(15), 2);
        assert_eq!(modifier(16), 3);
        assert_eq!(modifier(17), 3);
        assert_eq!(modifier(18), 4);
        assert_eq!(modifier(19), 4);
        assert_eq!(modifier(20), 5);
        assert_eq!(modifier(21), 5);
        assert_eq!(modifier(22), 6);
        assert_eq!(modifier(23), 6);
        assert_eq!(modifier(24), 7);
        assert_eq!(modifier(25), 7);
        assert_eq!(modifier(26), 8);
        assert_eq!(modifier(27), 8);
        assert_eq!(modifier(28), 9);
        assert_eq!(modifier(29), 9);
        assert_eq!(modifier(30), 10);
    }
    
    #[test]
    fn test_passive_score() {
        assert_eq!(passive_score(0, AdvantageStatus::Normal), 10);
        assert_eq!(passive_score(1, AdvantageStatus::Normal), 11);
        assert_eq!(passive_score(0, AdvantageStatus::Advantage), 15);
        assert_eq!(passive_score(0, AdvantageStatus::Disadvantage), 5);
        assert_eq!(passive_score(-1, AdvantageStatus::Normal), 9);
        assert_eq!(passive_score(0, AdvantageStatus::Disadvantage), 5);
        assert_eq!(passive_score(-5, AdvantageStatus::Disadvantage), 0);
        assert_eq!(passive_score(10, AdvantageStatus::Advantage), 25);
    }

    #[test]
    fn test_carrying_capacity() {
        assert_eq!(carrying_capacity(1, Size::Tiny), 7);
        assert_eq!(carrying_capacity(3, Size::Tiny), 22);
        assert_eq!(carrying_capacity(10, Size::Tiny), 75);
        assert_eq!(carrying_capacity(15, Size::Tiny), 112);
        assert_eq!(carrying_capacity(20, Size::Tiny), 150);
        assert_eq!(carrying_capacity(30, Size::Tiny), 225);

        assert_eq!(carrying_capacity(1, Size::Small), 15);
        assert_eq!(carrying_capacity(3, Size::Small), 45);
        assert_eq!(carrying_capacity(10, Size::Small), 150);
        assert_eq!(carrying_capacity(15, Size::Small), 225);
        assert_eq!(carrying_capacity(20, Size::Small), 300);
        assert_eq!(carrying_capacity(30, Size::Small), 450);

        assert_eq!(carrying_capacity(1, Size::Medium), 15);
        assert_eq!(carrying_capacity(3, Size::Medium), 45);
        assert_eq!(carrying_capacity(10, Size::Medium), 150);
        assert_eq!(carrying_capacity(15, Size::Medium), 225);
        assert_eq!(carrying_capacity(20, Size::Medium), 300);
        assert_eq!(carrying_capacity(30, Size::Medium), 450);

        assert_eq!(carrying_capacity(1, Size::Large), 30);
        assert_eq!(carrying_capacity(3, Size::Large), 90);
        assert_eq!(carrying_capacity(10, Size::Large), 300);
        assert_eq!(carrying_capacity(15, Size::Large), 450);
        assert_eq!(carrying_capacity(20, Size::Large), 600);
        assert_eq!(carrying_capacity(30, Size::Large), 900);

        assert_eq!(carrying_capacity(1, Size::Huge), 60);
        assert_eq!(carrying_capacity(3, Size::Huge), 180);
        assert_eq!(carrying_capacity(10, Size::Huge), 600);
        assert_eq!(carrying_capacity(15, Size::Huge), 900);
        assert_eq!(carrying_capacity(20, Size::Huge), 1200);
        assert_eq!(carrying_capacity(30, Size::Huge), 1800);

        assert_eq!(carrying_capacity(1, Size::Gargantuan), 120);
        assert_eq!(carrying_capacity(3, Size::Gargantuan), 360);
        assert_eq!(carrying_capacity(10, Size::Gargantuan), 1200);
        assert_eq!(carrying_capacity(15, Size::Gargantuan), 1800);
        assert_eq!(carrying_capacity(20, Size::Gargantuan), 2400);
        assert_eq!(carrying_capacity(30, Size::Gargantuan), 3600);

    }

    #[test]
    fn test_move_capacity() {
        assert_eq!(move_capacity(1, Size::Tiny), 15);
        assert_eq!(move_capacity(3, Size::Tiny), 45);
        assert_eq!(move_capacity(10, Size::Tiny), 150);
        assert_eq!(move_capacity(15, Size::Tiny), 225);
        assert_eq!(move_capacity(20, Size::Tiny), 300);
        assert_eq!(move_capacity(30, Size::Tiny), 450);

        assert_eq!(move_capacity(1, Size::Small), 30);
        assert_eq!(move_capacity(3, Size::Small), 90);
        assert_eq!(move_capacity(10, Size::Small), 300);
        assert_eq!(move_capacity(15, Size::Small), 450);
        assert_eq!(move_capacity(20, Size::Small), 600);
        assert_eq!(move_capacity(30, Size::Small), 900);

        assert_eq!(move_capacity(1, Size::Medium), 30);
        assert_eq!(move_capacity(3, Size::Medium), 90);
        assert_eq!(move_capacity(10, Size::Medium), 300);
        assert_eq!(move_capacity(15, Size::Medium), 450);
        assert_eq!(move_capacity(20, Size::Medium), 600);
        assert_eq!(move_capacity(30, Size::Medium), 900);

        assert_eq!(move_capacity(1, Size::Large), 60);
        assert_eq!(move_capacity(3, Size::Large), 180);
        assert_eq!(move_capacity(10, Size::Large), 600);
        assert_eq!(move_capacity(15, Size::Large), 900);
        assert_eq!(move_capacity(20, Size::Large), 1200);
        assert_eq!(move_capacity(30, Size::Large), 1800);

        assert_eq!(move_capacity(1, Size::Huge), 120);
        assert_eq!(move_capacity(3, Size::Huge), 360);
        assert_eq!(move_capacity(10, Size::Huge), 1200);
        assert_eq!(move_capacity(15, Size::Huge), 1800);
        assert_eq!(move_capacity(20, Size::Huge), 2400);
        assert_eq!(move_capacity(30, Size::Huge), 3600);

        assert_eq!(move_capacity(1, Size::Gargantuan), 240);
        assert_eq!(move_capacity(3, Size::Gargantuan), 720);
        assert_eq!(move_capacity(10, Size::Gargantuan), 2400);
        assert_eq!(move_capacity(15, Size::Gargantuan), 3600);
        assert_eq!(move_capacity(20, Size::Gargantuan), 4800);
        assert_eq!(move_capacity(30, Size::Gargantuan), 7200);

    }

}