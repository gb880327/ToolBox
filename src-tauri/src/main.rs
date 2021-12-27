#![cfg_attr(
all(not(debug_assertions), target_os = "windows"),
windows_subsystem = "windows"
)]
#[macro_use]
extern crate rbatis;
#[macro_use]
extern crate serde;
#[macro_use]
extern crate lazy_static;

use std::sync::Mutex;
use rbatis::rbatis::Rbatis;
use tauri::{LogicalPosition, LogicalSize, Menu, MenuItem, Position, Size};
use app::exec;
use service::Service;

mod model;
mod database;
mod app;
mod gencode;
mod cmd;
mod deploy;
mod service;

lazy_static! {
    static ref RB: Rbatis = Rbatis::new();
    static ref SERVICE: Mutex<Service> = Mutex::new(Service::default());
}

#[tokio::main]
async fn main() {
    app::init(&RB).await.unwrap();

    let menu = Menu::new()
        .add_native_item(MenuItem::Copy)
        .add_native_item(MenuItem::Paste)
        .add_native_item(MenuItem::Cut)
        .add_native_item(MenuItem::Redo)
        .add_native_item(MenuItem::SelectAll)
        .add_native_item(MenuItem::Undo)
        .add_native_item(MenuItem::Quit);
    tauri::Builder::default()
        .menu(menu)
        .on_page_load(move |win, _| {
            win.set_title("Rookie的工具箱").unwrap();
            let position = Position::Logical(LogicalPosition { x: 100 as f64, y: 100 as f64 });
            win.set_position(position).unwrap();
            let size = Size::Logical(LogicalSize { width: 1440 as f64, height: 800 as f64 });
            win.set_size(size).unwrap();
            SERVICE.lock().unwrap().set_win(win.clone())
        })
        .invoke_handler(tauri::generate_handler![exec])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
