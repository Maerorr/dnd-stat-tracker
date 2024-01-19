use strum::IntoEnumIterator; // 0.17.1
use strum_macros::EnumIter;

pub fn stat_to_modifier(stat: i32) -> i32 {
    (stat - 10) / 2
}

#[derive(Debug, EnumIter)]
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

    pub fn update(&mut self, value: i32) {
        self.value = value;
        self.modifier = stat_to_modifier(value);
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
    
    pub fn add_one(&mut self) {
        self.value += 1;
        self.modifier = stat_to_modifier(self.value);
    }

    pub fn subtract_one(&mut self) {
        self.value -= 1;
        self.modifier = stat_to_modifier(self.value);
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
    pub fn get_stat(&mut self, stat_type: StatType) -> &mut Stat {
        match stat_type {
            StatType::Strength => &mut self.strength,
            StatType::Dexterity => &mut self.dexterity,
            StatType::Constitution => &mut self.constitution,
            StatType::Intelligence => &mut self.intelligence,
            StatType::Wisdom => &mut self.wisdom,
            StatType::Charisma => &mut self.charisma,
        }
    }

    pub fn get_stat_value(&self, stat_type: StatType) -> i32 {
        match stat_type {
            StatType::Strength => self.strength.value,
            StatType::Dexterity => self.dexterity.value,
            StatType::Constitution => self.constitution.value,
            StatType::Intelligence => self.intelligence.value,
            StatType::Wisdom => self.wisdom.value,
            StatType::Charisma => self.charisma.value,
        }
    }

    pub fn get_stat_modifier(&self, stat_type: StatType) -> i32 {
        match stat_type {
            StatType::Strength => self.strength.modifier,
            StatType::Dexterity => self.dexterity.modifier,
            StatType::Constitution => self.constitution.modifier,
            StatType::Intelligence => self.intelligence.modifier,
            StatType::Wisdom => self.wisdom.modifier,
            StatType::Charisma => self.charisma.modifier,
        }
    }

    pub fn get_stat_saving_throw_proficiency(&self, stat_type: StatType) -> bool {
        match stat_type {
            StatType::Strength => self.strength.saving_throw_proficiency,
            StatType::Dexterity => self.dexterity.saving_throw_proficiency,
            StatType::Constitution => self.constitution.saving_throw_proficiency,
            StatType::Intelligence => self.intelligence.saving_throw_proficiency,
            StatType::Wisdom => self.wisdom.saving_throw_proficiency,
            StatType::Charisma => self.charisma.saving_throw_proficiency,
        }
    }

    pub fn set_save_proficiency(&mut self, stat_type: StatType, proficiency: bool) {
        match stat_type {
            StatType::Strength => self.strength.saving_throw_proficiency = proficiency,
            StatType::Dexterity => self.dexterity.saving_throw_proficiency = proficiency,
            StatType::Constitution => self.constitution.saving_throw_proficiency = proficiency,
            StatType::Intelligence => self.intelligence.saving_throw_proficiency = proficiency,
            StatType::Wisdom => self.wisdom.saving_throw_proficiency = proficiency,
            StatType::Charisma => self.charisma.saving_throw_proficiency = proficiency,
        }
    }

    pub fn set_stat_value(&mut self, stat_type: StatType, value: i32) {
        match stat_type {
            StatType::Strength => self.strength.update(value),
            StatType::Dexterity => self.dexterity.update(value),
            StatType::Constitution => self.constitution.update(value),
            StatType::Intelligence => self.intelligence.update(value),
            StatType::Wisdom => self.wisdom.update(value),
            StatType::Charisma => self.charisma.update(value),
        }
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
}

struct Skill {
    base_ability: StatType,
    skill_type: SkillType,
    proficiency: bool,
    expertise: bool,
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
                proficiency: true,
                expertise: false,
            },
            animal_handling: Skill {
                base_ability: StatType::Wisdom,
                skill_type: SkillType::AnimalHandling,
                proficiency: false,
                expertise: false,
            },
            arcana: Skill {
                base_ability: StatType::Intelligence,
                skill_type: SkillType::Arcana,
                proficiency: true,
                expertise: false,
            },
            athletics: Skill {
                base_ability: StatType::Strength,
                skill_type: SkillType::Athletics,
                proficiency: false,
                expertise: false,
            },
            deception: Skill {
                base_ability: StatType::Charisma,
                skill_type: SkillType::Deception,
                proficiency: true,
                expertise: false,
            },
            history: Skill {
                base_ability: StatType::Intelligence,
                skill_type: SkillType::History,
                proficiency: false,
                expertise: false,
            },
            insight: Skill {
                base_ability: StatType::Wisdom,
                skill_type: SkillType::Insight,
                proficiency: false,
                expertise: false,
            },
            intimidation: Skill {
                base_ability: StatType::Charisma,
                skill_type: SkillType::Intimidation,
                proficiency: true,
                expertise: false,
            },
            investigation: Skill {
                base_ability: StatType::Intelligence,
                skill_type: SkillType::Investigation,
                proficiency: false,
                expertise: false,
            },
            medicine: Skill {
                base_ability: StatType::Wisdom,
                skill_type: SkillType::Medicine,
                proficiency: true,
                expertise: false,
            },
            nature: Skill {
                base_ability: StatType::Intelligence,
                skill_type: SkillType::Nature,
                proficiency: true,
                expertise: true,
            },
            perception: Skill {
                base_ability: StatType::Wisdom,
                skill_type: SkillType::Perception,
                proficiency: false,
                expertise: false,
            },
            performance: Skill {
                base_ability: StatType::Charisma,
                skill_type: SkillType::Performance,
                proficiency: false,
                expertise: false,
            },
            persuasion: Skill {
                base_ability: StatType::Charisma,
                skill_type: SkillType::Persuasion,
                proficiency: false,
                expertise: false,
            },
            religion: Skill {
                base_ability: StatType::Intelligence,
                skill_type: SkillType::Religion,
                proficiency: false,
                expertise: false,
            },
            sleight_of_hand: Skill {
                base_ability: StatType::Dexterity,
                skill_type: SkillType::SleightOfHand,
                proficiency: false,
                expertise: false,
            },
            stealth: Skill {
                base_ability: StatType::Dexterity,
                skill_type: SkillType::Stealth,
                proficiency: true,
                expertise: false,
            },
            survival: Skill {
                base_ability: StatType::Wisdom,
                skill_type: SkillType::Survival,
                proficiency: false,
                expertise: true,
            },
        }
    }
}

impl Skills {
    pub fn get_skill(&mut self, skill_type: SkillType) -> &mut Skill {
        match skill_type {
            SkillType::Acrobatics => &mut self.acrobatics,
            SkillType::AnimalHandling => &mut self.animal_handling,
            SkillType::Arcana => &mut self.arcana,
            SkillType::Athletics => &mut self.athletics,
            SkillType::Deception => &mut self.deception,
            SkillType::History => &mut self.history,
            SkillType::Insight => &mut self.insight,
            SkillType::Intimidation => &mut self.intimidation,
            SkillType::Investigation => &mut self.investigation,
            SkillType::Medicine => &mut self.medicine,
            SkillType::Nature => &mut self.nature,
            SkillType::Perception => &mut self.perception,
            SkillType::Performance => &mut self.performance,
            SkillType::Persuasion => &mut self.persuasion,
            SkillType::Religion => &mut self.religion,
            SkillType::SleightOfHand => &mut self.sleight_of_hand,
            SkillType::Stealth => &mut self.stealth,
            SkillType::Survival => &mut self.survival,
        }
    }

    pub fn get_skill_proficiency(&self, skill_type: SkillType) -> bool {
        match skill_type {
            SkillType::Acrobatics => self.acrobatics.proficiency,
            SkillType::AnimalHandling => self.animal_handling.proficiency,
            SkillType::Arcana => self.arcana.proficiency,
            SkillType::Athletics => self.athletics.proficiency,
            SkillType::Deception => self.deception.proficiency,
            SkillType::History => self.history.proficiency,
            SkillType::Insight => self.insight.proficiency,
            SkillType::Intimidation => self.intimidation.proficiency,
            SkillType::Investigation => self.investigation.proficiency,
            SkillType::Medicine => self.medicine.proficiency,
            SkillType::Nature => self.nature.proficiency,
            SkillType::Perception => self.perception.proficiency,
            SkillType::Performance => self.performance.proficiency,
            SkillType::Persuasion => self.persuasion.proficiency,
            SkillType::Religion => self.religion.proficiency,
            SkillType::SleightOfHand => self.sleight_of_hand.proficiency,
            SkillType::Stealth => self.stealth.proficiency,
            SkillType::Survival => self.survival.proficiency,
        }
    }

    pub fn get_skill_expertise(&self, skill_type: SkillType) -> bool {
        match skill_type {
            SkillType::Acrobatics => self.acrobatics.expertise,
            SkillType::AnimalHandling => self.animal_handling.expertise,
            SkillType::Arcana => self.arcana.expertise,
            SkillType::Athletics => self.athletics.expertise,
            SkillType::Deception => self.deception.expertise,
            SkillType::History => self.history.expertise,
            SkillType::Insight => self.insight.expertise,
            SkillType::Intimidation => self.intimidation.expertise,
            SkillType::Investigation => self.investigation.expertise,
            SkillType::Medicine => self.medicine.expertise,
            SkillType::Nature => self.nature.expertise,
            SkillType::Perception => self.perception.expertise,
            SkillType::Performance => self.performance.expertise,
            SkillType::Persuasion => self.persuasion.expertise,
            SkillType::Religion => self.religion.expertise,
            SkillType::SleightOfHand => self.sleight_of_hand.expertise,
            SkillType::Stealth => self.stealth.expertise,
            SkillType::Survival => self.survival.expertise,
        }
    }
}