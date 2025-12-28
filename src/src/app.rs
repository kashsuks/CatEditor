use eframe::egui;
use crate::menu;

pub struct CatEditorApp {
    pub text: String,
}

impl Default for CatEditorApp {
    fn default() -> Self {
        Self {
            text: String::new(),
        }
    }
}

impl eframe::App for CatEditorApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        //show the menu bar
        menu::show_menu_bar(ctx, self);

        // main text editor area
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.add(
                    egui::TextEdit::multiline(&mut self.text)
                        .desired_width(f32::INFINITY)
                        .font(egui::TextStyle::Monospace)
                );
            });
        });
    }
}