use serde::{Deserialize, Serialize};

use super::{skill_type::SkillType, stat_type::StatType};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Skill {
    base_ability: StatType,
    skill_type: SkillType,
    proficiency: bool,
    expertise: bool,
    other_bonus: i32,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
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

    pub fn get_skill_mut(&mut self, skill_type: SkillType) -> &mut Skill {
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

    pub fn set_skill_expertise(&mut self, skill_type: SkillType, expertise: bool) {
        self.get_skill_mut(skill_type).expertise = expertise;
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

    pub fn set_skill_proficiency(&mut self, skill_type: SkillType, prof: bool) {
        self.get_skill_mut(skill_type).proficiency = prof;
    }
}