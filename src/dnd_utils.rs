use strum::IntoEnumIterator; // 0.17.1
use strum_macros::EnumIter;

pub fn stat_to_modifier(stat: i32) -> i32 {
    ((stat - 10) as f32 / 2.0).floor() as i32  
}

// write tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stat_to_modifier() {
        assert_eq!(stat_to_modifier(1), -5);
        assert_eq!(stat_to_modifier(2), -4);
        assert_eq!(stat_to_modifier(3), -4);
        assert_eq!(stat_to_modifier(4), -3);
        assert_eq!(stat_to_modifier(5), -3);
        assert_eq!(stat_to_modifier(6), -2);
        assert_eq!(stat_to_modifier(7), -2);
        assert_eq!(stat_to_modifier(8), -1);
        assert_eq!(stat_to_modifier(9), -1);
        assert_eq!(stat_to_modifier(10), 0);
        assert_eq!(stat_to_modifier(11), 0);
        assert_eq!(stat_to_modifier(12), 1);
        assert_eq!(stat_to_modifier(13), 1);
        assert_eq!(stat_to_modifier(14), 2);
        assert_eq!(stat_to_modifier(15), 2);
        assert_eq!(stat_to_modifier(16), 3);
        assert_eq!(stat_to_modifier(17), 3);
        assert_eq!(stat_to_modifier(18), 4);
        assert_eq!(stat_to_modifier(19), 4);
        assert_eq!(stat_to_modifier(20), 5);
        assert_eq!(stat_to_modifier(21), 5);
        assert_eq!(stat_to_modifier(22), 6);
        assert_eq!(stat_to_modifier(23), 6);
        assert_eq!(stat_to_modifier(24), 7);
        assert_eq!(stat_to_modifier(25), 7);
        assert_eq!(stat_to_modifier(26), 8);
        assert_eq!(stat_to_modifier(27), 8);
        assert_eq!(stat_to_modifier(28), 9);
        assert_eq!(stat_to_modifier(29), 9);
        assert_eq!(stat_to_modifier(30), 10);
    }
}

pub fn proficiency_bonus(level: i32) -> i32 {
    match level {
        1..=4 => 2,
        5..=8 => 3,
        9..=12 => 4,
        13..=16 => 5,
        17..=20 => 6,
        _ => 0,
    }
}

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

pub struct Stat {
    pub stat: StatType,
    pub value: i32,
    pub modifier: i32,
    pub saving_throw_proficiency: bool,
}

impl Stat {
    pub fn new(stat: StatType, value: i32, save_proficiency: bool) -> Self {
        Self {
            stat,
            value,
            modifier: stat_to_modifier(value),
            saving_throw_proficiency: save_proficiency,
        }
    }

    pub fn set_value(&mut self, value: i32) {
        if value < 1 {
            self.value = 1;
        } else {
            self.value = value;
            self.modifier = stat_to_modifier(value);
        }
    }

    pub fn get_name(&self) -> String {
        self.stat.get_name()
    }

    pub fn get_value(&self) -> i32 {
        self.value
    }

    pub fn get_modifier(&self) -> i32 {
        self.modifier
    }

    pub fn set_save_proficiency(&mut self, proficiency: bool) {
        self.saving_throw_proficiency = proficiency;
    }

    pub fn get_saving_throw_proficiency(&self) -> bool {
        self.saving_throw_proficiency
    }
    
    pub fn add_one(&mut self) {
        self.set_value(self.value + 1)
    }

    pub fn subtract_one(&mut self) {
        self.set_value(self.value - 1)
    }
}

pub struct Stats {
    pub strength: Stat,
    pub dexterity: Stat,
    pub constitution: Stat,
    pub intelligence: Stat,
    pub wisdom: Stat,
    pub charisma: Stat,
}

impl Default for Stats {
    fn default() -> Self {
        Self {
            strength: Stat::new(StatType::Strength, 10, false),
            dexterity: Stat::new(StatType::Dexterity, 10, true),
            constitution: Stat::new(StatType::Constitution, 10, false),
            intelligence: Stat::new(StatType::Intelligence, 10, true),
            wisdom: Stat::new(StatType::Wisdom, 10, false),
            charisma: Stat::new(StatType::Charisma, 10, true),
        }
    }
}

impl Stats {
    pub fn test_stats() -> Self {
        let mut test_values = Stats::default();

        // set some values
        test_values.strength.set_value(18);
        test_values.dexterity.set_value(16);
        test_values.constitution.set_value(14);
        test_values.intelligence.set_value(12);
        test_values.wisdom.set_value(10);
        test_values.charisma.set_value(8);

        test_values
    }

    pub fn get_stat_mut(&mut self, stat_type: StatType) -> &mut Stat {
        match stat_type {
            StatType::Strength => &mut self.strength,
            StatType::Dexterity => &mut self.dexterity,
            StatType::Constitution => &mut self.constitution,
            StatType::Intelligence => &mut self.intelligence,
            StatType::Wisdom => &mut self.wisdom,
            StatType::Charisma => &mut self.charisma,
        }
    }

    pub fn get_stat(&self, stat_type: StatType) -> &Stat {
        match stat_type {
            StatType::Strength => &self.strength,
            StatType::Dexterity => &self.dexterity,
            StatType::Constitution => &self.constitution,
            StatType::Intelligence => &self.intelligence,
            StatType::Wisdom => &self.wisdom,
            StatType::Charisma => &self.charisma,
        }
    }

    pub fn get_stat_value(&self, stat_type: StatType) -> i32 {
        self.get_stat(stat_type).get_value()
    }

    pub fn get_stat_modifier(&self, stat_type: StatType) -> i32 {
        self.get_stat(stat_type).get_modifier()
    }

    pub fn get_stat_saving_throw_proficiency(&self, stat_type: StatType) -> bool {
        self.get_stat(stat_type).get_saving_throw_proficiency()
    }

    pub fn set_save_proficiency(&mut self, stat_type: StatType, proficiency: bool) {
        self.get_stat_mut(stat_type).set_save_proficiency(proficiency)
    }

    pub fn set_stat_value(&mut self, stat_type: StatType, value: i32) {
        self.get_stat_mut(stat_type).set_value(value)
    }

    pub fn update_modifiers(&mut self) {
        self.strength.modifier = stat_to_modifier(self.strength.value);
        self.dexterity.modifier = stat_to_modifier(self.dexterity.value);
        self.constitution.modifier = stat_to_modifier(self.constitution.value);
        self.intelligence.modifier = stat_to_modifier(self.intelligence.value);
        self.wisdom.modifier = stat_to_modifier(self.wisdom.value);
        self.charisma.modifier = stat_to_modifier(self.charisma.value);
    }
}

pub struct Dice {
    pub sides: i32,
    pub count: i32,
}

impl Dice {
    pub fn new(sides: i32, count: i32) -> Self {
        Self { sides, count }
    }

    pub fn max_roll(&self) -> i32 {
        self.sides * self.count
    }
}

pub struct DeathSaves {
    pub successes: i32,
    pub failures: i32,
}

impl Default for DeathSaves {
    fn default() -> Self {
        Self { successes: 0, failures: 0 }
    }
}

#[derive(Debug, EnumIter, Clone, Copy)]
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
        }
    }

}

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

pub struct Skill {
    base_ability: StatType,
    skill_type: SkillType,
    proficiency: bool,
    expertise: bool,
    other_bonus: i32,
}

pub struct Skills {
    acrobatics: Skill,
    animal_handling: Skill,
    arcana: Skill,
    athletics: Skill,
    deception: Skill,
    history: Skill,
    insight: Skill,
    intimidation: Skill,
    investigation: Skill,
    medicine: Skill,
    nature: Skill,
    perception: Skill,
    performance: Skill,
    persuasion: Skill,
    religion: Skill,
    sleight_of_hand: Skill,
    stealth: Skill,
    survival: Skill,
}

impl Default for Skills {
    fn default() -> Self {
        Self {
            acrobatics: Skill {
                base_ability: StatType::Dexterity,
                skill_type: SkillType::Acrobatics,
                proficiency: false,
                expertise: false,
                other_bonus: 0,
            },
            animal_handling: Skill {
                base_ability: StatType::Wisdom,
                skill_type: SkillType::AnimalHandling,
                proficiency: false,
                expertise: false,
                other_bonus: 0,
            },
            arcana: Skill {
                base_ability: StatType::Intelligence,
                skill_type: SkillType::Arcana,
                proficiency: false,
                expertise: false,
                other_bonus: 0,
            },
            athletics: Skill {
                base_ability: StatType::Strength,
                skill_type: SkillType::Athletics,
                proficiency: false,
                expertise: false,
                other_bonus: 0,
            },
            deception: Skill {
                base_ability: StatType::Charisma,
                skill_type: SkillType::Deception,
                proficiency: false,
                expertise: false,
                other_bonus: 0,
            },
            history: Skill {
                base_ability: StatType::Intelligence,
                skill_type: SkillType::History,
                proficiency: false,
                expertise: false,
                other_bonus: 0,
            },
            insight: Skill {
                base_ability: StatType::Wisdom,
                skill_type: SkillType::Insight,
                proficiency: false,
                expertise: false,
                other_bonus: 0,
            },
            intimidation: Skill {
                base_ability: StatType::Charisma,
                skill_type: SkillType::Intimidation,
                proficiency: false,
                expertise: false,
                other_bonus: 0,
            },
            investigation: Skill {
                base_ability: StatType::Intelligence,
                skill_type: SkillType::Investigation,
                proficiency: false,
                expertise: false,
                other_bonus: 0,
            },
            medicine: Skill {
                base_ability: StatType::Wisdom,
                skill_type: SkillType::Medicine,
                proficiency: false,
                expertise: false,
                other_bonus: 0,
            },
            nature: Skill {
                base_ability: StatType::Intelligence,
                skill_type: SkillType::Nature,
                proficiency: false,
                expertise: false,
                other_bonus: 0,
            },
            perception: Skill {
                base_ability: StatType::Wisdom,
                skill_type: SkillType::Perception,
                proficiency: false,
                expertise: false,
                other_bonus: 0,
            },
            performance: Skill {
                base_ability: StatType::Charisma,
                skill_type: SkillType::Performance,
                proficiency: false,
                expertise: false,
                other_bonus: 0,
            },
            persuasion: Skill {
                base_ability: StatType::Charisma,
                skill_type: SkillType::Persuasion,
                proficiency: false,
                expertise: false,
                other_bonus: 0,
            },
            religion: Skill {
                base_ability: StatType::Intelligence,
                skill_type: SkillType::Religion,
                proficiency: false,
                expertise: false,
                other_bonus: 0,
            },
            sleight_of_hand: Skill {
                base_ability: StatType::Dexterity,
                skill_type: SkillType::SleightOfHand,
                proficiency: false,
                expertise: false,
                other_bonus: 0,
            },
            stealth: Skill {
                base_ability: StatType::Dexterity,
                skill_type: SkillType::Stealth,
                proficiency: false,
                expertise: false,
                other_bonus: 0,
            },
            survival: Skill {
                base_ability: StatType::Wisdom,
                skill_type: SkillType::Survival,
                proficiency: false,
                expertise: false,
                other_bonus: 0,
            },
        }
    }
}

impl Skills {

    pub fn test_skills() -> Self {
        let mut test_values = Skills::default();

        // add proficiency to some skills
        test_values.acrobatics.proficiency = true;
        test_values.animal_handling.proficiency = true;
        test_values.arcana.proficiency = true;
        test_values.athletics.proficiency = true;
        test_values.deception.proficiency = true;

        test_values.athletics.expertise = true;
        test_values.deception.expertise = true;

        test_values.religion.other_bonus = 10;

        test_values
    }

    pub fn get_skill(&self, skill_type: SkillType) -> &Skill {
        match skill_type {
            SkillType::Acrobatics => & self.acrobatics,
            SkillType::AnimalHandling => & self.animal_handling,
            SkillType::Arcana => & self.arcana,
            SkillType::Athletics => & self.athletics,
            SkillType::Deception => & self.deception,
            SkillType::History => & self.history,
            SkillType::Insight => & self.insight,
            SkillType::Intimidation => & self.intimidation,
            SkillType::Investigation => & self.investigation,
            SkillType::Medicine => & self.medicine,
            SkillType::Nature => & self.nature,
            SkillType::Perception => & self.perception,
            SkillType::Performance => & self.performance,
            SkillType::Persuasion => & self.persuasion,
            SkillType::Religion => & self.religion,
            SkillType::SleightOfHand => & self.sleight_of_hand,
            SkillType::Stealth => & self.stealth,
            SkillType::Survival => & self.survival,
        }
    }

    pub fn get_skill_proficiency(&self, skill_type: SkillType) -> bool {
        self.get_skill(skill_type).proficiency
    }

    pub fn get_skill_expertise(&self, skill_type: SkillType) -> bool {
        self.get_skill(skill_type).expertise
    }

    pub fn get_skill_other_bonus(&self, skill_type: SkillType) -> i32 {
        self.get_skill(skill_type).other_bonus
    }

    pub fn get_base_stat(&self, skill_type: SkillType) -> StatType {
        self.get_skill(skill_type).base_ability
    }
}