use eframe::glow::Context;
use egui::{Response, RichText, Sense};
use epaint::{vec2, Rect, Vec2};
use serde::{Serialize, Deserialize};


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
            window_open: false,
        }
    }

    pub fn display_spell_window(&mut self) {
        self.window_open = true;
    }

    pub fn try_to_show_spell_window(&mut self, ctx: &egui::Context, ui: &mut egui::Ui) {
        let window = egui::Window::new(format!("{}{}", self.name, self.level.to_string())).title_bar(false).open(&mut self.window_open)
            .vscroll(false)
            .resizable(true)
            .default_size([400.0, 500.0])
            .show(ctx, |ui| {
                ui.vertical_centered(|ui|{
                    ui.heading(RichText::new(format!("{}", self.name)));
                });
                
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

    pub fn display_spell_name(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.label(RichText::new(format!("{}", self.name)).size(14.0));
        });
    }

    pub fn display_spell_more_button(&mut self, ctx: &egui::Context, ui: &mut egui::Ui) {
        if ui.button("More").clicked() {
            if self.window_open {
                self.close_spell_window();
            } else {
                self.display_spell_window();
            }
        }
        self.try_to_show_spell_window(ctx, ui);
    }

    pub fn close_spell_window(&mut self) {
        self.window_open = false;
    }
}