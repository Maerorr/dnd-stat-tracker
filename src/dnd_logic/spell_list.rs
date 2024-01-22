use core::panic;

use serde::{Deserialize, Serialize};

use super::spell::Spell;

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct SpellList {
    pub spells_1_lvl: Vec<Spell>,
    pub spells_2_lvl: Vec<Spell>,
    pub spells_3_lvl: Vec<Spell>,
    pub spells_4_lvl: Vec<Spell>,
    pub spells_5_lvl: Vec<Spell>,
    pub spells_6_lvl: Vec<Spell>,
    pub spells_7_lvl: Vec<Spell>,
    pub spells_8_lvl: Vec<Spell>,
    pub spells_9_lvl: Vec<Spell>,
}

impl Default for SpellList {
    fn default() -> Self {
        Self {
            spells_1_lvl: Vec::new(),
            spells_2_lvl: Vec::new(),
            spells_3_lvl: Vec::new(),
            spells_4_lvl: Vec::new(),
            spells_5_lvl: Vec::new(),
            spells_6_lvl: Vec::new(),
            spells_7_lvl: Vec::new(),
            spells_8_lvl: Vec::new(),
            spells_9_lvl: Vec::new(),
        }
    }
}

impl SpellList {
    pub fn add_spell(&mut self, spell: &Spell) {
        match spell.level {
            1 => self.spells_1_lvl.push(spell.clone()),
            2 => self.spells_2_lvl.push(spell.clone()),
            3 => self.spells_3_lvl.push(spell.clone()),
            4 => self.spells_4_lvl.push(spell.clone()),
            5 => self.spells_5_lvl.push(spell.clone()),
            6 => self.spells_6_lvl.push(spell.clone()),
            7 => self.spells_7_lvl.push(spell.clone()),
            8 => self.spells_8_lvl.push(spell.clone()),
            9 => self.spells_9_lvl.push(spell.clone()),
            _ => (),
        }
    }

    pub fn get_spells_of_level(&mut self, level: i32) -> &mut Vec<Spell> {
        match level {
            1 => &mut self.spells_1_lvl,
            2 => &mut self.spells_2_lvl,
            3 => &mut self.spells_3_lvl,
            4 => &mut self.spells_4_lvl,
            5 => &mut self.spells_5_lvl,
            6 => &mut self.spells_6_lvl,
            7 => &mut self.spells_7_lvl,
            8 => &mut self.spells_8_lvl,
            9 => &mut self.spells_9_lvl,
            _ => panic!("Invalid spell level"),
        }
    }
}