use egui::{Sense, RichText, style::Spacing};
use epaint::{Vec2, Stroke, Pos2, Rounding};
use strum::IntoEnumIterator;

use crate::{app::{EDIT_MODE, MAIN_COLOR}, dnd_logic::prelude::*};

pub struct UiWidgets {
    exp_change: String,
    curr_hp_change: String,
    max_hp_change: String,
    temp_hp_change: String,
    damage_taken: String,
    temp_hp: String,
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
        }
    }
}

impl UiWidgets {
    pub fn single_stat_widget(&self, ui: &mut egui::Ui, stat: &mut Stat, i: usize) {
        egui::Grid::new(format!("stat_{}{}", stat.get_name(), if unsafe {EDIT_MODE} { "edit" } else { "" }))
        .min_col_width(10.0)
        .max_col_width(100.0)
        .show(ui, |ui| {
            ui.centered_and_justified(|ui| {
                ui.add(egui::Label::new(stat.get_name()));
            });
            ui.end_row();
            ui.centered_and_justified(|ui| {
                ui.label(format!("{} ({})", stat.get_value(), stat.get_modifier()));
            });
            if unsafe { EDIT_MODE } {
                let sub_resp = ui.add(egui::Button::new("-"));
                if sub_resp.clicked() {
                    stat.subtract_one();
                }

                let add_resp = ui.add(egui::Button::new("+"));
                if add_resp.clicked() {
                    stat.add_one();
                }
            }
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
                ui.text_edit_singleline(&mut self.exp_change);
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
                        ui.checkbox(&mut prof, " ");

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

                    ui.label(stat.get_name());
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
                let (rect, _response) = ui.allocate_at_least(Vec2::new(10.0, 10.0), Sense::hover());

                // is proficient?
                let mut prof = character.skills.get_skill_proficiency(skill);
                // has expertise?
                let expert = character.skills.get_skill_expertise(skill);
                let other_bonus = character.skills.get_skill_other_bonus(skill);
                let skill_mod = character.stats.get_stat(skill.get_base_stat()).get_modifier();

                // proficiency sombol using empty or filled circle
                if unsafe {EDIT_MODE} {
                    ui.checkbox(&mut prof, " ");
                    character.skills.set_skill_proficiency(skill, prof);
                } else {
                    if prof {
                        ui.painter().circle_filled(rect.center(), 5.0, egui::Color32::from_rgb(255, 255, 255));
                    } else {
                        ui.painter().circle_stroke(rect.center(), 5.0, Stroke::new(1.0, egui::Color32::from_rgb(255, 255, 255)));
                    }
                }
                // calculate total bonus
                let bonus = skill_mod + character.proficiency_bonus * prof as i32 + character.proficiency_bonus * expert as i32 + other_bonus;
                let sign = if bonus > 0 { "+" } else { "" };

                ui.label(RichText::new(format!("({}{})", sign, bonus)));
                ui.label(skill.get_name());
                ui.colored_label(egui::Color32::from_gray(100), RichText::new(format!("({})", skill.get_base_stat().get_short_name())).size(10.0));
                if expert {
                    ui.label("e");
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
                                ui.label(character.armor_class.to_string());
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

                        ui.label(RichText::new("HP Current: ").size(24.0));
                        ui.text_edit_singleline(&mut self.curr_hp_change);

                        ui.end_row();
                        ui.label(RichText::new("HP Temp: ").size(24.0));
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
                
            } else {
                ui.columns(3, |columns| {
                    columns[0].set_width(80.0);
                    columns[0].vertical_centered(|ui| {
                        ui.label(character.armor_class.to_string());
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

                draw_horizontal_line_at_least(ui, Vec2::new(380.0, 1.0), egui::Color32::from_gray(100));
            
                egui::Grid::new(format!("{}{}", "hp_grid", if unsafe {EDIT_MODE} { "edit" } else { "" }))
                .min_col_width(50.0)
                .show(ui, |ui| {
                    if unsafe {EDIT_MODE} {
                    } else {
                        ui.label(RichText::new("HP Max: ").size(14.0));
                        ui.label(RichText::new(character.maximum_hit_points.to_string()).size(14.0));
                        ui.end_row();
                        ui.label(RichText::new("HP Current: ").size(18.0));
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
                        ui.label(RichText::new("HP Temp: ").size(18.0));
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
                
            }
            
            
            
        });
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
