use eframe::{egui, epi};

mod ui;
mod api;

struct CratesIoApp;

impl epi::App for CratesIoApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut epi::Frame) {
        ui::create_ui(ctx);
    }

    fn name(&self) -> &str {
        "Crates.io Desktop"
    }
}

fn main() {
    let app = CratesIoApp;
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(Box::new(app), native_options);
}
