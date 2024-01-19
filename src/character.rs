use crate::dnd_utils::{stat_to_modifier, Stat, Dice, DeathSaves, Class, Stats, Skills, proficiency_bonus};

pub struct Character {
    pub name: String,
    pub level: i32,
    pub class: Class,

    pub experience: i32,
    pub armor_class: i32,
    pub initiative_bonus: i32,
    pub speed: i32,
    pub hit_dice_total: Dice,
    pub hit_dice_used: Dice,
    pub current_hit_points: i32,
    pub temporary_hit_points: i32,
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
            hit_dice_total: class.get_hit_dice(),
            hit_dice_used: Dice::new(class.get_hit_dice().sides, 0),
            current_hit_points: 0,
            temporary_hit_points: 0,
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
        character.name = String::from("test character");
        character.level = 5;
        character.class = Class::Barbarian;
        character.stats = Stats::test_stats();
        character.skills = Skills::test_skills();
        character.proficiency_bonus = proficiency_bonus(character.level);
        character
    }
}

