#![forbid(unsafe_code)]
#![cfg_attr(not(debug_assertions), deny(warnings))]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![warn(clippy::all, rust_2018_idioms)]

use eframe::{egui::vec2, epi::IconData};

mod app;
mod config;

fn main() {
    let app = app::App::default();
    let mut native_options = eframe::NativeOptions::default();
    native_options.initial_window_size = Some(vec2(600.0, 300.0));
    native_options.icon_data = set_icon();
    eframe::run_native(Box::new(app), native_options);
}

fn set_icon() -> Option<IconData> {
    let bytes = include_bytes!("../asset/mark.ico");
    let image_buffer = image::load_from_memory(bytes).ok().unwrap();
    let img = image_buffer.to_rgba8();
    let size = (img.width() as u32, img.height() as u32);
    let pixels = img.into_vec();
    let icon_data = IconData {
        rgba: pixels,
        width: size.0,
        height: size.1,
    };
    Some(icon_data)
}
