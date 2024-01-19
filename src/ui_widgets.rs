use egui::Sense;
use epaint::{Vec2, Stroke};
use strum::IntoEnumIterator;

use crate::{dnd_utils::{Stat, StatType, SkillType}, character::Character};

pub struct UiWidgets;


impl UiWidgets {
    pub fn single_stat_widget(&self, ui: &mut egui::Ui, stat: &mut Stat, i: usize) {
        egui::Grid::new(format!("stat_{}", i))
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
            if ui.button("-").clicked() {
                stat.subtract_one();
            }
            if ui.button("+").clicked() {
                stat.add_one();
            }
        });
    }

    pub fn basic_character_info(&self, ui: &mut egui::Ui, character: &mut Character) {
        ui.horizontal_centered(|ui| {
            ui.horizontal(|ui| {
                ui.label("Name:");
                ui.label(&character.name)
            });
            ui.horizontal(|ui| {
                ui.label("Level:");
                ui.label(&character.level.to_string())
            });
            ui.horizontal(|ui| {
                ui.label("Class:");
                ui.label(&character.class.get_name())
            });
            ui.horizontal(|ui| {
                ui.label("Experience:");
                ui.label(&character.experience.to_string())
            });
        });
    }

    pub fn display_saving_throws_proficiencies(&self, ui: &mut egui::Ui, character: &mut Character) {
        egui::Grid::new("saving_throws_grid")
        .show(ui, |ui| {
            for stat in StatType::iter() {
                    ui.label(stat.get_name());
                    // paint a filled circle if yes, empty circle if no
                    let (rect, _response) = ui.allocate_at_least(Vec2::new(10.0, 10.0), Sense::hover());
                    if character.stats.get_stat_saving_throw_proficiency(stat) {
                        // draw a filled circle using epaint
                        ui.painter().circle_filled(rect.center(), 5.0, egui::Color32::from_rgb(255, 255, 255));
                    } else {
                        ui.painter().circle_stroke(rect.center(), 5.0, Stroke::new(1.0, egui::Color32::from_rgb(255, 255, 255)));
                    }
                    ui.end_row();
            }
        });
    }

    pub fn display_proficiencies(&self, ui: &mut egui::Ui, character: &mut Character) {
        egui::Grid::new("proficiencies_grid")
        .min_col_width(15.0)
        .show(ui, |ui| {
            for skill in SkillType::iter() {
                ui.label(skill.get_name());
                // paint a filled circle if yes, empty circle if no
                let (rect, _response) = ui.allocate_at_least(Vec2::new(10.0, 10.0), Sense::hover());
                if character.skills.get_skill_proficiency(skill) {
                    // draw a filled circle using epaint
                    ui.painter().circle_filled(rect.center(), 5.0, egui::Color32::from_rgb(255, 255, 255));
                } else {
                    ui.painter().circle_stroke(rect.center(), 5.0, Stroke::new(1.0, egui::Color32::from_rgb(255, 255, 255)));
                }
                if (character.skills.get_skill_expertise(skill)) {
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