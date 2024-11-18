use eframe::egui::{self, CentralPanel, TopBottomPanel, Context, Layout, Ui};

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Flash",
        options,
        Box::new(|_cc| Box::new(FlashApp::new())),
    )
}

struct FlashApp;

impl FlashApp {
    fn new() -> Self {
        Self {}
    }

    fn render_custom_bar(ui: &mut Ui) {
        // Özel bar tasarımı
        ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing.x = 10.0; // Butonlar arasındaki boşluk
            ui.visuals_mut().override_text_color = Some(egui::Color32::LIGHT_GRAY);

            ui.label("Flash"); // Sol üst köşede uygulamanın ismi

            // Butonlar
            ui.add(egui::Button::new("Bots"));
            ui.add(egui::Button::new("Item Database"));
            ui.add(egui::Button::new("Proxy"));
            ui.add(egui::Button::new("Settings"));
            ui.add(egui::Button::new("+ Add Bot"));
            ui.add(egui::Button::new("- Remove Bot"));
        });
    }
}

impl eframe::App for FlashApp {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        TopBottomPanel::top("header").show(ctx, |ui| {
            FlashApp::render_custom_bar(ui);
        });

        CentralPanel::default().show(ctx, |ui| {
            ui.label("Welcome to Flash!");
        });
    }
}
