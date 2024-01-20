use crate::dnd_logic::prelude::*;

pub struct Character {
    pub name: String,
    pub level: i32,
    pub class: Class,

    pub experience: i32,
    pub armor_class: i32,
    pub initiative_bonus: i32,
    pub speed: i32,

    pub maximum_hit_points: i32,
    pub current_hit_points: i32,
    pub temporary_hit_points: i32,

    pub hit_dice_total: Dice,
    pub hit_dice_used: Dice,
    pub death_saves: DeathSaves,
    pub inspiration: bool,

    pub proficiency_bonus: i32,
    pub stats: Stats,
    pub skills: Skills
}

impl Default for Character {
    fn default() -> Self {

        let class = Class::Barbarian;

        Self {
            name: String::from("character"),
            level: 1,
            class: class,
            experience: 0,
            armor_class: 10,
            initiative_bonus: 0,
            speed: 30,
            maximum_hit_points: 1,
            current_hit_points: 1,
            temporary_hit_points: 0,
            hit_dice_total: class.get_hit_dice(),
            hit_dice_used: Dice::new(class.get_hit_dice().sides, 0),
            death_saves: DeathSaves::default(),
            inspiration: false,
            proficiency_bonus: 2,

            stats: Stats::default(),
            skills: Skills::default()
        }
    }
}

impl Character {
    pub fn test_character() -> Self {
        let mut character = Self::default();
        character.maximum_hit_points = 55;
        character.current_hit_points = 55;
        character.temporary_hit_points = 15;
        character.name = String::from("test character");
        character.set_level(5);
        character.class = Class::Barbarian;
        character.stats = Stats::test_stats();
        character.skills = Skills::test_skills();
        character.proficiency_bonus = proficiency_bonus(character.level);
        character.hit_dice_total.count = 5;
        character
    }

    pub fn add_level(&mut self) {
        if self.level >= 20 {
            return;
        }
        self.level += 1;
        self.proficiency_bonus = proficiency_bonus(self.level);
        self.experience = exp_needed_to_lvl(self.level);
    }

    pub fn subtract_level(&mut self) {
        if self.level <= 1 {
            return;
        }
        self.level -= 1;
        self.proficiency_bonus = proficiency_bonus(self.level);
        self.experience = exp_needed_to_lvl(self.level);
    }

    pub fn get_class(&mut self) -> &mut Class {
        &mut self.class
    }

    pub fn set_experience(&mut self, exp: i32) {
        if exp < 0 {
            self.experience = 0;
            return;
        }
        self.experience = exp;
        self.level = exp_to_lvl(exp);
        self.proficiency_bonus = proficiency_bonus(self.level);
    }

    pub fn set_level(&mut self, lvl: i32) {
        if lvl < 1 {
            self.level = 1;
            return;
        }
        self.level = lvl;
        self.experience = exp_needed_to_lvl(lvl);
        self.proficiency_bonus = proficiency_bonus(self.level);
    }

    pub fn add_one_ac(&mut self) {
        self.armor_class += 1;
    }

    pub fn subtract_one_ac(&mut self) {
        if self.armor_class <= 0 {
            return;
        }
        self.armor_class -= 1;
    }

    pub fn add_one_initiative(&mut self) {
        self.initiative_bonus += 1;
    }

    pub fn subtract_one_initiative(&mut self) {
        self.initiative_bonus -= 1;
    }

    pub fn add_5_speed(&mut self) {
        self.speed += 5;
    }

    pub fn subtract_5_speed(&mut self) {
        if self.speed <= 5 {
            self.speed = 0;
            return;
        }
        self.speed -= 5;
    }

    pub fn get_hit_points_max(&self) -> i32 {
        self.hit_dice_total.max_roll() + self.stats.get_stat_modifier(StatType::Constitution) * self.level
    }

    pub fn get_hit_points_current(&self) -> i32 {
        self.current_hit_points
    }

    pub fn get_hit_points_temp(&self) -> i32 {
        self.temporary_hit_points
    }

    pub fn take_damage(&mut self, damage: i32) {
        if damage < 0 {
            return;
        }
        self.current_hit_points -= damage;
        if self.current_hit_points < self.maximum_hit_points * -1 {
            self.current_hit_points = -9999;
        }
    }

    pub fn set_maximum_hit_points(&mut self, max: i32) {
        if max < 0 {
            self.maximum_hit_points = 0;
            return;
        }
        self.maximum_hit_points = max;
    }

    pub fn set_current_hit_points(&mut self, current: i32) {
        if current < 0 {
            self.current_hit_points = 0;
            return;
        }
        if current > self.maximum_hit_points {
            self.current_hit_points = self.maximum_hit_points;
            return;
        }
        self.current_hit_points = current;
    }

    pub fn set_temporary_hit_points(&mut self, temp: i32) {
        if temp < 0 {
            self.temporary_hit_points = 0;
            return;
        }
        self.temporary_hit_points = temp;
    }
}
