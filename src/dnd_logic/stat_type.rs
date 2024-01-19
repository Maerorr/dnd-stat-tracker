use strum_macros::EnumIter;

#[derive(Debug, EnumIter, Clone, Copy)]
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
}