use eframe::{NativeOptions, run_native};
use egui::{CentralPanel, RichText, TopBottomPanel};

#[derive(Default)]
struct TestApp {
    value: f64,
}

impl TestApp {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self::default()
    }
}

impl eframe::App for TestApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        TopBottomPanel::top("result_screen").show(ctx, |ui| {
            ui.label(
                RichText::new(self.value.to_string())
                    .font(egui::FontId::new(40.0, egui::FontFamily::Monospace)),
            )
        });

        CentralPanel::default().show(ctx, |ui| {
            if ui.button("Increment").clicked() {
                self.value += 1.0;
            }
        });
    }
}

fn main() -> eframe::Result {
    let native_options = NativeOptions::default();
    run_native(
        "Calculator",
        native_options,
        Box::new(|cc| Ok(Box::new(TestApp::new(cc))))
    )
}
