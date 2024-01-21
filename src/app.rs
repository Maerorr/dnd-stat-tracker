use egui::{FontFamily, TextStyle, FontId, Sense, Align, Ui};
use egui_extras::StripBuilder;
use epaint::{Vec2, Rect, Pos2, Stroke};
use strum::IntoEnumIterator;

use crate::{ui_widgets::{UiWidgets, self, centered_label, centered_heading}, dnd_logic::prelude::*, ui::stat_tracker_ui};

//create global variable EDIT_MODE

pub static mut EDIT_MODE: bool = false;
pub const MAIN_COLOR: egui::Color32 = egui::Color32::from_gray(150);

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
    StatTrackerEdit,
}

pub struct StatTracker {
    pub state: AppState,
    pub characters: Vec<Character>,
    pub current_character: usize, // index of current character in characters
    pub ui_widgets: UiWidgets,
    pub first_frame: bool,
}

impl Default for StatTracker {
    fn default() -> Self {

        let def_char = Character::default();

        Self {
            state: AppState::StatTracker,
            characters: vec![def_char],
            current_character: 0,
            ui_widgets: UiWidgets::default(),
            first_frame: true,
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
                stat_tracker_ui(&ctx, self);
            },
            AppState::StatTrackerEdit => {
                stat_tracker_ui(&ctx, self);
            }
        }
    }
}

impl StatTracker {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        configure_text_styles(&cc.egui_ctx);
        let def_char = Character::test_character();

        Self {
            state: AppState::StatTracker,
            characters: vec![def_char],
            current_character: 0,
            ui_widgets: UiWidgets::default(),
            first_frame: true,
        }
    }

    pub fn stats_ui(&mut self, ui: &mut egui::Ui) {
        egui::Grid::new(format!("{}{}", "stats_grid", unsafe {EDIT_MODE.to_string()})).show(ui, |ui| {
            centered_heading(ui, "Stats");
            ui.end_row();
            for (i, stat) in StatType::iter().enumerate() {
                self.ui_widgets.single_stat_widget(ui, &mut self.characters[self.current_character as usize].stats.get_stat_mut(stat), i);
                ui.end_row();
            }
            
        });
    }
}