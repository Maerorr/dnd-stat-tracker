use egui::{FontFamily, TextStyle, FontId, Sense, Align, Ui};
use egui_extras::StripBuilder;
use epaint::{Vec2, Rect, Pos2, Stroke};
use strum::IntoEnumIterator;

use crate::{character::Character, ui_widgets::{UiWidgets, self, centered_label, centered_heading}, dnd_logic::stat_type::StatType};

//create global variable EDIT_MODE

pub static mut EDIT_MODE: bool = false;

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
    state: AppState,
    characters: Vec<Character>,
    current_character: usize, // index of current character in characters
    ui_widgets: UiWidgets,
    first_frame: bool,
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
                egui::CentralPanel::default().show(ctx, |ui| {
                    //create a top panel
                    egui::TopBottomPanel::top("top_panel")
                    .min_height(64.0)
                    .show_inside(ui, |ui| {
                        ui.vertical(|ui| {
                            ui.vertical_centered(|ui| {
                                ui.heading("Stat Tracker");
                            });
    
                            ui.horizontal(|ui| {
                                self.ui_widgets.basic_character_info(ui, &mut self.characters[self.current_character]);

                                ui.with_layout(egui::Layout::right_to_left(Align::RIGHT), |ui| {
                                        let (but_rect, _) = ui.allocate_at_least(Vec2::new(90.0, 30.0), Sense::hover());
                                        let edit_button_response = ui.put(but_rect, egui::Button::new("Edit").min_size(Vec2::new(90.0, 30.0)));

                                        if edit_button_response.clicked() {
                                            self.state = AppState::StatTrackerEdit;
                                            unsafe { EDIT_MODE = true };
                                            self.first_frame = true;
                                        }
                                });

                            });
                            
                        });
                        
                        
                    });

                    let side_panel = egui::SidePanel::left("stat_panel")
                    .min_width(350.0)
                    .resizable(true)
                    .show_inside(ui, |ui| {
                        ui.horizontal(|ui| {
                            self.stats_ui(ui);
                            ui.add_space(10.0);

                            // height of 1500, assuming the panel willalways span to the very bottom of the window
                            let (rect, _response) = ui.allocate_at_least(Vec2::new(2.0, 1500.0), Sense::hover());

                            ui.painter().line_segment(
                                [Pos2::new(rect.left(), rect.top()), Pos2::new(rect.left(), rect.bottom())], 
                                Stroke::new(
                                    1.0, 
                                    egui::Color32::from_gray(60)
                                )
                            );

                            ui.add_space(10.0);
                            ui.vertical(|ui| {
                                ui.heading("Saves");
                                self.ui_widgets.display_saving_throws_proficiencies(ui, &mut self.characters[self.current_character]);
                                ui.heading("Proficiencies");
                                self.ui_widgets.display_proficiencies(ui, &mut self.characters[self.current_character]);
                            });
                        
                        });
                        
                    });                  
                });
            },
            AppState::StatTrackerEdit => {
                egui::CentralPanel::default().show(ctx, |ui| {
                    //create a top panel
                    egui::TopBottomPanel::top("top_panel")
                    .min_height(64.0)
                    .show_inside(ui, |ui| {
                        ui.vertical(|ui| {
                            ui.vertical_centered(|ui| {
                                ui.heading("Stat Tracker");
                            });
    
                            ui.horizontal(|ui| {
                                self.ui_widgets.basic_character_info(ui, &mut self.characters[self.current_character]);

                                ui.with_layout(egui::Layout::right_to_left(Align::RIGHT), |ui| {
                                        let (but_rect, _) = ui.allocate_at_least(Vec2::new(120.0, 30.0), Sense::hover());
                                        let edit_button_response = ui.put(but_rect, egui::Button::new("End Editing").min_size(Vec2::new(120.0, 30.0)));

                                        if edit_button_response.clicked() {
                                            self.state = AppState::StatTracker;
                                            unsafe { EDIT_MODE = false };
                                            self.first_frame = true;
                                        }
                                });

                            });
                            
                        });
                        
                        
                    });

                    egui::SidePanel::left(format!("{}{}", "stat_panel", unsafe {EDIT_MODE.to_string()}))
                    .min_width(350.0)
                    .resizable(true)
                    .show_inside(ui, |ui| {
                        ui.horizontal(|ui| {
                            self.stats_ui(ui);
                            ui.add_space(10.0);

                            // height of 1500, assuming the panel willalways span to the very bottom of the window
                            let (rect, _response) = ui.allocate_at_least(Vec2::new(2.0, 1500.0), Sense::hover());

                            ui.painter().line_segment(
                                [Pos2::new(rect.left(), rect.top()), Pos2::new(rect.left(), rect.bottom())], 
                                Stroke::new(
                                    1.0, 
                                    egui::Color32::from_gray(60)
                                )
                            );

                            ui.add_space(10.0);
                            ui.vertical(|ui| {
                                ui.heading("Saves");
                                self.ui_widgets.display_saving_throws_proficiencies(ui, &mut self.characters[self.current_character]);
                                ui.heading("Proficiencies");
                                self.ui_widgets.display_proficiencies(ui, &mut self.characters[self.current_character]);
                            });
                        
                        });
                        
                    });                  
                });
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