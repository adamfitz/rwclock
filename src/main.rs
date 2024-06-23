use chrono::{Local,Utc};
use eframe::egui;
use egui::RichText;

fn main() {
    let native_options = eframe::NativeOptions::default();
    let _ = eframe::run_native("World clock", native_options, Box::new(|cc| Box::new(MyWorldClockApp::new(cc))));
}

#[derive(Default)]
struct MyWorldClockApp {}

impl MyWorldClockApp {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.
        Self::default()
    }
}

impl eframe::App for MyWorldClockApp {
   fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.request_repaint();
        egui::CentralPanel::default().show(ctx, |ui: &mut egui::Ui| {

        // local time
        let local_time = local_time();
        ui.heading("Local time:");
        ui.heading(RichText::new(local_time));

        // UTC time
        let utc_time = utc_time();
        ui.heading("GMT time:");
        ui.heading(RichText::new(utc_time));
       });
   }
}


fn local_time() -> String {
    // return local time
    let local_time = Local::now();
    let local_time_formatted = format!("{}", local_time.format("%H:%M:%S"));
    local_time_formatted
}

fn utc_time() -> String {
    // return local time
    let utc_time = Utc::now();
    let utc_time_formatted = format!("{}", utc_time.format("%H:%M:%S"));
    utc_time_formatted
}