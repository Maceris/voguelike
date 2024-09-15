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
    Wizard
}

pub enum Size {
    Tiny,
    Small,
    Medium,
    Large,
    Huge,
    Gargantuan
}

pub enum AdvantageStatus {
    Advantage,
    Normal,
    Disadvantage
}

pub struct Stats {
    charisma: u8,
    constitution: u8,
    dexterity: u8,
    intelligence: u8,
    strength: u8,
    wisdom: u8
}

pub fn modifier(ability: u8) -> i8 {
    return (ability as i8 / 2) - 5;
}

pub fn passive_score(modifiers: i8, advantage: AdvantageStatus) -> i8 {
    let advantage_mod: i8 = match advantage {
        AdvantageStatus::Advantage => 5,
        AdvantageStatus::Normal=> 0,
        AdvantageStatus::Disadvantage => -5
    };

    return 10 + modifiers + advantage_mod;
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