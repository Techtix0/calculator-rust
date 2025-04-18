use eframe::run_native;
use eframe::App;
use eframe::NativeOptions;
use egui::CentralPanel;
use egui::TopBottomPanel;

#[derive(Default)]
struct TestApp;

impl TestApp {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self::default()
    }
}

impl App for TestApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        TopBottomPanel::top("result_screen").show(ctx, |ui| {
            ui.heading("This is the TopBottomPanel");
        }); 

        CentralPanel::default().show(ctx, |ui| {
           ui.heading("This is the CentralPanel"); 
        });
    }
}

fn main() {
    let win_option = NativeOptions::default();
    let _ = run_native("TestApp", win_option, Box::new(|cc| Ok(Box::new(TestApp::new(cc)))));
}

