use std::fs::File;

use egui::{FontFamily, TextStyle, FontId, Sense, Align, Ui};
use egui_extras::StripBuilder;
use epaint::{Vec2, Rect, Pos2, Stroke};
use serde_json::from_reader;
use strum::IntoEnumIterator;

use crate::{ui_widgets::{UiWidgets, self, centered_label, centered_heading}, dnd_logic::{character, prelude::*, spell}, ui::*};

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

pub const SPELLS_PATH: &str = "res/spells";
pub const CHARACTERS_PATH: &str = "res/characters";

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
    let path = std::path::Path::new(SPELLS_PATH);
    let cantrips_json = File::open(path.join("cantrips.json")).unwrap();
    
    let mut spell_database = SpellList::default();
    spell_database.cantrips = {
        let spells: Vec<Spell> = serde_json::from_reader(cantrips_json).unwrap();
        let spells = spells.into_iter()
        .map(|x| (x, false))
        .collect::<Vec<(Spell, bool)>>();
        spells
    };
    for i in 0..9 {
        let name = format!("{}{}", i+1, "_lvl_spells.json");
        let file = File::open(path.join(name)).unwrap();
        spell_database.spells[i] = {
            let spells: Vec<Spell> = serde_json::from_reader(file).unwrap();
            let spells = spells.into_iter()
            .map(|x| (x, false))
            .collect::<Vec<(Spell, bool)>>();
            spells
        };
    }    spell_database
}

fn load_characters(spell_database: &SpellList) -> Vec<Character> {
    let path = std::path::Path::new(CHARACTERS_PATH);
    let mut characters = Vec::new();
    for entry in std::fs::read_dir(path).unwrap() {
        let entry = entry.unwrap();
        let file = File::open(entry.path()).unwrap();
        let mut character: Character = serde_json::from_reader(file).unwrap();
        character.load_spells(spell_database);
        characters.push(character);
    }
    characters
}


#[derive(PartialEq, Eq, Clone, Copy)]
pub enum AppState {
    CharacterSelect,
    CharacterCreate,
    StatTracker,
    StatTrackerEdit,
}

pub struct StatTracker {
    pub state: AppState,
    pub previous_state: AppState,
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
        let saved_characters = load_characters(&spell_database);

        Self {
            state: AppState::CharacterSelect,
            previous_state: AppState::CharacterSelect,
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
                if self.previous_state != AppState::CharacterSelect {
                    self.previous_state = AppState::CharacterSelect;
                    self.characters.clear();
                    self.characters = load_characters(&self.spell_database);
                }
                character_select_ui(&ctx, self);
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
                self.previous_state = AppState::CharacterCreate;
            }
            AppState::StatTracker => {
                stat_tracker_ui(&ctx, self);
                self.previous_state = AppState::StatTracker;
            },
            AppState::StatTrackerEdit => {
                stat_tracker_ui(&ctx, self);
                self.previous_state = AppState::StatTrackerEdit;
            }
        }
    }
}

impl StatTracker {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        configure_text_styles(&cc.egui_ctx);
        let spell_database = load_spells_from_files();
        let mut def_char = Character::test_character();

        let characters = load_characters(&spell_database);

        // add all spells from spell database to character
        for spell in spell_database.cantrips.iter() {
            def_char.add_spell(&spell.0);
        }
        for spell_list in spell_database.spells.iter() {
            for spell in spell_list.iter() {
                def_char.add_spell(&spell.0);
            }
        }

        let mut ui_widgets = UiWidgets::default();
        ui_widgets.set_spell_database(spell_database.clone());

        Self {
            state: AppState::CharacterSelect,
            previous_state: AppState::CharacterSelect,
            characters: characters,
            current_character: 0,
            ui_widgets,
            first_frame: true,
            spell_database,
        }
    }

    pub fn stats_ui(&mut self, ui: &mut egui::Ui) {
        egui::Grid::new(format!("{}{}", "stats_grid", unsafe {EDIT_MODE.to_string()}))
        .show(ui, |ui| {
            centered_heading(ui, "Stats");
            ui.end_row();
            for (i, stat) in StatType::iter().enumerate() {
                self.ui_widgets.single_stat_widget(ui, &mut self.characters[self.current_character as usize].stats.get_stat_mut(stat), i);
                ui.end_row();
            }
        });
    }
}