use std::fs::File;

use egui::{FontFamily, TextStyle, FontId, Sense, Align, Ui};
use egui_extras::StripBuilder;
use epaint::{Vec2, Rect, Pos2, Stroke};
use strum::IntoEnumIterator;

use crate::{ui_widgets::{UiWidgets, self, centered_label, centered_heading}, dnd_logic::prelude::*, ui::stat_tracker_ui};

//create global variable EDIT_MODE

pub static mut EDIT_MODE: bool = false;
pub const MAIN_COLOR: egui::Color32 = egui::Color32::from_gray(150);

pub const COPPER_COLOR: egui::Color32 = egui::Color32::from_rgb(184,115,51);
pub const SILVER_COLOR: egui::Color32 = egui::Color32::from_rgb(192,192,192);
pub const ELECTRUM_COLOR: egui::Color32 = egui::Color32::from_rgb(143, 168, 179);
pub const GOLD_COLOR: egui::Color32 = egui::Color32::from_rgb(255,215,0);
pub const PLATINUM_COLOR: egui::Color32 = egui::Color32::from_rgb(229,228,226);
pub const CURRENT_HP_COLOR: egui::Color32 = egui::Color32::from_rgb(222, 120, 121);
pub const TEMP_HP_COLOR: egui::Color32 = egui::Color32::from_rgb(168, 221, 240);

pub const STRENGTH_COLOR: egui::Color32 = egui::Color32::from_rgb(193,96,77);
pub const DEXTERITY_COLOR: egui::Color32 = egui::Color32::from_rgb(84,222,178);
pub const CONSTITUTION_COLOR: egui::Color32 = egui::Color32::from_rgb(236,207,73);
pub const INTELLIGENCE_COLOR: egui::Color32 = egui::Color32::from_rgb(140,196,123);
pub const WISDOM_COLOR: egui::Color32 = egui::Color32::from_rgb(171,98,156);
pub const CHARISMA_COLOR: egui::Color32 = egui::Color32::from_rgb(233,219,204);

// zielony, fioletowy, niebieski czerwony zolty bialy

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

fn load_spells_from_files() -> SpellList {
    let path = std::path::Path::new("res/spells");
    let spells_1_lvl_json = File::open(path.join("1_lvl_spells.json")).unwrap();
    let spells_2_lvl_json = File::open(path.join("2_lvl_spells.json")).unwrap();
    // let spells_3_lvl_json = File::open(path.join("3_lvl_spells.json")).unwrap();
    // let spells_4_lvl_json = File::open(path.join("4_lvl_spells.json")).unwrap();
    // let spells_5_lvl_json = File::open(path.join("5_lvl_spells.json")).unwrap();
    // let spells_6_lvl_json = File::open(path.join("6_lvl_spells.json")).unwrap();
    // let spells_7_lvl_json = File::open(path.join("7_lvl_spells.json")).unwrap();
    // let spells_8_lvl_json = File::open(path.join("8_lvl_spells.json")).unwrap();
    // let spells_9_lvl_json = File::open(path.join("9_lvl_spells.json")).unwrap();
    
    let mut spell_database = SpellList::default();
    spell_database.spells_1_lvl = serde_json::from_reader(spells_1_lvl_json).unwrap();
    spell_database.spells_2_lvl = serde_json::from_reader(spells_2_lvl_json).unwrap();
    // spell_database.spells_3_lvl = serde_json::from_reader(spells_3_lvl_json)).unwrap();
    // spell_database.spells_4_lvl = serde_json::from_reader(spells_4_lvl_json)).unwrap();
    // spell_database.spells_5_lvl = serde_json::from_reader(spells_5_lvl_json)).unwrap();
    // spell_database.spells_6_lvl = serde_json::from_reader(spells_6_lvl_json)).unwrap();
    // spell_database.spells_7_lvl = serde_json::from_reader(spells_7_lvl_json)).unwrap();
    // spell_database.spells_8_lvl = serde_json::from_reader(spells_8_lvl_json)).unwrap();
    // spell_database.spells_9_lvl = serde_json::from_reader(spells_9_lvl_json)).unwrap();

    spell_database
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
    pub spell_database: SpellList,
}

impl Default for StatTracker {
    fn default() -> Self {

        let def_char = Character::default();

        let spell_database = load_spells_from_files();

        Self {
            state: AppState::StatTracker,
            characters: vec![def_char],
            current_character: 0,
            ui_widgets: UiWidgets::default(),
            first_frame: true,
            spell_database,
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
        let spell_database = load_spells_from_files();
        let mut def_char = Character::test_character();
        def_char.spell_list.add_spell(&spell_database.spells_1_lvl[0]);
        def_char.spell_list.add_spell(&spell_database.spells_1_lvl[1]);
        def_char.spell_list.add_spell(&spell_database.spells_1_lvl[2]);

        def_char.spell_list.add_spell(&spell_database.spells_2_lvl[0]);
        def_char.spell_list.add_spell(&spell_database.spells_2_lvl[1]);
        def_char.spell_list.add_spell(&spell_database.spells_2_lvl[2]);

        Self {
            state: AppState::StatTracker,
            characters: vec![def_char],
            current_character: 0,
            ui_widgets: UiWidgets::default(),
            first_frame: true,
            spell_database,
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