use eframe::egui::{self, CentralPanel, TopBottomPanel, Context};

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Flash",
        options,
        Box::new(|_cc| Box::new(Flash::new())),
    )
}

struct Flash;

impl Flash {
    fn new() -> Self {
        Self {}
    }
}

impl eframe::App for Flash {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        TopBottomPanel::top("header").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.add(egui::Button::new("Bots"));
                ui.add(egui::Button::new("Item Database"));
                ui.add(egui::Button::new("Proxy"));
                ui.add(egui::Button::new("Settings"));
                ui.add(egui::Button::new("+ Add Bot"));
                ui.add(egui::Button::new("- Remove Bot"));
            });
        });

        CentralPanel::default().show(ctx, |ui| {
            ui.label("Welcome to Flash!");
        });
    }
}
