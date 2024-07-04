#![windows_subsystem = "windows"] // this is a crate attribute, it stops the terminal opening by default on windows
use chrono::{Local,Utc};
use eframe::egui::{self, Color32, Frame, TopBottomPanel, RichText};
use chrono_tz::Asia::Kolkata;
use chrono_tz::Europe::Berlin;
use chrono_tz::America::New_York;
use clap::{Parser, ValueEnum};

#[derive(Parser)]
#[command(name = "World Clock")]
#[command(about = "An application to display world clock times", long_about = None)]
struct Cli {
	#[arg(value_enum, default_value = "default")]
	layout: Layout,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum Layout {
	Default,
	Alternate,
}

// Implement the Default trait for the Layout enum
impl Default for Layout {
    fn default() -> Self {
        Layout::Default
    }
}

// Define the Alternate trait
trait Alternate {
    fn alternate() -> Self;
}

// Implement the Default trait for the Layout enum
impl Alternate for Layout {
    fn alternate() -> Self {
        Layout::Alternate
    }
}

fn main() {
    let cli = Cli::parse();

    let layout = cli.layout;

    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
        .with_inner_size([110.0, 120.0])
        .with_always_on_top()
        .with_resizable(false)
        .with_maximize_button(false)
        .with_minimize_button(false),
        ..Default::default()
    };
    
    let _ = eframe::run_native("RWC", native_options, Box::new(move |cc| Box::new(MyWorldClockApp::new(cc, layout))));
}

#[derive(Default)]
struct MyWorldClockApp {
    layout: Layout
}

impl MyWorldClockApp {
    fn new(_cc: &eframe::CreationContext<'_>, layout: Layout) -> Self {
        Self { layout };
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

        let central_panel_height = 120.0;
        let panel_height = central_panel_height * 0.2;

        match self.layout {
			Layout::Default => {
				// Define your default layout panels here
				render_default_layout(ctx, panel_height);
			}
			Layout::Alternate => {
				// Define an alternate layout here
				render_alternate_layout(ctx, panel_height);
			}
		}
        
   }


}



fn render_default_layout(ctx: &egui::Context, panel_height: f32) {
    // Default layout is stacked top to bottom

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

    });
}

fn render_alternate_layout(ctx: &egui::Context, panel_height: f32) {
	// Define an alternate layout
	TopBottomPanel::top("panel0")
		.resizable(false)
		.min_height(panel_height)
		.max_height(panel_height)
		.frame(Frame::none().fill(Color32::LIGHT_GREEN))
		.show(ctx, |ui| {
			let local_time = calculate_time("local");
			ui.with_layout(egui::Layout::centered_and_justified(egui::Direction::TopDown), |ui| {
				ui.label(RichText::new(format!("Local Time: \t{local_time}")).strong());
			});
		});

	// Define other panels for the alternate layout
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
