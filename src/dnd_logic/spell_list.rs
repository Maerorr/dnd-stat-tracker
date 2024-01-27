use core::panic;
use std::ops::Index;

use serde::{Deserialize, Serialize};

use super::{class::Class, spell::Spell};

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
/// all spells are held in Vec<(Spell, bool)> where the bool is whether or not the spell is prepared
pub struct SpellList {
    // cantrips have the same structure as spells for simplicity of accessing them from UI
    pub cantrips: Vec<(Spell, bool)>,
    // spells of levels, where idx = (level - 1)
    pub spells: Vec<Vec<(Spell, bool)>>,
}

impl Default for SpellList {
    fn default() -> Self {
        Self {
            cantrips: Vec::new(),
            spells: vec![Vec::new(); 9],
        }
    }
}

impl SpellList {
    pub fn add_spell(&mut self, spell: &Spell, prepared: bool) {
        match spell.level {
            0 => {
                if !self.cantrips.iter().any(|x| x.0.name == spell.name) {
                    self.cantrips.push((spell.clone(), true));
                }
            },
            1..=9 => {
                if !self.spells[spell.level as usize - 1]
                .iter()
                .any(|x| x.0.name == spell.name) {
                    self.spells[spell.level as usize - 1].push((spell.clone(), prepared));
                }
            },

            _ => (),
        }
    }

    pub fn get_spells_of_level(&mut self, level: i32) -> &mut Vec<(Spell, bool)> {
        match level {
            0 => &mut self.cantrips,
            1..=9 => &mut self.spells[level as usize - 1],
            _ => panic!("{}: {}", "Invalid spell level", level.to_string()),
        }
    }

    pub fn get_spells_of_level_and_class(&mut self, level: i32, class: &Class) -> Vec<Spell> {
        let mut spells = Vec::new();
        match level {
            0 => {
                for spell in &self.cantrips {
                    if spell.0.classes.contains(class) {
                        spells.push(spell.0.clone());
                    }
                }
            }
            1..=9 => {
                for spell in &self.spells[level as usize - 1] {
                    if spell.0.classes.contains(class) {
                        spells.push(spell.0.clone());
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
                // if self.cantrips.contains(spell) {
                //     self.cantrips.remove(self.cantrips.iter().position(|x| x == spell).unwrap());
                // }
                self.cantrips.retain(|x| x.0.name != spell.name);
            }
            1..=9 => {
                self.spells[level as usize - 1].retain(|x| x.0.name != spell.name);
            }
            _ => panic!("Invalid spell level"),
        }
    }

    pub fn get_spell_by_name(&self, name: &str) -> Option<Spell> {
        for spell in &self.cantrips {
            if spell.0.name == name {
                return Some(spell.0.clone());
            }
        }
        for spell in &self.spells {
            for spell in spell {
                if spell.0.name == name {
                    return Some(spell.0.clone());
                }
            }
        }
        None
    }
}