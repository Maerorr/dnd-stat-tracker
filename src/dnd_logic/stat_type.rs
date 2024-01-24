use serde::{Deserialize, Serialize};
use strum_macros::EnumIter;

use crate::app::*;

#[derive(Debug, EnumIter, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum StatType {
    Strength,
    Dexterity,
    Constitution,
    Intelligence,
    Wisdom,
    Charisma,
}

impl StatType {
    pub fn get_name(&self) -> String {
        match self {
            StatType::Strength => String::from("Strength"),
            StatType::Dexterity => String::from("Dexterity"),
            StatType::Constitution => String::from("Constitution"),
            StatType::Intelligence => String::from("Intelligence"),
            StatType::Wisdom => String::from("Wisdom"),
            StatType::Charisma => String::from("Charisma"),
        }
    }

    pub fn get_short_name(&self) -> String {
        match self {
            StatType::Strength => String::from("STR"),
            StatType::Dexterity => String::from("DEX"),
            StatType::Constitution => String::from("CON"),
            StatType::Intelligence => String::from("INT"),
            StatType::Wisdom => String::from("WIS"),
            StatType::Charisma => String::from("CHA"),
        }
    }

    pub fn get_stat_color(&self) -> egui::Color32 {
        match self {
            StatType::Strength => STRENGTH_COLOR,
            StatType::Dexterity => DEXTERITY_COLOR,
            StatType::Constitution => CONSTITUTION_COLOR,
            StatType::Intelligence => INTELLIGENCE_COLOR,
            StatType::Wisdom => WISDOM_COLOR,
            StatType::Charisma => CHARISMA_COLOR,
        }
    }
}