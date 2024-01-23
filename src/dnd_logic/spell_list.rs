use core::panic;
use std::ops::Index;

use serde::{Deserialize, Serialize};

use super::{class::Class, spell::Spell};

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct SpellList {
    pub cantrips: Vec<Spell>,
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
            cantrips: Vec::new(),
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
            0 => {
                if !self.cantrips.contains(spell) {
                    self.cantrips.push(spell.clone());
                }
            },
            1 => {
                if !self.spells_1_lvl.contains(spell) {
                    self.spells_1_lvl.push(spell.clone());
                }
            },
            2 => {
                if !self.spells_2_lvl.contains(spell) {
                    self.spells_2_lvl.push(spell.clone());
                }
            },
            3 => {
                if !self.spells_3_lvl.contains(spell) {
                    self.spells_3_lvl.push(spell.clone());
                }
            },
            4 => {
                if !self.spells_4_lvl.contains(spell) {
                    self.spells_4_lvl.push(spell.clone());
                }
            },
            5 => {
                if !self.spells_5_lvl.contains(spell) {
                    self.spells_5_lvl.push(spell.clone());
                }
            },
            6 => {
                if !self.spells_6_lvl.contains(spell) {
                    self.spells_6_lvl.push(spell.clone());
                }
            },
            7 => {
                if !self.spells_7_lvl.contains(spell) {
                    self.spells_7_lvl.push(spell.clone());
                }
            },
            8 => {
                if !self.spells_8_lvl.contains(spell) {
                    self.spells_8_lvl.push(spell.clone());
                }
            },
            9 => {
                if !self.spells_9_lvl.contains(spell) {
                    self.spells_9_lvl.push(spell.clone());
                }
            },
            _ => (),
        }
    }

    pub fn get_spells_of_level(&mut self, level: i32) -> &mut Vec<Spell> {
        match level {
            0 => &mut self.cantrips,
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

    pub fn get_spells_of_level_and_class(&mut self, level: i32, class: &Class) -> Vec<Spell> {
        let mut spells = Vec::new();
        match level {
            0 => {
                for spell in &self.cantrips {
                    if spell.classes.contains(class) {
                        spells.push(spell.clone());
                    }
                }
            }
            1 => {
                for spell in &self.spells_1_lvl {
                    if spell.classes.contains(class) {
                        spells.push(spell.clone());
                    }
                }
            }
            2 => {
                for spell in &self.spells_2_lvl {
                    if spell.classes.contains(class) {
                        spells.push(spell.clone());
                    }
                }
            }
            3 => {
                for spell in &self.spells_3_lvl {
                    if spell.classes.contains(class) {
                        spells.push(spell.clone());
                    }
                }
            }
            4 => {
                for spell in &self.spells_4_lvl {
                    if spell.classes.contains(class) {
                        spells.push(spell.clone());
                    }
                }
            }
            5 => {
                for spell in &self.spells_5_lvl {
                    if spell.classes.contains(class) {
                        spells.push(spell.clone());
                    }
                }
            }
            6 => {
                for spell in &self.spells_6_lvl {
                    if spell.classes.contains(class) {
                        spells.push(spell.clone());
                    }
                }
            }
            7 => {
                for spell in &self.spells_7_lvl {
                    if spell.classes.contains(class) {
                        spells.push(spell.clone());
                    }
                }
            }
            8 => {
                for spell in &self.spells_8_lvl {
                    if spell.classes.contains(class) {
                        spells.push(spell.clone());
                    }
                }
            }
            9 => {
                for spell in &self.spells_9_lvl {
                    if spell.classes.contains(class) {
                        spells.push(spell.clone());
                    }
                }
            }
            _ => panic!("Invalid spell level"),
        }
        spells
    }

    pub fn remove_spell_of_level(&mut self, level: i32, spell: &Spell) {
        match level {
            0 => {
                if self.cantrips.contains(spell) {
                    self.cantrips.remove(self.cantrips.iter().position(|x| x == spell).unwrap());
                }
            }
            1 => {
                if self.spells_1_lvl.contains(spell) {
                    self.spells_1_lvl.remove(self.spells_1_lvl.iter().position(|x| x == spell).unwrap());
                }
            }
            2 => {
                if self.spells_2_lvl.contains(spell) {
                    self.spells_2_lvl.remove(self.spells_2_lvl.iter().position(|x| x == spell).unwrap());
                }
            }
            3 => {
                if self.spells_3_lvl.contains(spell) {
                    self.spells_3_lvl.remove(self.spells_3_lvl.iter().position(|x| x == spell).unwrap());
                }
            }
            4 => {
                if self.spells_4_lvl.contains(spell) {
                    self.spells_4_lvl.remove(self.spells_4_lvl.iter().position(|x| x == spell).unwrap());
                }
            }
            5 => {
                if self.spells_5_lvl.contains(spell) {
                    self.spells_5_lvl.remove(self.spells_5_lvl.iter().position(|x| x == spell).unwrap());
                }
            }
            6 => {
                if self.spells_6_lvl.contains(spell) {
                    self.spells_6_lvl.remove(self.spells_6_lvl.iter().position(|x| x == spell).unwrap());
                }
            }
            7 => {
                if self.spells_7_lvl.contains(spell) {
                    self.spells_7_lvl.remove(self.spells_7_lvl.iter().position(|x| x == spell).unwrap());
                }
            }
            8 => {
                if self.spells_8_lvl.contains(spell) {
                    self.spells_8_lvl.remove(self.spells_8_lvl.iter().position(|x| x == spell).unwrap());
                }
            }
            9 => {
                if self.spells_9_lvl.contains(spell) {
                    self.spells_9_lvl.remove(self.spells_9_lvl.iter().position(|x| x == spell).unwrap());
                }
            }
            _ => panic!("Invalid spell level"),
        }
    }
}