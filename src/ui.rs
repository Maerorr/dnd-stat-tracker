use std::fmt::format;

use egui::{Context, Ui, Align, Sense};
use epaint::{Vec2, Pos2, Stroke};

use crate::{dnd_logic::prelude::*, app::{StatTracker, EDIT_MODE, AppState}};

pub fn character_select_ui(ctx: &Context, stat_tracker: &mut StatTracker) {
    egui::CentralPanel::default().show(ctx, |ui| {
        
        egui::TopBottomPanel::top("character_select_top_panel")
        .min_height(64.0)
        .show_inside(ui, |ui| {
            ui.vertical_centered(|ui| {
                ui.heading("Character Select");
            });
        });

        egui::SidePanel::left("character_select_left_panel")
        .exact_width(200.0)
        .resizable(false)
        .show_inside(ui, |ui| {
        });

        egui::SidePanel::right("character_select_right_panel")
        .exact_width(200.0)
        .resizable(false)
        .show_inside(ui, |ui| {
        });

        egui::CentralPanel::default()
        .show_inside(ui, |ui| {
            ui.vertical_centered(|ui| {
                ui.add_space(15.0);
                // split characters into slices of up to 5 characters each
                let mut up_to_five_characters = stat_tracker.characters.chunks_mut(5).collect::<Vec<_>>();
                let mut count = 0;
    
                for char in up_to_five_characters.iter_mut() {  
                    ui.horizontal(|ui| {
                        ui.add_space(10.0);
                        for character in char.iter_mut() {
                            let response = stat_tracker.ui_widgets.display_character_select_square(ui, character);
    
                            if response.clicked() {
                                stat_tracker.current_character = count;
                                stat_tracker.state = AppState::StatTracker;
                            }
                            count +=1;
                        }
                    });
                }                
            });
        });
        
       
    });

}

pub fn stat_tracker_ui(ctx: &Context, stat_tracker: &mut StatTracker) {
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
                    stat_tracker.ui_widgets.basic_character_info(ui, &mut stat_tracker.characters[stat_tracker.current_character]);

                    ui.with_layout(egui::Layout::right_to_left(Align::RIGHT), |ui| {
                        if unsafe {EDIT_MODE} {
                            let (but_rect, _) = ui.allocate_at_least(Vec2::new(90.0, 30.0), Sense::hover());
                            let edit_button_response = ui.put(but_rect, egui::Button::new("Stop Editing").min_size(Vec2::new(90.0, 30.0)));

                            if edit_button_response.clicked() {
                                stat_tracker.state = AppState::StatTracker;
                                unsafe { EDIT_MODE = false };
                                stat_tracker.first_frame = true;
                            }
                        } else {
                            let (but_rect, _) = ui.allocate_at_least(Vec2::new(90.0, 36.0), Sense::hover());
                            let edit_button_response = ui.put(but_rect, egui::Button::new("Edit").min_size(Vec2::new(90.0, 36.0)));

                            let (save_rect, _) = ui.allocate_at_least(Vec2::new(90.0, 36.0), Sense::hover());
                            let save_button_response = ui.put(save_rect, egui::Button::new("Save").min_size(Vec2::new(90.0, 36.0)));

                            let (champ_select_rect, _) = ui.allocate_at_least(Vec2::new(150.0, 36.0), Sense::hover());
                            let champ_select_button_response = ui.put(champ_select_rect, egui::Button::new("Save & Switch to Character Select").min_size(Vec2::new(150.0, 36.0)));

                            if save_button_response.clicked() {
                                stat_tracker.characters[stat_tracker.current_character].save_to_file();
                            }

                            if edit_button_response.clicked() {
                                stat_tracker.state = AppState::StatTrackerEdit;
                                unsafe { EDIT_MODE = true };
                                stat_tracker.first_frame = true;
                            }

                            if champ_select_button_response.clicked() {
                                stat_tracker.characters[stat_tracker.current_character].save_to_file();
                                stat_tracker.state = AppState::CharacterSelect;
                            }
                        }
                            
                    });

                });
                
            });
            
            
        });

        let left_side_panel = egui::SidePanel::left(format!("{}{}", "side_panel", unsafe {EDIT_MODE.to_string()}))
        .min_width(350.0)
        .resizable(false)
        .show_inside(ui, |ui| {
            ui.horizontal(|ui| {
                ui.add_space(15.0);
                stat_tracker.stats_ui(ui);
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
                    ui.add_space(10.0);
                    ui.heading("Saves");
                    stat_tracker.ui_widgets.display_saving_throws_proficiencies(ui, &mut stat_tracker.characters[stat_tracker.current_character]);
                    ui.heading("Proficiencies");
                    stat_tracker.ui_widgets.display_proficiencies(ui, &mut stat_tracker.characters[stat_tracker.current_character]);
                });

                ui.add_space(10.0);

                let (rect, _response) = ui.allocate_at_least(Vec2::new(2.0, 1500.0), Sense::hover());
                ui.painter().line_segment(
                    [Pos2::new(rect.left(), rect.top()), Pos2::new(rect.left(), rect.bottom())], 
                    Stroke::new(
                        1.0, 
                        egui::Color32::from_gray(60)
                    )
                );
                
                ui.vertical(|ui| {
                    ui.add_space(10.0);
                    stat_tracker.ui_widgets.display_health_stats(ui, &mut stat_tracker.characters[stat_tracker.current_character]);

                    stat_tracker.ui_widgets.display_money(ui, &mut stat_tracker.characters[stat_tracker.current_character]);
                });
            
            });
            
        });     

        egui::SidePanel::right(format!("{}{}", "right_side_panel", unsafe {EDIT_MODE.to_string()}))
        .min_width(ui.available_width())
        .resizable(false)
        .show_inside(ui, |ui| {
            ui.horizontal(|ui| {
                //ui.add_space(15.0);
                ui.vertical_centered(|ui| {
                    ui.heading("todo: switch button for spells/eq");
                    stat_tracker.ui_widgets.display_spell_list(ctx, ui, &mut stat_tracker.characters[stat_tracker.current_character]);
                });
                
                ui.add_space(15.0);
            });      
        });      
    });
}