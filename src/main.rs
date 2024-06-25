use chrono::{Local,Utc};
use eframe::egui;
use crate::egui::TopBottomPanel;
use chrono_tz::Asia::Kolkata;
use chrono_tz::Europe::Berlin;
use chrono_tz::America::New_York;

fn main() {
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
        .with_inner_size([170.0, 200.0])
        .with_always_on_top()
        .with_resizable(false)
        .with_maximize_button(false)
        .with_minimize_button(false),
        ..Default::default()
    };
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

        let central_panel_height = 180.0;
        let panel_height = central_panel_height * 0.2;


        TopBottomPanel::top("panel0")
            .resizable(false)
            .min_height(panel_height)
            .max_height(panel_height)
            .show(ctx, |ui| {
            // local time
            let local_time = calculate_time("local");
            ui.horizontal_centered(|ui| {
                ui.vertical_centered(|ui|{
                    ui.heading(format!("SYD:\t {local_time}"));
                }
                )
        });
        });

        TopBottomPanel::top("panel1")
            .resizable(false)
            .min_height(panel_height)
            .max_height(panel_height)
            .show(ctx, |ui| {
            // local time
            let utc_time = calculate_time("utc");
            ui.vertical_centered(|ui| {
                ui.horizontal_centered(|ui|{
                    ui.heading(format!("UTC:\t {utc_time}"));
                }
                )
        });
        });


        TopBottomPanel::top("panel2")
            .resizable(false)
            .min_height(panel_height)
            .max_height(panel_height)
            .show(ctx, |ui| {
            // BANGALORE
            let blr_time = calculate_time("blr");
            ui.vertical_centered(|ui| {
            ui.heading(format!("BLR:\t {blr_time}"));
        });
        });

        TopBottomPanel::top("panel3")
            .resizable(false)
            .min_height(panel_height)
            .max_height(panel_height)
            .show(ctx, |ui| {
            // ERDING
            let erd_time = calculate_time("erd");
            ui.vertical_centered(|ui| {
            ui.heading(format!("ERD:\t {erd_time}"));
        });
        });

        TopBottomPanel::top("panel5")
            .resizable(false)
            .min_height(panel_height)
            .max_height(panel_height)
            .show(ctx, |ui| {
            // MIAMI
        let mia_time = calculate_time("mia");
        ui.vertical_centered(|ui| {
            ui.heading(format!("MIA:\t {mia_time}"));
        });
        });

        egui::CentralPanel::default().show(ctx, |_ui: &mut egui::Ui| {

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
