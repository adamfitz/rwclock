#![windows_subsystem = "windows"] // Prevents terminal opening on Windows
use chrono::{Local, Utc};
use eframe::egui::{self, Color32, RichText};
use chrono_tz::{Asia::Kolkata, Europe::Berlin, America::New_York};

fn main() {
    let clock_width = 90.0;
    let num_clocks = 5;
    let window_width = (clock_width * num_clocks as f32) + 50 as f32; // adding 50 more pixels because the window layou is not working properly
    let window_height = 40.0; // Height of a single clock panel

    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([window_width, window_height])
            .with_always_on_top()
            .with_resizable(false)
            .with_maximize_button(false)
            .with_minimize_button(false),
        ..Default::default()
    };
    let _ = eframe::run_native("RWC", native_options, Box::new(|cc| Box::new(MyWorldClockApp::new(cc))));
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

        let num_clocks = 5; // Number of clocks
        let clock_height = 40.0; // Matches window height
        let clock_width = ctx.available_rect().width() / num_clocks as f32; // Exact fit

        // Remove top and left gaps
        egui::SidePanel::left("clock_panel")
            .frame(egui::Frame::none()) // Remove default padding/margins
            .show(ctx, |ui| {
                ui.spacing_mut().item_spacing = egui::vec2(0.0, 0.0); // Remove horizontal gaps

                let full_width_rect = ui.available_rect_before_wrap();
                ui.allocate_ui_at_rect(full_width_rect, |ui| {
                    ui.horizontal(|ui| {
                        let clocks = vec![
                            ("LT", calculate_time("local"), Color32::LIGHT_GREEN),
                            ("UTC", calculate_time("utc"), Color32::WHITE),
                            ("BLR", calculate_time("blr"), Color32::RED),
                            ("ERD", calculate_time("erd"), Color32::YELLOW),
                            ("MIA", calculate_time("mia"), Color32::LIGHT_BLUE),
                        ];

                        for (label, time, color) in clocks {
                            ui.allocate_ui_with_layout(
                                egui::Vec2::new(clock_width, clock_height),
                                egui::Layout::top_down(egui::Align::Center),
                                |ui| {
                                    let rect = ui.max_rect();
                                    ui.painter().rect_filled(rect, 0.0, color);
                                    ui.label(RichText::new(format!("{}:\n{}", label, time)).strong());
                                },
                            );
                        }
                    });
                });
            });
    }
   /* 
   fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.request_repaint();

        let central_panel_height = 120.0;
        let panel_height = central_panel_height * 0.2;

        TopBottomPanel::top("panel0")
            .resizable(false)
            .min_height(panel_height)
            .max_height(panel_height)
            .frame(Frame::none().fill(Color32::LIGHT_GREEN))
            .show(ctx, |ui| {
            // local time
            let local_time = calculate_time("local");
            ui.with_layout(egui::Layout::centered_and_justified(egui::Direction::TopDown), |ui| {
                ui.label(RichText::new(format!("LT: \t{local_time}")).strong());
            });
        });

        TopBottomPanel::top("panel1")
            .resizable(false)
            .min_height(panel_height)
            .max_height(panel_height)
            .show(ctx, |ui| {
            // local time
            let utc_time = calculate_time("utc");
            ui.with_layout(egui::Layout::centered_and_justified(egui::Direction::TopDown), |ui| {
                ui.label(RichText::new(format!("UTC:\t{utc_time}")).strong());
            });
        });

        TopBottomPanel::top("panel2")
            .resizable(false)
            .min_height(panel_height)
            .max_height(panel_height)
            .frame(Frame::none().fill(Color32::RED))
            .show(ctx, |ui| {
            // BANGALORE
            let blr_time = calculate_time("blr");
            ui.with_layout(egui::Layout::centered_and_justified(egui::Direction::TopDown), |ui| {
                ui.label(RichText::new(format!("BLR:\t{blr_time}")).strong());
            });
        });

        TopBottomPanel::top("panel3")
            .resizable(false)
            .min_height(panel_height)
            .max_height(panel_height)
            .frame(Frame::none().fill(Color32::YELLOW))
            .show(ctx, |ui| {
            // ERDING
            let erd_time = calculate_time("erd");
            ui.with_layout(egui::Layout::centered_and_justified(egui::Direction::TopDown), |ui| {
                ui.label(RichText::new(format!("ERD:\t{erd_time}")).strong());
            });
        });

        TopBottomPanel::top("panel4")
            .resizable(false)
            .min_height(panel_height)
            .max_height(panel_height)
            .frame(Frame::none().fill(Color32::LIGHT_BLUE))
            .show(ctx, |ui| {
            // MIAMI
            let mia_time = calculate_time("mia");
            ui.with_layout(egui::Layout::centered_and_justified(egui::Direction::TopDown), |ui| {
                ui.label(RichText::new(format!("MIA:\t{mia_time}")).strong());
            });
        });


        egui::CentralPanel::default()
            .show(ctx, |_ui: &mut egui::Ui| {

    );
   }}*/
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
