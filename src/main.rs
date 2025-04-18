use eframe::{NativeOptions, run_native};
use egui::{CentralPanel, RichText, TopBottomPanel};

#[derive(Default)]
struct TestApp;

impl TestApp {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self::default()
    }
}

impl eframe::App for TestApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        TopBottomPanel::top("result_screen").show(ctx, |ui| {
            ui.label(
                RichText::new("0")
                    .font(egui::FontId::new(40.0, egui::FontFamily::Monospace)),
            )
        });

        CentralPanel::default().show(ctx, |ui| {
            if ui.button("1").clicked() {
            }
        });
    }
}

fn main() {
    let win_option = NativeOptions::default();
    let _ = run_native(
        "TestApp",
        win_option,
        Box::new(|cc| Ok(Box::new(TestApp::new(cc)))),
    );
}
