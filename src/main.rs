#![windows_subsystem = "windows"] // Prevents terminal opening on Windows

use chrono::{Local, Utc};
use eframe::egui::{self, Color32, RichText};
use chrono_tz::{Asia::Kolkata, Europe::Berlin, America::New_York};
use windows::Win32::UI::Input::KeyboardAndMouse::ReleaseCapture;
use raw_window_handle::{HasRawWindowHandle, RawWindowHandle};
use windows::Win32::Foundation::HWND;
use windows::Win32::Foundation::{WPARAM, LPARAM};
use windows::Win32::UI::WindowsAndMessaging::{
    GetWindowLongW, SetWindowLongW, SendMessageW, GWL_STYLE, WS_CAPTION, WS_SYSMENU,
    WM_NCLBUTTONDOWN, HTCAPTION,
};

// Windows API to handle clicking anywhere on the window and dragging it
fn handle_drag(hwnd: HWND) {
    println!("Dragging window... {:?}", hwnd); // Debug output
    unsafe {
        ReleaseCapture();
        SendMessageW(hwnd, WM_NCLBUTTONDOWN, WPARAM(HTCAPTION as usize), LPARAM(0));
    }
}



fn main() {
    let clock_width = 60.0;
    let num_clocks = 5;
    let window_width = (clock_width * num_clocks as f32) + 50.0;
    let window_height = 30.0;

    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([window_width, window_height])
            .with_decorations(false)
            .with_always_on_top()
            .with_resizable(false)
            .with_maximize_button(false)
            .with_minimize_button(false),
        ..Default::default()
    };

    let _ = eframe::run_native("World Clock", native_options, Box::new(|cc| Box::new(MyWorldClockApp::new(cc))));
}

#[derive(Default)]
struct MyWorldClockApp {}

impl MyWorldClockApp {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self::default()
    }
}

impl eframe::App for MyWorldClockApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.request_repaint();

        let num_clocks = 5;
        let clock_height = 40.0;
        let clock_width = ctx.available_rect().width() / num_clocks as f32;

        egui::CentralPanel::default().frame(egui::Frame::none()).show(ctx, |ui| {
            ui.spacing_mut().item_spacing = egui::vec2(0.0, 0.0);
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

        // Make window draggable
        if ctx.input(|i| i.pointer.any_pressed()) {
            if let Ok(handle) = _frame.raw_window_handle() { // Handle the Result properly
                if let RawWindowHandle::Win32(handle) = handle {
                    handle_drag(HWND(isize::from(handle.hwnd)));
                }
            }
        }
    }
}

fn calculate_time(location: &str) -> String {
    match location {
        "local" => Local::now().format("%H:%M:%S").to_string(),
        "utc" => Utc::now().format("%H:%M:%S").to_string(),
        "blr" => Utc::now().with_timezone(&Kolkata).format("%H:%M:%S").to_string(),
        "erd" => Utc::now().with_timezone(&Berlin).format("%H:%M:%S").to_string(),
        "mia" => Utc::now().with_timezone(&New_York).format("%H:%M:%S").to_string(),
        _ => String::from("Invalid location"),
    }
}
