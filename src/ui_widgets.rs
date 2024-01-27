use std::collections::btree_map::Range;

use egui::{style::Spacing, Align, Align2, Layout, Response, RichText, Sense};
use epaint::{Color32, FontId, Pos2, Rect, Rounding, Stroke, Vec2};
use strum::IntoEnumIterator;

use crate::{app::*, dnd_logic::{prelude::*, spell}};

pub struct UiWidgets {
    exp_change: String,
    curr_hp_change: String,
    max_hp_change: String,
    temp_hp_change: String,
    damage_taken: String,
    temp_hp: String,
    copper: String,
    silver: String,
    electrum: String,
    gold: String,
    platinum: String,
    convert_from_string: String,
    convert_from: Currency,
    convert_to: Currency,
    window_opened: bool,
    spell_database: SpellList,
    display_add_spell_window: bool,
    add_spell_lvl: i32,
}

impl Default for UiWidgets {
    fn default() -> Self {
        Self {
            exp_change: String::new(),
            curr_hp_change: String::new(),
            max_hp_change: String::new(),
            temp_hp_change: String::new(),
            damage_taken: String::new(),
            temp_hp: String::new(),
            copper: String::new(),
            silver: String::new(),
            electrum: String::new(),
            gold: String::new(),
            platinum: String::new(),
            convert_from_string: String::new(),
            convert_from: Currency::Copper,
            convert_to: Currency::Gold,
            window_opened: false,
            spell_database: SpellList::default(),
            display_add_spell_window: false,
            add_spell_lvl: 0,
        }
    }
}

impl UiWidgets {
    pub fn set_spell_database(&mut self, spell_database: SpellList) {
        self.spell_database = spell_database;
    }

    pub fn single_stat_widget(&self, ui: &mut egui::Ui, stat: &mut Stat, i: usize) {
        egui::Grid::new(format!("stat_{}{}", stat.get_name(), if unsafe {EDIT_MODE} { "edit" } else { "" }))
        .min_col_width(10.0)
        .max_col_width(100.0)
        .show(ui, |ui| {
            ui.centered_and_justified(|ui| {
                ui.add(egui::Label::new(RichText::new(stat.get_name()).color(stat.get_stat_color())));
            });
            ui.end_row();
            ui.centered_and_justified(|ui| {
                if unsafe { EDIT_MODE } {
                    ui.horizontal(|ui|{
                        ui.label(format!("{} ({})", stat.get_value(), stat.get_modifier()));
                        let sub_resp = ui.add(egui::Button::new("-"));
                        if sub_resp.clicked() {
                            stat.subtract_one();
                        }
        
                        let add_resp = ui.add(egui::Button::new("+"));
                        if add_resp.clicked() {
                            stat.add_one();
                        }
                    });
                } else {
                    ui.label(format!("{} ({})", stat.get_value(), stat.get_modifier()));
                }
            });
        });
    }

    pub fn basic_character_info(&mut self, ui: &mut egui::Ui, character: &mut Character) {
        ui.horizontal_centered(|ui| {

            draw_vertical_line_at_least(ui, Vec2::new(1.0, 25.0), egui::Color32::from_gray(100));

            ui.label("Name:");
            if unsafe { EDIT_MODE } {
                ui.text_edit_singleline(&mut character.name);
            } else {
                ui.label(&character.name);
            }

            draw_vertical_line_at_least(ui, Vec2::new(1.0, 25.0), egui::Color32::from_gray(100));

            ui.label("Level:");
            ui.label(&character.level.to_string());
            if unsafe { EDIT_MODE } {
                let sub_resp = ui.add(egui::Button::new("-"));
                if sub_resp.clicked() {
                    character.subtract_level();
                }

                let add_resp = ui.add(egui::Button::new("+"));
                if add_resp.clicked() {
                    character.add_level();
                }
            }

            draw_vertical_line_at_least(ui, Vec2::new(1.0, 25.0), egui::Color32::from_gray(100));

            let prof_sign = if character.proficiency_bonus > 0 { "+" } else { "" };
            ui.label(format!("Proficiency Bonus: {}{}", prof_sign, character.proficiency_bonus.to_string()));

            draw_vertical_line_at_least(ui, Vec2::new(1.0, 25.0), egui::Color32::from_gray(100));

            ui.label("Class:");
            if unsafe {EDIT_MODE} {
                // add combo box with all classes
                egui::ComboBox::new("class_combo", "Class")
                .selected_text(character.class.get_name())
                .show_ui(ui, |ui| {
                    ui.style_mut().wrap = Some(false);
                    ui.set_min_width(60.0);
                    ui.selectable_value(character.get_class(), Class::Barbarian, "Barbarian");
                    ui.selectable_value(character.get_class(), Class::Bard, "Bard");
                    ui.selectable_value(character.get_class(), Class::Cleric, "Cleric");
                    ui.selectable_value(character.get_class(), Class::Druid, "Druid");
                    ui.selectable_value(character.get_class(), Class::Fighter, "Fighter");
                    ui.selectable_value(character.get_class(), Class::Monk, "Monk");
                    ui.selectable_value(character.get_class(), Class::Paladin, "Paladin");
                    ui.selectable_value(character.get_class(), Class::Ranger, "Ranger");
                    ui.selectable_value(character.get_class(), Class::Rogue, "Rogue");
                    ui.selectable_value(character.get_class(), Class::Sorcerer, "Sorcerer");
                    ui.selectable_value(character.get_class(), Class::Warlock, "Warlock");
                    ui.selectable_value(character.get_class(), Class::Wizard, "Wizard");
                });
            } else {
                ui.label(character.class.get_name());
            }

            draw_vertical_line_at_least(ui, Vec2::new(1.0, 25.0), egui::Color32::from_gray(100));

            ui.label("Experience:");
            ui.label(&character.experience.to_string());

            if unsafe { EDIT_MODE } {
                //ui.text_edit_singleline(&mut self.exp_change);
                ui.add(egui::TextEdit::singleline(&mut self.exp_change).desired_width(75.0));
                if ui.button("Add").on_hover_text("Add experience to character").clicked() {
                    if let Ok(exp) = self.exp_change.parse::<i32>() {
                        let new_exp = exp + character.experience;
                        character.set_experience(new_exp)
                    }
                };
                if ui.button("Subtract").on_hover_text("Subtract experience from character").clicked() {
                    if let Ok(exp) = self.exp_change.parse::<i32>() {
                        let new_exp = character.experience - exp;
                        character.set_experience(new_exp)
                    }
                };
            }

            draw_vertical_line_at_least(ui, Vec2::new(1.0, 25.0), egui::Color32::from_gray(100));
        });
    }

    pub fn display_saving_throws_proficiencies(&self, ui: &mut egui::Ui, character: &mut Character) {
        egui::Grid::new(format!("{}{}", "saving_throws_grid", if unsafe {EDIT_MODE} { "edit" } else { "" }))
        .min_col_width(1.0)
        .show(ui, |ui| {
            for stat in StatType::iter() {
                    
                    // paint a filled circle if yes, empty circle if no
                    let (rect, _response) = ui.allocate_at_least(Vec2::new(10.0, 10.0), Sense::hover());
                    let mut prof = character.stats.get_stat_saving_throw_proficiency(stat);

                    if unsafe {EDIT_MODE} {
                        //ui.checkbox(&mut prof, " ");
                        small_checkbox(ui, &mut prof);

                        character.stats.set_save_proficiency(stat, prof);
                    } else {
                        if prof {
                            ui.painter().circle_filled(rect.center(), 5.0, egui::Color32::from_rgb(255, 255, 255));
                        } else {
                            ui.painter().circle_stroke(rect.center(), 5.0, Stroke::new(1.0, egui::Color32::from_rgb(255, 255, 255)));
                        }
                    }

                    let bonus = character.proficiency_bonus * prof as i32 + character.stats.get_stat(stat).get_modifier();
                    let sign = if bonus > 0 { "+" } else { "" };

                    ui.label(RichText::new(format!("({}{})", sign, bonus)));

                    ui.label(RichText::new(stat.get_name()).color(stat.get_stat_color()));
                    ui.end_row();
            }
        });
    }

    pub fn display_proficiencies(&self, ui: &mut egui::Ui, character: &mut Character) {
        egui::Grid::new("proficiencies_grid")
        .min_col_width(1.0)
        .show(ui, |ui| {
            for skill in SkillType::iter() {
                // paint a filled circle if yes, empty circle if no
            
                // is proficient?
                let mut prof = character.skills.get_skill_proficiency(skill);
                // has expertise?
                let mut expert = character.skills.get_skill_expertise(skill);
                let other_bonus = character.skills.get_skill_other_bonus(skill);
                let skill_mod = character.stats.get_stat(skill.get_base_stat()).get_modifier();

                // calculate total bonus
                let bonus = skill_mod + character.proficiency_bonus * prof as i32 + character.proficiency_bonus * expert as i32 + other_bonus;
                let sign = if bonus > 0 { "+" } else { "" };
                if unsafe {EDIT_MODE} {
                    //ui.checkbox(&mut prof, " ");
                    small_checkbox(ui, &mut prof);
                    character.skills.set_skill_proficiency(skill, prof);

                    ui.label(RichText::new(format!("({}{})", sign, bonus)));
                    ui.label(skill.get_name());
                    ui.colored_label(egui::Color32::from_gray(100), RichText::new(format!("({})", skill.get_base_stat().get_short_name())).size(10.0));
                    //ui.checkbox(&mut expert, "");
                    small_checkbox(ui, &mut expert);
                    character.skills.set_skill_expertise(skill, expert);
                } else {
                    if prof {
                        let (rect, _response) = ui.allocate_at_least(Vec2::new(12.0, 12.0), Sense::hover());
                        ui.painter().circle_filled(rect.center(), 5.0, egui::Color32::from_rgb(255, 255, 255));
                    } else {
                        let (rect, _response) = ui.allocate_at_least(Vec2::new(12.0, 12.0), Sense::hover());
                        ui.painter().circle_stroke(rect.center(), 5.0, Stroke::new(1.0, egui::Color32::from_rgb(255, 255, 255)));
                    }
                    // proficiency sombol using empty or filled circle
                    ui.label(RichText::new(format!("({}{})", sign, bonus)));
                    ui.label(RichText::new(format!("{}", skill.get_name())));
                    ui.label( RichText::new(format!("({})", skill.get_base_stat().get_short_name())).color(skill.get_base_stat().get_stat_color()).size(10.0));
                    if expert {
                        ui.label("e");
                    }
                    
                }
                
                ui.end_row();
            }
        });
    }

    pub fn display_health_stats(&mut self, ui: &mut egui::Ui, character: &mut Character) {
        ui.vertical(|ui| {
            if unsafe {EDIT_MODE} {
                ui.horizontal(|ui| {
                    ui.columns(3, |columns| {

                        columns[0].vertical_centered(|ui| {
                            egui::Grid::new(format!("{}{}", "ac_grid", if unsafe {EDIT_MODE} { "edit" } else { "" }))
                            .min_col_width(10.0)
                            .show(ui, |ui| {
                                ui.add_space(30.0);
                                if ui.button("-").clicked() {
                                    character.subtract_one_ac();
                                }
                                ui.label(character.base_armor_class.to_string());
                                if ui.button("+").clicked() {
                                    character.add_one_ac();
                                }
                                ui.add_space(20.0);
                            });
                            ui.label(RichText::new("Armor Class").size(14.0));
                        });

                        columns[1].vertical_centered(|ui| {
                            egui::Grid::new(format!("{}{}", "initiative_grid", if unsafe {EDIT_MODE} { "edit" } else { "" }))
                            .min_col_width(10.0)
                            .show(ui, |ui| {
                                ui.add_space(30.0);
                                if ui.button("-").clicked() {
                                    character.subtract_one_initiative();
                                }
                                let init = character.stats.get_stat(StatType::Dexterity).get_modifier() + character.initiative_bonus;
                                let init_sign = if init > 0 { "+" } else { "" };
                                ui.label(format!("{}{}", init_sign, init));
                                if ui.button("+").clicked() {
                                    character.add_one_initiative();
                                }
                                //ui.add_space(20.0);
                            });
                            ui.label(RichText::new("Initiative").size(14.0));
                        });
                        columns[2].vertical_centered(|ui| {
                            egui::Grid::new(format!("{}{}", "speed_grid", if unsafe {EDIT_MODE} { "edit" } else { "" }))
                            .min_col_width(10.0)
                            .show(ui, |ui| {
                                ui.add_space(20.0);
                                if ui.button("-").clicked() {
                                    character.subtract_5_speed();
                                }
                                ui.label(format!("{}{}", character.speed.to_string(), "ft."));
                                if ui.button("+").clicked() {
                                    character.add_5_speed();
                                }
                            });
                            ui.label(RichText::new("Speed").size(14.0));
                        });

                    });
                });

                draw_horizontal_line_at_least(ui, Vec2::new(380.0, 1.0), egui::Color32::from_gray(100));
            
                egui::Grid::new(format!("{}{}", "hp_grid", if unsafe {EDIT_MODE} { "edit" } else { "" }))
                .min_col_width(50.0)
                .show(ui, |ui| {
                        let max_hp_no_change = character.maximum_hit_points;
                        let curr_hp_no_change = character.current_hit_points;
                        let temp_hp_no_change = character.temporary_hit_points;
                        self.max_hp_change = character.maximum_hit_points.to_string();
                        self.curr_hp_change = character.current_hit_points.to_string();
                        self.temp_hp_change = character.temporary_hit_points.to_string();
                        ui.label(RichText::new("HP Max: ").size(14.0));
                        ui.text_edit_singleline(&mut self.max_hp_change);

                        ui.end_row();

                        ui.label(RichText::new("HP Current: ").color(CURRENT_HP_COLOR).size(18.0));
                        ui.text_edit_singleline(&mut self.curr_hp_change);

                        ui.end_row();
                        ui.label(RichText::new("HP Temp: ").color(TEMP_HP_COLOR).size(18.0));
                        ui.text_edit_singleline(&mut self.temp_hp_change);

                        // set the values to character, if parsing fails, don't change the value
                        if let Ok(max_hp) = self.max_hp_change.parse::<i32>() {
                            if max_hp != max_hp_no_change {
                                character.set_maximum_hit_points(max_hp);
                            }
                        }
                        if let Ok(curr_hp) = self.curr_hp_change.parse::<i32>() {
                            if curr_hp != curr_hp_no_change {
                                character.set_current_hit_points(curr_hp);
                            }
                        }
                        if let Ok(temp_hp) = self.temp_hp_change.parse::<i32>() {
                            if temp_hp != temp_hp_no_change {
                                character.set_temporary_hit_points(temp_hp);
                            }
                        }
                        ui.end_row();
                });

                draw_horizontal_line_at_least(ui, Vec2::new(380.0, 1.0), egui::Color32::from_gray(100));

                ui.horizontal(|ui| {
                    ui.vertical(|ui| {
                        ui.label(RichText::new("Hit Dice Total"));
                        ui.end_row();
                        ui.label(RichText::new(character.hit_dice_total.to_string()));
                    });
                    ui.add_space(80.0);
                    ui.vertical(|ui| {
                        ui.horizontal(|ui| {
                            ui.label(RichText::new("Successes").size(14.0));
                            let successes = character.death_saves.successes;
                            for _ in 0..successes {
                                draw_circle_filled(ui, Vec2::new(10.0, 10.0), 5.0, MAIN_COLOR);
                            }
                            for _ in 0..(3 - successes) {
                                draw_circle_stroke(ui, Vec2::new(10.0, 10.0), 5.0, MAIN_COLOR);
                            }

                            if ui.button("Success").clicked() {
                                character.add_success_death_save();
                            }

                        });

                        ui.horizontal(|ui| {
                            ui.label(RichText::new("Failures").size(14.0));
                            ui.add_space(15.0);
                            let failures = character.death_saves.failures;
                            for _ in 0..failures {
                                draw_circle_filled(ui, Vec2::new(10.0, 10.0), 5.0, MAIN_COLOR);
                            }
                            for _ in 0..(3 - failures) {
                                draw_circle_stroke(ui, Vec2::new(10.0, 10.0), 5.0, MAIN_COLOR);
                            }

                            if ui.button("Failure").clicked() {
                                character.add_fail_death_save();
                            }
                        });

                        ui.vertical_centered(|ui| {
                            if ui.button("Reset Death Saves").clicked() {
                                character.death_saves = DeathSaves::default();
                            }
                        });
                    });
                });
                
            } else {

                // NON EDIT UI

                ui.columns(3, |columns| {
                    columns[0].set_width(80.0);
                    columns[0].vertical_centered(|ui| {
                        ui.label(character.base_armor_class.to_string());
                        ui.label(RichText::new("Armor Class").size(14.0));
                    });
                    columns[1].set_width(80.0);
                    columns[1].vertical_centered(|ui| {
                        let init = character.stats.get_stat(StatType::Dexterity).get_modifier() + character.initiative_bonus;
                        let init_sign = if init > 0 { "+" } else { "" };
                        ui.label(format!("{}{}", init_sign, init));
                        ui.label(RichText::new("Initiative").size(14.0));
                    });
                    columns[2].set_width(80.0);
                    columns[2].vertical_centered(|ui| {
                        ui.label(format!("{}{}", character.speed.to_string(), "ft."));
                        ui.label(RichText::new("Speed").size(14.0));
                    });
                });

                ui.add_space(10.0);
                draw_horizontal_line_at_least(ui, Vec2::new(380.0, 1.0), egui::Color32::from_gray(100));
                ui.add_space(10.0);

                egui::Grid::new(format!("{}{}", "hp_grid", if unsafe {EDIT_MODE} { "edit" } else { "" }))
                .min_col_width(50.0)
                .show(ui, |ui| {
                    if unsafe {EDIT_MODE} {
                    } else {
                        ui.label(RichText::new("HP Max: ").size(14.0));
                        ui.label(RichText::new(character.maximum_hit_points.to_string()).size(14.0));
                        ui.end_row();
                        ui.label(RichText::new("HP Current: ").color(CURRENT_HP_COLOR).size(18.0));
                        ui.label(RichText::new(character.current_hit_points.to_string()).size(18.0));
                        ui.text_edit_singleline(&mut self.damage_taken);
                        if ui.button("Take Damage").clicked() {
                            if let Ok(damage) = self.damage_taken.parse::<i32>() {
                                character.take_damage(damage);
                            }
                        }
                        if ui.button("Heal").clicked() {
                            if let Ok(heal) = self.damage_taken.parse::<i32>() {
                                character.heal_damage(heal);
                            }
                        }
                        ui.end_row();
                        ui.label(RichText::new("HP Temp: ").color(TEMP_HP_COLOR).size(18.0));
                        ui.label(RichText::new(character.temporary_hit_points.to_string()).size(18.0));
                        ui.text_edit_singleline(&mut self.temp_hp);
                        if ui.button("Add").clicked() {
                            if let Ok(damage) = self.damage_taken.parse::<i32>() {
                                character.add_temporary_hit_points(damage);
                            }
                        }
                        if ui.button("Subtract").clicked() {
                            if let Ok(heal) = self.damage_taken.parse::<i32>() {
                                character.subtract_temporary_hit_points(heal);
                            }
                        }
                        ui.end_row();
                    }
                });

                ui.add_space(10.0);
                draw_horizontal_line_at_least(ui, Vec2::new(380.0, 1.0), egui::Color32::from_gray(100));
                ui.add_space(10.0);

                ui.horizontal(|ui| {
                    ui.vertical(|ui| {
                        ui.label(RichText::new("Hit Dice Total"));
                        ui.end_row();
                        ui.label(RichText::new(character.hit_dice_total.to_string()));
                    });
                    ui.add_space(80.0);
                    ui.vertical(|ui| {
                        ui.horizontal(|ui| {
                            ui.label(RichText::new("Successes").size(14.0));
                            let successes = character.death_saves.successes;
                            for _ in 0..successes {
                                draw_circle_filled(ui, Vec2::new(10.0, 10.0), 5.0, MAIN_COLOR);
                            }
                            for _ in 0..(3 - successes) {
                                draw_circle_stroke(ui, Vec2::new(10.0, 10.0), 5.0, MAIN_COLOR);
                            }

                            if ui.button("Success").clicked() {
                                character.add_success_death_save();
                            }

                        });

                        ui.horizontal(|ui| {
                            ui.label(RichText::new("Failures").size(14.0));
                            ui.add_space(15.0);
                            let failures = character.death_saves.failures;
                            for _ in 0..failures {
                                draw_circle_filled(ui, Vec2::new(10.0, 10.0), 5.0, MAIN_COLOR);
                            }
                            for _ in 0..(3 - failures) {
                                draw_circle_stroke(ui, Vec2::new(10.0, 10.0), 5.0, MAIN_COLOR);
                            }

                            if ui.button("Failure").clicked() {
                                character.add_fail_death_save();
                            }
                        });

                        ui.vertical_centered(|ui| {
                            if ui.button("Reset Death Saves").clicked() {
                                character.death_saves = DeathSaves::default();
                            }
                        });
                    });
                });
                
            }
            ui.add_space(10.0);
            draw_horizontal_line_at_least(ui, Vec2::new(380.0, 1.0), egui::Color32::from_gray(100));
        });
    }

    pub fn display_money(&mut self, ui: &mut egui::Ui, character: &mut Character) {
        ui.add_space(10.0);
        ui.vertical(|ui| {
            ui.horizontal(|ui| {
                ui.columns(5, |columns| {
                    columns[0].vertical_centered(|ui| {
                        ui.label(format!("{}", character.money.copper));
                        ui.label(RichText::new("cp").color(COPPER_COLOR));
                        ui.add(egui::TextEdit::singleline(&mut self.copper).desired_width(50.0));
                        if ui.button("Add").clicked() {
                            if let Ok(copper) = self.copper.parse::<i32>() {
                                character.money.add_money(&Currency::Copper, copper);
                                self.copper = "".to_string();
                            }
                        }
                        if ui.button("Subtract").clicked() {
                            if let Ok(copper) = self.copper.parse::<i32>() {
                                character.money.subtract_money(&Currency::Copper, copper);
                                self.copper = "".to_string();
                            }
                        }
                    });
                    columns[1].vertical_centered(|ui| {
                        ui.label(format!("{}", character.money.silver));
                        ui.label(RichText::new("sp").color(SILVER_COLOR));
                        ui.add(egui::TextEdit::singleline(&mut self.silver).desired_width(50.0));
                        if ui.button("Add").clicked() {
                            if let Ok(silver) = self.silver.parse::<i32>() {
                                character.money.add_money(&Currency::Silver, silver);
                                self.silver = "".to_string();
                            }
                        }
                        if ui.button("Subtract").clicked() {
                            if let Ok(silver) = self.silver.parse::<i32>() {
                                character.money.subtract_money(&Currency::Silver, silver);
                                self.silver = "".to_string();
                            }
                        }
                    });
                    columns[2].vertical_centered(|ui| {
                        ui.label(format!("{}", character.money.electrum));
                        ui.label(RichText::new("ep").color(ELECTRUM_COLOR));
                        ui.add(egui::TextEdit::singleline(&mut self.electrum).desired_width(50.0));
                        if ui.button("Add").clicked() {
                            if let Ok(electrum) = self.electrum.parse::<i32>() {
                                character.money.add_money(&Currency::Electrum, electrum);
                                self.electrum = "".to_string();
                            }
                        }
                        if ui.button("Subtract").clicked() {
                            if let Ok(electrum) = self.electrum.parse::<i32>() {
                                character.money.subtract_money(&Currency::Electrum, electrum);
                                self.electrum = "".to_string();
                            }
                        }
                    });
                    columns[3].vertical_centered(|ui| {
                        ui.label(format!("{}", character.money.gold));
                        ui.label(RichText::new("gp").color(GOLD_COLOR));
                        ui.add(egui::TextEdit::singleline(&mut self.gold).desired_width(50.0));
                        if ui.button("Add").clicked() {
                            if let Ok(gold) = self.gold.parse::<i32>() {
                                character.money.add_money(&Currency::Gold, gold);
                                self.gold = "".to_string();
                            }
                        }
                        if ui.button("Subtract").clicked() {
                            if let Ok(gold) = self.gold.parse::<i32>() {
                                character.money.subtract_money(&Currency::Gold, gold);
                                self.gold = "".to_string();
                            }
                        }
                    });
                    columns[4].vertical_centered(|ui| {
                        ui.label(format!("{}", character.money.platinum));
                        ui.label(RichText::new("pp").color(PLATINUM_COLOR));
                        ui.add(egui::TextEdit::singleline(&mut self.platinum).desired_width(50.0));
                        if ui.button("Add").clicked() {
                            if let Ok(platinum) = self.platinum.parse::<i32>() {
                                character.money.add_money(&Currency::Platinum, platinum);
                                self.platinum = "".to_string();
                            }
                        }
                        if ui.button("Subtract").clicked() {
                            if let Ok(platinum) = self.platinum.parse::<i32>() {
                                character.money.subtract_money(&Currency::Platinum, platinum);
                                self.platinum = "".to_string();
                            }
                        }
                    });

                });
            });

            draw_horizontal_line_at_least(ui, Vec2::new(380.0, 1.0), egui::Color32::from_gray(100));

            ui.vertical_centered(|ui| {ui.label("Convert");});

            ui.horizontal(|ui| {
                ui.add(egui::TextEdit::singleline(&mut self.convert_from_string).desired_width(50.0));
                egui::ComboBox::new("convert_from", "")
                .selected_text(&self.convert_from.to_string())
                .show_ui(ui, |ui| {
                    ui.style_mut().wrap = Some(false);
                    ui.set_min_width(60.0);
                    ui.selectable_value(&mut self.convert_from, Currency::Copper, "Copper");
                    ui.selectable_value(&mut self.convert_from, Currency::Silver, "Silver");
                    ui.selectable_value(&mut self.convert_from, Currency::Electrum, "Electrum");
                    ui.selectable_value(&mut self.convert_from, Currency::Gold, "Gold");
                    ui.selectable_value(&mut self.convert_from, Currency::Platinum, "Platinum");
                });
                let (rect, response) = ui.allocate_at_least(Vec2::new(75.0, 10.0), Sense::hover());
                ui.painter().arrow(rect.left_center(), Vec2::new(70.0, 0.0), Stroke::new(2.0, MAIN_COLOR));
                egui::ComboBox::new("convert_to", "")
                .selected_text(&self.convert_to.to_string())
                .show_ui(ui, |ui| {
                    ui.style_mut().wrap = Some(false);
                    ui.set_min_width(60.0);
                    ui.selectable_value(&mut self.convert_to, Currency::Copper, "Copper");
                    ui.selectable_value(&mut self.convert_to, Currency::Silver, "Silver");
                    ui.selectable_value(&mut self.convert_to, Currency::Electrum, "Electrum");
                    ui.selectable_value(&mut self.convert_to, Currency::Gold, "Gold");
                    ui.selectable_value(&mut self.convert_to, Currency::Platinum, "Platinum");

                })
            });

            ui.vertical_centered(|ui| {
                if ui.button("Convert").clicked() {
                    if let Ok(amount) = self.convert_from_string.parse::<i32>() {
                        if let Some(new_amount) = character.money.convert_money(amount, &self.convert_from, &self.convert_to) {
                            if *character.money.get_currency_mut(&self.convert_from) >= amount {
                                character.money.subtract_money(&self.convert_from, amount);
                                character.money.add_money(&self.convert_to, new_amount);
                                println!("Converted {} {} to {} {}", amount, self.convert_from.to_string(), new_amount, self.convert_to.to_string());
                            } else {
                                println!("Not enough money to convert {} {}", self.convert_from.to_string(), amount);
                            }
                        }
                    }
                }
            });
            
        });
    }

    pub fn display_spell_list(&mut self, ctx: &egui::Context, ui: &mut egui::Ui, character: &mut Character) {
        let mut spells_to_delete: Vec<(Spell, bool)> = Vec::new();

        ui.horizontal(|ui|{
            ui.vertical(|ui| {
                ui.label("Cantrips");
                draw_horizontal_line_at_least(ui, Vec2::new(250.0, 1.0), egui::Color32::from_gray(100));
    
                // DISPLAY CANTRIPS
                for mut spell_0 in character.spell_list.get_spells_of_level(0) {
                    let delete = draw_spell_entry(ctx, ui, &mut spell_0);
                    if delete {
                        spells_to_delete.push(spell_0.clone());
                    }
                }
                for spell in spells_to_delete.iter() {
                    character.spell_list.remove_spell_of_level(0, &spell.0);
                }
                spells_to_delete.clear();
    
                if unsafe {EDIT_MODE} {
                    if ui.button("Add Cantrip").clicked() {
                        self.add_spell_lvl = 0;
                        self.display_add_spell_window = true;
                    }
                }
                
                ui.add_space(20.0);
                ui.label("Level 1");
                draw_horizontal_line_at_least(ui, Vec2::new(250.0, 1.0), egui::Color32::from_gray(100));
    
                // DISPLAY LEVEL 1 SPELLS
                for mut spell_1 in character.spell_list.get_spells_of_level(1) {
                    let delete = draw_spell_entry(ctx, ui, &mut spell_1);
                    if delete {
                        spells_to_delete.push(spell_1.clone());
                    }
                }
                for spell in spells_to_delete.iter() {
                    character.spell_list.remove_spell_of_level(1, &spell.0);
                }
                spells_to_delete.clear();
    
                if unsafe {EDIT_MODE} {
                    if ui.button("Add Level 1 Spell").clicked() {
                        self.add_spell_lvl = 1;
                        self.display_add_spell_window = true;
                    }
                }
    
                ui.add_space(20.0);
                ui.label("Level 2");
                
                draw_horizontal_line_at_least(ui, Vec2::new(250.0, 1.0), egui::Color32::from_gray(100));
    
                // DISPLAY LEVEL 2 SPELLS
                for mut spell_2 in character.spell_list.get_spells_of_level(2) {
                    let delete = draw_spell_entry(ctx, ui, &mut spell_2);
                    if delete {
                        spells_to_delete.push(spell_2.clone());
                    }
                }
                for spell in spells_to_delete.iter() {
                    character.spell_list.remove_spell_of_level(2, &spell.0);
                }
                spells_to_delete.clear();
    
                if unsafe {EDIT_MODE} {
                    if ui.button("Add Level 2 Spell").clicked() {
                        self.add_spell_lvl = 2;
                        self.display_add_spell_window = true;
                    }
                }
            });
            ui.vertical(|ui| {
                ui.label("Level 3");
                draw_horizontal_line_at_least(ui, Vec2::new(250.0, 1.0), egui::Color32::from_gray(100));
    
                // DISPLAY LEVEL 3 SPELLS
                for mut spell_3 in character.spell_list.get_spells_of_level(3) {
                    let delete = draw_spell_entry(ctx, ui, &mut spell_3);
                    if delete {
                        spells_to_delete.push(spell_3.clone());
                    }
                }
                for spell in spells_to_delete.iter() {
                    character.spell_list.remove_spell_of_level(3, &spell.0);
                }
                spells_to_delete.clear();
    
                if unsafe {EDIT_MODE} {
                    if ui.button("Add Level 3 Spell").clicked() {
                        self.add_spell_lvl = 3;
                        self.display_add_spell_window = true;
                    }
                }
    
                ui.add_space(20.0);
                ui.label("Level 4");
                draw_horizontal_line_at_least(ui, Vec2::new(250.0, 1.0), egui::Color32::from_gray(100));
    
                // DISPLAY LEVEL 4 SPELLS
                for mut spell_4 in character.spell_list.get_spells_of_level(4) {
                    let delete = draw_spell_entry(ctx, ui, &mut spell_4);
                    if delete {
                        spells_to_delete.push(spell_4.clone());
                    }
                }
                for spell in spells_to_delete.iter() {
                    character.spell_list.remove_spell_of_level(4, &spell.0);
                }
                spells_to_delete.clear();
    
                if unsafe {EDIT_MODE} {
                    if ui.button("Add Level 4 Spell").clicked() {
                        self.add_spell_lvl = 4;
                        self.display_add_spell_window = true;
                    }
                }

                ui.add_space(20.0);
                ui.label("Level 5");
                draw_horizontal_line_at_least(ui, Vec2::new(250.0, 1.0), egui::Color32::from_gray(100));

                // DISPLAY LEVEL 5 SPELLS
                for mut spell_5 in character.spell_list.get_spells_of_level(5) {
                    let delete = draw_spell_entry(ctx, ui, &mut spell_5);
                    if delete {
                        spells_to_delete.push(spell_5.clone());
                    }
                }
                for spell in spells_to_delete.iter() {
                    character.spell_list.remove_spell_of_level(5, &spell.0);
                }
                spells_to_delete.clear();

                if unsafe {EDIT_MODE} {
                    if ui.button("Add Level 5 Spell").clicked() {
                        self.add_spell_lvl = 5;
                        self.display_add_spell_window = true;
                    }
                }
            });
            ui.vertical(|ui|{
                ui.label("Level 6");
                draw_horizontal_line_at_least(ui, Vec2::new(250.0, 1.0), egui::Color32::from_gray(100));

                // DISPLAY LEVEL 6 SPELLS
                for mut spell_6 in character.spell_list.get_spells_of_level(6) {
                    let delete = draw_spell_entry(ctx, ui, &mut spell_6);
                    if delete {
                        spells_to_delete.push(spell_6.clone());
                    }
                }

                for spell in spells_to_delete.iter() {
                    character.spell_list.remove_spell_of_level(6, &spell.0);
                }
                spells_to_delete.clear();

                if unsafe {EDIT_MODE} {
                    if ui.button("Add Level 6 Spell").clicked() {
                        self.add_spell_lvl = 6;
                        self.display_add_spell_window = true;
                    }
                }

                ui.add_space(20.0);
                ui.label("Level 7");
                draw_horizontal_line_at_least(ui, Vec2::new(250.0, 1.0), egui::Color32::from_gray(100));

                // DISPLAY LEVEL 7 SPELLS
                for mut spell_7 in character.spell_list.get_spells_of_level(7) {
                    let delete = draw_spell_entry(ctx, ui, &mut spell_7);
                    if delete {
                        spells_to_delete.push(spell_7.clone());
                    }
                }
                for spell in spells_to_delete.iter() {
                    character.spell_list.remove_spell_of_level(7, &spell.0);
                }

                if unsafe {EDIT_MODE} {
                    if ui.button("Add Level 7 Spell").clicked() {
                        self.add_spell_lvl = 7;
                        self.display_add_spell_window = true;
                    }
                }

                ui.add_space(20.0);
                ui.label("Level 8");
                draw_horizontal_line_at_least(ui, Vec2::new(250.0, 1.0), egui::Color32::from_gray(100));

                // DISPLAY LEVEL 8 SPELLS
                for mut spell_8 in character.spell_list.get_spells_of_level(8) {
                    let delete = draw_spell_entry(ctx, ui, &mut spell_8);
                    if delete {
                        spells_to_delete.push(spell_8.clone());
                    }
                }
                for spell in spells_to_delete.iter() {
                    character.spell_list.remove_spell_of_level(8, &spell.0);
                }

                if unsafe {EDIT_MODE} {
                    if ui.button("Add Level 8 Spell").clicked() {
                        self.add_spell_lvl = 8;
                        self.display_add_spell_window = true;
                    }
                }

                ui.add_space(20.0);
                ui.label("Level 9");
                draw_horizontal_line_at_least(ui, Vec2::new(250.0, 1.0), egui::Color32::from_gray(100));

                // DISPLAY LEVEL 9 SPELLS
                for mut spell_9 in character.spell_list.get_spells_of_level(9) {
                    let delete = draw_spell_entry(ctx, ui, &mut spell_9);
                    if delete {
                        spells_to_delete.push(spell_9.clone());
                    }
                }
                for spell in spells_to_delete.iter() {
                    character.spell_list.remove_spell_of_level(9, &spell.0);
                }

                if unsafe {EDIT_MODE} {
                    if ui.button("Add Level 9 Spell").clicked() {
                        self.add_spell_lvl = 9;
                        self.display_add_spell_window = true;
                    }
                }
            });
        });
        
        
        self.display_add_spell(&ui, character, self.add_spell_lvl)
    }

    pub fn display_add_spell(&mut self, ui: &egui::Ui, character: &mut Character, lvl: i32) {
        let spells_of_lvl = {
            self.spell_database.get_spells_of_level(lvl).clone()
        };
        // display a new window with all spells as buttons
        egui::Window::new(format!("Add Spell Level {}", lvl))
        .open(&mut self.display_add_spell_window)
        .show(ui.ctx(), |ui| {
            // split spells into two halfs, and display in two columns
            ui.columns(2, |columns| {
                columns[0].vertical(|ui| {
                    for (spell, _) in spells_of_lvl.iter().take(spells_of_lvl.len() / 2) {
                        if ui.button(spell.name.clone()).clicked() {
                            character.add_spell(spell);
                        }
                    }
                });
                columns[1].vertical(|ui| {
                    for (spell, _) in spells_of_lvl.iter().skip(spells_of_lvl.len() / 2) {
                        if ui.button(spell.name.clone()).clicked() {
                            character.add_spell(spell);
                        }
                    }
                });
            });
        });
    }

    pub fn display_character_select_square(&mut self, ui: &mut egui::Ui, character: &mut Character) -> egui::Response {
        character_square(ui, character)
    }
}

pub fn centered_label(ui: &mut egui::Ui, text: &str) {
    ui.centered_and_justified(|ui| {
        ui.label(text);
    });
}

pub fn centered_heading(ui: &mut egui::Ui, text: &str) {
    ui.centered_and_justified(|ui| {
        ui.heading(text);
    });
}

pub fn draw_vertical_line_at_least(ui: &mut egui::Ui, vec2: Vec2, color: egui::Color32) {
    let (rect, _response) = ui.allocate_at_least(vec2, Sense::hover());

    ui.painter().line_segment(
        [Pos2::new(rect.left(), rect.top()), Pos2::new(rect.left(), rect.bottom())], 
        Stroke::new(
            1.0, 
            color
        )
    );
}

pub fn draw_horizontal_line_at_least(ui: &mut egui::Ui, vec2: Vec2, color: egui::Color32) {
    let (rect, _response) = ui.allocate_at_least(vec2, Sense::hover());

    ui.painter().line_segment(
        [Pos2::new(rect.left(), rect.top()), Pos2::new(rect.right(), rect.top())], 
        Stroke::new(
            1.0, 
            color
        )
    );
}

pub fn draw_circle_stroke(ui: &mut egui::Ui, vec2: Vec2, radius: f32, color: egui::Color32) {
    let (rect, _response) = ui.allocate_at_least(vec2, Sense::hover());

    ui.painter().circle_stroke(rect.center(), radius, Stroke::new(1.0, color));
}

pub fn draw_circle_filled(ui: &mut egui::Ui, vec2: Vec2, radius: f32, color: egui::Color32) {
    let (rect, _response) = ui.allocate_at_least(vec2, Sense::hover());

    ui.painter().circle_filled(rect.center(), radius, color);
}

// RETURNS: bool - true meaning remove the spell, false means nothing
pub fn draw_spell_entry(ctx: &egui::Context, ui: &mut egui::Ui, (spell, prepared): &mut (spell::Spell, bool)) -> bool {
    ui.end_row();
    let mut flag = false;
    ui.allocate_ui(Vec2::new(250.0, 36.0), |ui| {
        ui.horizontal(|ui| {
            ui.with_layout(Layout::left_to_right(Align::LEFT), |ui| {
                if unsafe {EDIT_MODE} {
                    //ui.checkbox(prepared, "");
                    small_checkbox(ui, prepared);
                } else {
                    if *prepared {
                        draw_circle_filled(ui, Vec2::new(12.0, 14.0), 5.0, MAIN_COLOR);
                    } else {
                        draw_circle_stroke(ui, Vec2::new(12.0, 14.0), 5.0, MAIN_COLOR);
                    }
                }
                
                spell.display_spell_name(ui);
            });
            ui.end_row();
            ui.with_layout(Layout::right_to_left(Align::RIGHT), |ui| {
                if unsafe {EDIT_MODE} {
                    if ui.button("✖").clicked() {
                        flag = true;
                    }
                } else {
                    spell.display_spell_more_button(ui);
                }
            });
        });
    });
    
    ui.end_row();
    spell.try_to_show_spell_window(ctx, ui);
    draw_horizontal_line_at_least(ui, Vec2::new(250.0, 1.0), egui::Color32::from_gray(100));
    flag
}

pub fn character_square(ui: &mut egui::Ui, character: &mut Character) -> egui::Response{
    let mut hovered = false;
    
    // 1. SIZE OF THE WIDGET
    let desired_size = Vec2::new(230.0, 120.0);

    // 2. ALLOCATE SPACE FOR THE WIDGET
    let (rect, mut response) = ui.allocate_at_least(desired_size, Sense::click());

    // 3. CHECK INTERACTION
    if response.clicked() {
        response.mark_changed();
    }

    if response.hovered() {
        hovered = true;
    }

    // 3. DRAW THE WIDGET
    if ui.is_rect_visible(rect) {
        let visuals = ui.style().interact(&response);

        ui.painter().rect_filled(rect, 5.0, egui::Color32::from_gray(40));
        if hovered {
            ui.painter().rect_stroke(rect, 5.0, Stroke::new(4.0, egui::Color32::from_gray(220)));
        } else {
            ui.painter().rect_stroke(rect, 5.0, Stroke::new(2.0, MAIN_COLOR));
        }
        
        // place three labels inside of rect. With character name, class and level
        // make three rect values each being a vertical 1/3 of the base rect
        let name_rect = Rect::from_min_size(rect.min, Vec2::new(rect.width(), rect.height() / 3.0));
        let class_rect = Rect::from_min_size(Pos2::new(rect.min.x, rect.min.y + name_rect.height()), Vec2::new(rect.width(), rect.height() / 3.0));
        let level_rect = Rect::from_min_size(Pos2::new(rect.min.x, rect.min.y + name_rect.height() + class_rect.height()), Vec2::new(rect.width(), rect.height() / 3.0));
        
        ui.painter().text(
            name_rect.center(),
            Align2::CENTER_CENTER,
            character.name.to_string(),
            FontId::default(),
            egui::Color32::WHITE
        );

        ui.painter().text(
            class_rect.center(),
            Align2::CENTER_CENTER,
            format!("{}", character.class.get_name()),
            FontId::default(),
            egui::Color32::WHITE
        );

        ui.painter().text(
            level_rect.center(),
            Align2::CENTER_CENTER,
            format!("Level {}", character.level),
            FontId::default(),
            egui::Color32::WHITE
        );
    }

    response
}

fn small_checkbox(ui: &mut egui::Ui, checked: &mut bool) -> Response {
    let mut response = ui.allocate_response(Vec2::new(12.0, 12.0), Sense::click());
    let visuals = ui.style().interact(&response);
    let rect = response.rect;
    let center = rect.center();

    ui.painter().rect_filled(rect, 2.0, Color32::from_gray(60));
    ui.painter().rect_stroke(rect, 2.0, Stroke::new(1.0, MAIN_COLOR));

    let c = rect.center();
    let w = (rect.width() / 2.0) * 0.6;
    let h = (rect.height() / 2.0) * 0.6;

    if *checked {
        let radius = 1.0;
        // draw an X symbol using ui.painter.line_segment
        ui.painter().line_segment(
            [Pos2::new(c.x - w, c.y - h), Pos2::new(c.x + w, c.y + h)], 
            Stroke::new(
                radius, 
                Color32::from_gray(200)
            )
        );
        ui.painter().line_segment(
            [Pos2::new(c.x - w, c.y + h), Pos2::new(c.x + w, c.y - h)], 
            Stroke::new(
                radius, 
                Color32::from_gray(200)
            )
        );
    }

    if response.clicked() {
        *checked = !*checked;
        response.mark_changed();
    }

    response
}