use super::{stat_type::StatType, utils::stat_to_modifier};

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

    pub fn set_proficiency(&mut self, stat_type: StatType, proficiency: bool) {
        self.get_stat_mut(stat_type).set_save_proficiency(proficiency);
    }
}