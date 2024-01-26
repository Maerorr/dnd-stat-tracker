use eframe::{egui, Renderer};

mod app;
mod ui_widgets;
mod ui;

mod dnd_logic;

use app::StatTracker;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_maximized(true),
        renderer: Renderer::Glow,
        ..Default::default()
    };
    eframe::run_native(
        "Maeror's D&D Stat Tracker",
        options,
        Box::new(|cc| {
            // This gives us image support:
            egui_extras::install_image_loaders(&cc.egui_ctx);

            Box::new(StatTracker::new(cc))
        }),
    )
}

