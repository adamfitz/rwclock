use chrono::{Local,Utc};
use eframe::egui;
use chrono_tz::Asia::Kolkata;
use chrono_tz::Europe::Berlin;
use chrono_tz::America::New_York;

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
        let local_time = calculate_time("local");
        ui.heading(format!("SYD:\t {local_time}"));
        // UTC
        let utc_time = calculate_time("utc");
        ui.heading(format!("UTC:\t {utc_time}"));
        // BANGALORE
        let blr_time = calculate_time("blr");
        ui.heading(format!("BLR:\t {blr_time}"));
        // ERDING
        let erd_time = calculate_time("erd");
        ui.heading(format!("ERD:\t {erd_time}"));
        // MIAMI
        let mia_time = calculate_time("mia");
        ui.heading(format!("MIA:\t {mia_time}"));
       });
   }
}


fn calculate_time(location: &str) -> String {
    // calculate time based on given location input
    match location {
        "local" => {
            let local_time = Local::now();
            local_time.format("%H:%M:%S").to_string()
        },
        "utc" => {
            let utc_time = Utc::now();
            utc_time.format("%H:%M:%S").to_string()
        },
        "blr" => {
            let blr_time = Utc::now().with_timezone(&Kolkata);
            blr_time.format("%H:%M:%S").to_string()
        },
        "erd" => {
            let erd_time = Utc::now().with_timezone(&Berlin);
            erd_time.format("%H:%M:%S").to_string()
        },
        "mia" => {
            let mia_time = Utc::now().with_timezone(&New_York);
            mia_time.format("%H:%M:%S").to_string()
        },
        _ => String::from("Invalid location"),
    }
}
