use eframe::{NativeOptions, run_native};
use egui::{CentralPanel, RichText, TopBottomPanel};

use calculator;

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

    fn render_buttons(&mut self, ctx: &egui::Context, ui: &mut egui::Ui) {
        let screen: egui::Rect = ctx.input(|i: &egui::InputState| i.screen_rect());
        let width = (&screen.max.x - &screen.min.x - 40.0) * 0.25;
        let height = (&screen.max.y - &screen.min.y - 80.0) * 0.20;

        egui::Grid::new("button_panel").show(ui, |ui| {
            if ui.add(egui::Button::new("CE").min_size(egui::Vec2::new(width, height))).clicked() {
                self.value = String::new();
            }
            if ui.add(egui::Button::new("C").min_size(egui::Vec2::new(width, height))).clicked() {}
            if ui.add(egui::Button::new("^").min_size(egui::Vec2::new(width, height))).clicked() {
                self.value += "^";
            }
            if ui.add(egui::Button::new("÷").min_size(egui::Vec2::new(width, height))).clicked() {
                self.value += "/";
            }
            ui.end_row();

            if ui.add(egui::Button::new("7").min_size(egui::Vec2::new(width, height))).clicked() {
                self.value += "7";
            }
            if ui.add(egui::Button::new("8").min_size(egui::Vec2::new(width, height))).clicked() {
                self.value += "8"
            }
            if ui.add(egui::Button::new("9").min_size(egui::Vec2::new(width, height))).clicked() {
                self.value += "9";
            }
            if ui.add(egui::Button::new("*").min_size(egui::Vec2::new(width, height))).clicked() {
                self.value += "*";
            }
            ui.end_row();

            if ui.add(egui::Button::new("4").min_size(egui::Vec2::new(width, height))).clicked() {
                self.value += "4";
            }
            if ui.add(egui::Button::new("5").min_size(egui::Vec2::new(width, height))).clicked() {
                self.value += "5"
            }
            if ui.add(egui::Button::new("6").min_size(egui::Vec2::new(width, height))).clicked() {
                self.value += "6";
            }
            if ui.add(egui::Button::new("-").min_size(egui::Vec2::new(width, height))).clicked() {
                self.value += "-";
            }
            ui.end_row();

            if ui.add(egui::Button::new("1").min_size(egui::Vec2::new(width, height))).clicked() {
                self.value += "1";
            }
            if ui.add(egui::Button::new("2").min_size(egui::Vec2::new(width, height))).clicked() {
                self.value += "2";
            }
            if ui.add(egui::Button::new("3").min_size(egui::Vec2::new(width, height))).clicked() {
                self.value += "3";
            }
            if ui.add(egui::Button::new("+").min_size(egui::Vec2::new(width, height))).clicked() {
                self.value += "+";
            }
            ui.end_row();

            if ui.add(egui::Button::new("%").min_size(egui::Vec2::new(width, height))).clicked() {}
            if ui.add(egui::Button::new("0").min_size(egui::Vec2::new(width, height))).clicked() {
                self.value += "0";
            }
            if ui.add(egui::Button::new(".").min_size(egui::Vec2::new(width, height))).clicked() {
                self.value += ".";
            }
            if ui.add(egui::Button::new("=").min_size(egui::Vec2::new(width, height))).clicked() {
                self.value += " ";
                self.value = calculator::translate_infix_to_postfix(self.value.clone());
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
            self.render_buttons(ctx, ui);
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
