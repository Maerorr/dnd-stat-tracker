use eframe::glow::Context;
use egui::{Response, RichText, Sense};
use epaint::{vec2, Rect, Vec2};
use serde::{Serialize, Deserialize};

use super::class::Class;


#[derive(Debug, PartialEq, Eq, Clone, Copy, Serialize, Deserialize)]
pub enum SchoolOfMagic {
    Abjuration,
    Conjuration,
    Divination,
    Enchantment,
    Evocation,
    Illusion,
    Necromancy,
    Transmutation,
    Dunamancy,
}

impl ToString for SchoolOfMagic {
    fn to_string(&self) -> String{
        match self {
            SchoolOfMagic::Abjuration => String::from("Abjuration"),
            SchoolOfMagic::Conjuration => String::from("Conjuration"),
            SchoolOfMagic::Divination => String::from("Divination"),
            SchoolOfMagic::Enchantment => String::from("Enchantment"),
            SchoolOfMagic::Evocation => String::from("Evocation"),
            SchoolOfMagic::Illusion => String::from("Illusion"),
            SchoolOfMagic::Necromancy => String::from("Necromancy"),
            SchoolOfMagic::Transmutation => String::from("Transmutation"),
            SchoolOfMagic::Dunamancy => String::from("Dunamancy"),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct Spell {
    pub name: String,
    pub level: i32,
    pub school: SchoolOfMagic,
    pub casting_time: String,
    pub range: String,
    pub target: String,
    pub components: String,
    pub duration: String,
    pub description: String,
    pub higher_levels: String,
    pub classes: Vec<Class>,
    pub ritual: bool,
    pub window_open: bool,
}

impl Spell {
    pub fn new(
        name: String,
        level: i32,
        school: SchoolOfMagic,
        casting_time: String,
        range: String,
        target: String,
        components: String,
        duration: String,
        description: String,
        classes: Vec<Class>,
        ritual: bool,
        higher_levels: String,
    ) -> Self {
        Self {
            name,
            level,
            school,
            casting_time,
            target,
            range,
            components,
            duration,
            description,
            higher_levels,
            classes,
            ritual,
            window_open: false,
        }
    }
    pub fn try_to_show_spell_window(&mut self, ctx: &egui::Context, ui: &mut egui::Ui) {
        egui::Window::new(format!("{}", self.name).to_string()).title_bar(true).open(&mut self.window_open)
            .vscroll(false)
            .resizable(true)
            .default_size([400.0, 500.0])
            .show(ctx, |ui| {
                ui.vertical(|ui| {
                    ui.separator();
                    ui.vertical(|ui| {
                        ui.label(RichText::new(format!("Level: {}", self.level)));
                        ui.label(RichText::new(format!("School: {}", self.school.to_string())));
                        ui.label(RichText::new(format!("Casting Time: {}", self.casting_time)));
                        ui.label(RichText::new(format!("Range: {}", self.range)));
                        ui.label(RichText::new(format!("Components: {}", self.components)));
                        ui.label(RichText::new(format!("Duration: {}", self.duration)));
                    });
                    ui.separator();
                    ui.label(self.description.clone());
                    ui.separator();
                    //bold text
                    ui.label(RichText::new(format!("At Higher Levels: {}", self.higher_levels)));
                });
                ui.allocate_space(ui.available_size());
            });
    }

    pub fn display_spell_name(&self, ui: &mut egui::Ui) {
        ui.horizontal_wrapped(|ui| {
            let (rect, response) = ui.allocate_at_least(vec2(130.0, 14.0), Sense::hover());
            ui.put(rect, egui::Label::new(RichText::new(format!("{}", self.name)).size(14.0)));
            //ui.label(RichText::new(format!("{}", self.name)).size(14.0));
        });
    }

    pub fn display_spell_more_button(&mut self, ui: &mut egui::Ui) {
        if ui.button("More").clicked() {
            if self.window_open {
                self.window_open = false;
            } else {
                self.window_open = true;
            }
        }
    }
}