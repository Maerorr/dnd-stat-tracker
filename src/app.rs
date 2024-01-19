use egui::{FontFamily, TextStyle, FontId};
use egui_extras::StripBuilder;
use strum::IntoEnumIterator;

use crate::{character::Character, dnd_utils::{Stats, StatType}, ui_widgets::{UiWidgets, self, centered_label, centered_heading}};

fn configure_text_styles(ctx: &egui::Context) {
    use FontFamily::Proportional;
    use TextStyle::*;

    let mut style = (*ctx.style()).clone();
    style.text_styles = [
        (Heading, FontId::new(30.0, Proportional)),
        (Body, FontId::new(18.0, Proportional)),
        (Monospace, FontId::new(14.0, Proportional)),
        (Button, FontId::new(14.0, Proportional)),
        (Small, FontId::new(10.0, Proportional)),
    ]
    .into();
    ctx.set_style(style);
}

pub enum AppState {
    CharacterSelect,
    CharacterCreate,
    StatTracker,
}

pub struct StatTracker {
    state: AppState,
    characters: Vec<Character>,
    current_character: usize, // index of current character in characters
    ui_widgets: UiWidgets,
}

impl Default for StatTracker {
    fn default() -> Self {

        let def_char = Character::default();

        Self {
            state: AppState::StatTracker,
            characters: vec![def_char],
            current_character: 0,
            ui_widgets: UiWidgets,
        }
    }
}

impl eframe::App for StatTracker {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        match self.state {
            AppState::CharacterSelect => {
                egui::CentralPanel::default().show(ctx, |ui| {
                    ui.heading("Character Select");
                    ui.horizontal(|ui| {
                        if ui.button("Create Character").clicked() {
                            self.state = AppState::CharacterCreate;
                        }
                    });
                });
            }
            AppState::CharacterCreate => {
                egui::CentralPanel::default().show(ctx, |ui| {
                    ui.heading("Character Create");
                    ui.horizontal(|ui| {
                        if ui.button("Back").clicked() {
                            self.state = AppState::CharacterSelect;
                        }
                    });
                });
            }
            AppState::StatTracker => {
                egui::CentralPanel::default().show(ctx, |ui| {
                    //create a top panel
                    egui::TopBottomPanel::top("top_panel")
                    .min_height(64.0)
                    .show_inside(ui, |ui| {
                        ui.vertical_centered(|ui| {
                            ui.heading("Stat Tracker");
                            self.ui_widgets.basic_character_info(ui, &mut self.characters[self.current_character]);
                        });
                        
                    });

                    egui::SidePanel::left("stat_panel")
                    .min_width(330.0)
                    .resizable(true)
                    .show_inside(ui, |ui| {
                        ui.horizontal(|ui| {
                            self.stats_ui(ui);
                            ui.vertical(|ui| {
                                ui.heading("Saves");
                                self.ui_widgets.display_saving_throws_proficiencies(ui, &mut self.characters[self.current_character]);
                                ui.heading("Proficiencies");
                                self.ui_widgets.display_proficiencies(ui, &mut self.characters[self.current_character]);
                            });
                        });
                        
                    });
                    
                    ui.horizontal(|ui| {
                        if ui.button("Back").clicked() {
                            self.state = AppState::CharacterSelect;
                        }
                    });
                });
            }
        }
    }
}

impl StatTracker {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        configure_text_styles(&cc.egui_ctx);
        let def_char = Character::default();

        Self {
            state: AppState::StatTracker,
            characters: vec![def_char],
            current_character: 0,
            ui_widgets: UiWidgets,
        }
    }

    pub fn stats_ui(&mut self, ui: &mut egui::Ui) {
        egui::Grid::new("stats_grid").show(ui, |ui| {
            centered_heading(ui, "Stats");
            ui.end_row();
            for (i, stat) in StatType::iter().enumerate() {
                self.ui_widgets.single_stat_widget(ui, &mut self.characters[self.current_character as usize].stats.get_stat(stat), i);
                ui.end_row();
            }
            
        });
    }
}