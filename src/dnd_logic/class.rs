use serde::{Deserialize, Serialize};
use strum_macros::EnumIter;

use super::{dice::Dice, stat_type::StatType};

#[derive(Debug, EnumIter, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
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
    Artificer,
    BloodHunter,
    Gunslinger,
}

impl Class {
    pub fn get_hit_dice(&self) -> Dice {
        match self {
            Class::Barbarian => Dice::new(12, 1),
            Class::Bard => Dice::new(8, 1),
            Class::Cleric => Dice::new(8, 1),
            Class::Druid => Dice::new(8, 1),
            Class::Fighter => Dice::new(10, 1),
            Class::Monk => Dice::new(8, 1),
            Class::Paladin => Dice::new(10, 1),
            Class::Ranger => Dice::new(10, 1),
            Class::Rogue => Dice::new(8, 1),
            Class::Sorcerer => Dice::new(6, 1),
            Class::Warlock => Dice::new(8, 1),
            Class::Wizard => Dice::new(6, 1),
            Class::Artificer => Dice::new(8, 1),
            Class::BloodHunter => Dice::new(10, 1),
            Class::Gunslinger => Dice::new(10, 1),
        }
    }

    pub fn get_name(&self) -> String {
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
            Class::Artificer => String::from("Artificer"),
            Class::BloodHunter => String::from("Blood Hunter"),
            Class::Gunslinger => String::from("Gunslinger"),
        }
    }

    pub fn get_spellcasting_ability(&self) -> Option<StatType> {
        match self {
            Class::Barbarian => None,
            Class::Bard => Some(StatType::Charisma),
            Class::Cleric => Some(StatType::Wisdom),
            Class::Druid => Some(StatType::Wisdom),
            Class::Fighter => None,
            Class::Monk => None,
            Class::Paladin => Some(StatType::Charisma),
            Class::Ranger => Some(StatType::Wisdom),
            Class::Rogue => None,
            Class::Sorcerer => Some(StatType::Charisma),
            Class::Warlock => Some(StatType::Charisma),
            Class::Wizard => Some(StatType::Intelligence),
            Class::Artificer => Some(StatType::Intelligence),
            Class::BloodHunter => Some(StatType::Wisdom),
            Class::Gunslinger => Some(StatType::Wisdom),
        }
    }

}