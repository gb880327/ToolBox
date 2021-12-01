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

use rbatis::rbatis::Rbatis;
use app::exec;

mod model;
mod database;
mod app;
mod gencode;
mod cmd;
mod deploy;
mod service;

lazy_static! {
    static ref RB: Rbatis = Rbatis::new();
}

#[tokio::main]
async fn main() {
    app::init(&RB).await.unwrap();
    tauri::Builder::default()
        .on_page_load(move |win, _| {

        })
        .invoke_handler(tauri::generate_handler![exec])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
