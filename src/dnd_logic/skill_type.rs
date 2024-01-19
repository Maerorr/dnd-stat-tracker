use strum_macros::EnumIter;

use super::stat_type::StatType;

#[derive(Debug, EnumIter, Clone, Copy)]
pub enum SkillType {
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
    Survival,
}

impl SkillType {
    pub fn get_name(&self) -> String {
        match self {
            SkillType::Acrobatics => String::from("Acrobatics"),
            SkillType::AnimalHandling => String::from("Animal Handling"),
            SkillType::Arcana => String::from("Arcana"),
            SkillType::Athletics => String::from("Athletics"),
            SkillType::Deception => String::from("Deception"),
            SkillType::History => String::from("History"),
            SkillType::Insight => String::from("Insight"),
            SkillType::Intimidation => String::from("Intimidation"),
            SkillType::Investigation => String::from("Investigation"),
            SkillType::Medicine => String::from("Medicine"),
            SkillType::Nature => String::from("Nature"),
            SkillType::Perception => String::from("Perception"),
            SkillType::Performance => String::from("Performance"),
            SkillType::Persuasion => String::from("Persuasion"),
            SkillType::Religion => String::from("Religion"),
            SkillType::SleightOfHand => String::from("Sleight of Hand"),
            SkillType::Stealth => String::from("Stealth"),
            SkillType::Survival => String::from("Survival"),
        }
    }

    pub fn get_base_stat(&self) -> StatType {
        match self {
            SkillType::Acrobatics => StatType::Dexterity,
            SkillType::AnimalHandling => StatType::Wisdom,
            SkillType::Arcana => StatType::Intelligence,
            SkillType::Athletics => StatType::Strength,
            SkillType::Deception => StatType::Charisma,
            SkillType::History => StatType::Intelligence,
            SkillType::Insight => StatType::Wisdom,
            SkillType::Intimidation => StatType::Charisma,
            SkillType::Investigation => StatType::Intelligence,
            SkillType::Medicine => StatType::Wisdom,
            SkillType::Nature => StatType::Intelligence,
            SkillType::Perception => StatType::Wisdom,
            SkillType::Performance => StatType::Charisma,
            SkillType::Persuasion => StatType::Charisma,
            SkillType::Religion => StatType::Intelligence,
            SkillType::SleightOfHand => StatType::Dexterity,
            SkillType::Stealth => StatType::Dexterity,
            SkillType::Survival => StatType::Wisdom,
        }
    }
}