use egui::{Sense, RichText};
use epaint::{Vec2, Stroke, Pos2, Rounding};
use strum::IntoEnumIterator;

use crate::{character::Character, app::EDIT_MODE, dnd_logic::prelude::*};

pub struct UiWidgets {
    exp_change: String
}

impl Default for UiWidgets {
    fn default() -> Self {
        Self {
            exp_change: String::new(),
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

            draw_line_at_least(ui, Vec2::new(1.0, 25.0), egui::Color32::from_gray(100));

            ui.label("Name:");
            if unsafe { EDIT_MODE } {
                ui.text_edit_singleline(&mut character.name);
            } else {
                ui.label(&character.name);
            }

            draw_line_at_least(ui, Vec2::new(1.0, 25.0), egui::Color32::from_gray(100));

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

            draw_line_at_least(ui, Vec2::new(1.0, 25.0), egui::Color32::from_gray(100));

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

            draw_line_at_least(ui, Vec2::new(1.0, 25.0), egui::Color32::from_gray(100));

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

            draw_line_at_least(ui, Vec2::new(1.0, 25.0), egui::Color32::from_gray(100));
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

pub fn draw_line_at_least(ui: &mut egui::Ui, vec2: Vec2, color: egui::Color32) {
    let (rect, _response) = ui.allocate_at_least(vec2, Sense::hover());

    ui.painter().line_segment(
        [Pos2::new(rect.left(), rect.top()), Pos2::new(rect.left(), rect.bottom())], 
        Stroke::new(
            1.0, 
            color
        )
    );
}

pub fn proficiency_edit(ui: &mut egui::Ui, on: &mut bool) -> egui::Response {
    let (rect, mut response) = ui.allocate_at_least(Vec2::new(12.0, 12.0), Sense::click());

    response.widget_info(|| egui::WidgetInfo::selected(egui::WidgetType::Checkbox, *on, ""));
    if response.clicked() {
        *on = !*on;
        response.mark_changed();
    }
    let mut rounding = Rounding::default();
    rounding.at_least(4.0);
    if *on {
        ui.painter().rect_filled(rect, rounding, egui::Color32::from_rgb(255, 255, 255));
    } else {
        ui.painter().rect_stroke(rect, rounding, Stroke::new(1.0, egui::Color32::from_rgb(255, 255, 255)));
    }

    response
}

pub fn proficiency_edit_switch(on: &mut bool) -> impl egui::Widget + '_ {
    move |ui: &mut egui::Ui| proficiency_edit(ui, on)
}