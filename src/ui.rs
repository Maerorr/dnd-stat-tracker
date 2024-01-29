use std::fmt::format;

use eframe::App;
use egui::{Align, Align2, Context, RichText, Sense, Ui};
use epaint::{FontId, Pos2, Stroke, Vec2};
use serde::de;

use crate::{app::{AppState, StatTracker, EDIT_MODE, MAIN_COLOR}, dnd_logic::{prelude::*, spell}, ui_widgets::{draw_horizontal_line_at_least, draw_vertical_line_at_least}};

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

                ui.horizontal(|ui|{
                    ui.with_layout(egui::Layout::centered_and_justified(egui::Direction::TopDown), |ui| {
                        ui.heading("Stat Tracker");
                    });
                });

                ui.horizontal(|ui| {
                    stat_tracker.ui_widgets.basic_character_info(ui, &mut stat_tracker.characters[stat_tracker.current_character]);

                    ui.with_layout(egui::Layout::right_to_left(Align::RIGHT), |ui| {
                        if unsafe {EDIT_MODE} {
                            let (but_rect, _) = ui.allocate_at_least(Vec2::new(90.0, 36.0), Sense::hover());
                            let edit_button_response = ui.put(but_rect, egui::Button::new("Stop Editing").min_size(Vec2::new(90.0, 36.0)));

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
                //ui.add_space(15.0);
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
                    ui.add_space(10.0);
                    draw_horizontal_line_at_least(ui, Vec2::new(250.0, 0.0), MAIN_COLOR);
                    ui.add_space(10.0);
                    ui.label("Proficiencies & Languages");
                    ui.add_space(20.0);
                    let text_split = stat_tracker.characters[stat_tracker.current_character].proficiencies_and_languages.split("\n").collect::<Vec<_>>();
                    for text in text_split.iter() {
                        ui.horizontal_wrapped(|ui| {
                            if !text.is_empty() {
                                ui.set_max_width(250.0);
                                ui.add(egui::Label::new(RichText::new(*text).size(18.0)));
                            }
                        });
                    }
                    // let (rect, _) = ui.allocate_exact_size(Vec2::new(250.0, 200.0), Sense::hover());
                    // ui.put(rect, egui::TextEdit::multiline(&mut stat_tracker.characters[stat_tracker.current_character].proficiencies_and_languages));
                    
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

                ui.vertical(|ui| {
                    // i couldn't center it without breaking the layout 💀
                    ui.label("                   Features & Traits");
                    
                    let text_split = stat_tracker.characters[stat_tracker.current_character].features_and_traits.split("\n").collect::<Vec<_>>();
                    for text in text_split.iter() {
                        ui.horizontal_wrapped(|ui| {
                            if text.is_empty() {
                                draw_horizontal_line_at_least(ui, Vec2::new(250.0, 0.0), MAIN_COLOR);
                            } else {
                                ui.set_max_width(300.0);
                                ui.add(egui::Label::new(RichText::new(*text).size(14.0)));
                            }
                        });
                    }
                });

                let (rect, _response) = ui.allocate_at_least(Vec2::new(2.0, 1500.0), Sense::hover());

                ui.painter().line_segment(
                    [Pos2::new(rect.left(), rect.top()), Pos2::new(rect.left(), rect.bottom())], 
                    Stroke::new(
                        1.0, 
                        egui::Color32::from_gray(60)
                    )
                );
                
                //ui.add_space(15.0);
                ui.vertical_centered(|ui| {
                    ui.horizontal(|ui|{
                        let char = &mut stat_tracker.characters[stat_tracker.current_character];
                        let ability = char.get_class().get_spellcasting_ability();
                        let spellcasting_ability = if ability.is_some() {
                            char.get_class().get_spellcasting_ability().unwrap().get_short_name()
                        } else {
                            "None".to_string()
                        };
                        let spell_save_dc = if ability.is_some() {
                            let num = 8 + char.stats.get_stat_modifier(ability.unwrap()) + char.proficiency_bonus;
                            num.to_string()
                        } else {
                            "None".to_string()
                        };

                        let spell_attack_bonus = if ability.is_some() {
                            let num = char.proficiency_bonus + char.stats.get_stat_modifier(ability.unwrap());
                            num.to_string()
                        } else {
                            "None".to_string()
                        };

                        let color = if ability.is_some() {
                            ability.unwrap().get_stat_color()
                        } else {
                            MAIN_COLOR
                        };
                        ui.label(RichText::new(format!("Spellcasting Ability: {}", spellcasting_ability)).color(color));
                        draw_vertical_line_at_least(ui, Vec2::new(0.0, 18.0), MAIN_COLOR);

                        ui.label(RichText::new(format!("Spell Save DC: {}", spell_save_dc)).color(color));
                        draw_vertical_line_at_least(ui, Vec2::new(0.0, 18.0), MAIN_COLOR);

                        ui.label(RichText::new(format!("Spell Attack Bonus: {}", spell_attack_bonus)).color(color));
                        draw_vertical_line_at_least(ui, Vec2::new(0.0, 18.0), MAIN_COLOR);
                    });
                    ui.heading("todo: switch button for spells/eq");
                    stat_tracker.ui_widgets.display_spell_list(ctx, ui, &mut stat_tracker.characters[stat_tracker.current_character]);
                });
                
                ui.add_space(15.0);
            });      
        });      
    });
}

