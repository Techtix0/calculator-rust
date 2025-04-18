use eframe::{NativeOptions, run_native};
use egui::{CentralPanel, RichText, TopBottomPanel};

#[derive(Default)]
struct TestApp {
    value: String,
}

impl TestApp {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            value: String::new(),
        }
    }

    fn render_buttons(&mut self, ui: &mut egui::Ui) {
        ui.with_layout(egui::Layout::left_to_right(egui::Align::TOP), |ui| {
            if ui.add(egui::Button::new("clear").min_size(egui::Vec2::new(100.0, 100.0))).clicked() {
                self.value = String::new();
            }

            if ui.add(egui::Button::new("^").min_size(egui::Vec2::new(100.0, 100.0))).clicked() {
            }

            if ui.add(egui::Button::new("9").min_size(egui::Vec2::new(100.0, 100.0))).clicked() {
            }

            if ui.add(egui::Button::new("=").min_size(egui::Vec2::new(100.0, 100.0))).clicked() {
            }
        });

        ui.add_space(5.0);

        ui.with_layout(egui::Layout::left_to_right(egui::Align::TOP), |ui| {
            if ui.add(egui::Button::new("7").min_size(egui::Vec2::new(100.0, 100.0))).clicked() {
                self.value += "7";
            }

            if ui.add(egui::Button::new("8").min_size(egui::Vec2::new(100.0, 100.0))).clicked() {
                self.value += "8"
            }

            if ui.add(egui::Button::new("9").min_size(egui::Vec2::new(100.0, 100.0))).clicked() {
                self.value += "9";
            }

            if ui.add(egui::Button::new("÷").min_size(egui::Vec2::new(100.0, 100.0))).clicked() {
            }

        });

        ui.add_space(5.0);

        ui.with_layout(egui::Layout::left_to_right(egui::Align::TOP), |ui| {
            if ui.add(egui::Button::new("4").min_size(egui::Vec2::new(100.0, 100.0))).clicked() {
                self.value += "4";
            }

            if ui.add(egui::Button::new("5").min_size(egui::Vec2::new(100.0, 100.0))).clicked() {
                self.value += "5"
            }

            if ui.add(egui::Button::new("6").min_size(egui::Vec2::new(100.0, 100.0))).clicked() {
                self.value += "6";
            }

            if ui.add(egui::Button::new("*").min_size(egui::Vec2::new(100.0, 100.0))).clicked() {
            }
        });

        ui.add_space(5.0);

        ui.with_layout(egui::Layout::left_to_right(egui::Align::TOP), |ui| {
            if ui.add(egui::Button::new("1").min_size(egui::Vec2::new(100.0, 100.0))).clicked() {
                self.value += "1";
            }

            if ui.add(egui::Button::new("2").min_size(egui::Vec2::new(100.0, 100.0))).clicked() {
                self.value += "2";
            }

            if ui.add(egui::Button::new("3").min_size(egui::Vec2::new(100.0, 100.0))).clicked() {
                self.value += "3";
            }

            if ui.add(egui::Button::new("-").min_size(egui::Vec2::new(100.0, 100.0))).clicked() {
            }
        });

        ui.add_space(5.0);

        ui.with_layout(egui::Layout::left_to_right(egui::Align::TOP), |ui| {
            if ui.add(egui::Button::new("0").min_size(egui::Vec2::new(100.0, 100.0))).clicked() {
                self.value += "0";
            }

            if ui.add(egui::Button::new(".").min_size(egui::Vec2::new(100.0, 100.0))).clicked() {
                self.value += ".";
            }

            if ui.add(egui::Button::new("=").min_size(egui::Vec2::new(100.0, 100.0))).clicked() {
            }

            if ui.add(egui::Button::new("+").min_size(egui::Vec2::new(100.0, 100.0))).clicked() {
            }
        });
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

        CentralPanel::default().show(ctx, |ui: &mut egui::Ui| {
            self.render_buttons(ui);
        });
    }
}

fn main() -> eframe::Result {
    let native_options = NativeOptions::default();
    run_native(
        "Calculator",
        native_options,
        Box::new(|cc| Ok(Box::new(TestApp::new(cc)))),
    )
}
