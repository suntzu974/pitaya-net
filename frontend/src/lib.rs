#![recursion_limit = "1024"]

#[allow(dead_code)]
mod app;
pub mod components;
pub mod error;
pub mod hooks;
pub mod routes;
pub mod services;
pub mod types;

use app::PitayaApp;
use wasm_bindgen::prelude::*;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen(start)]
pub fn run_app() {
    wasm_logger::init(wasm_logger::Config::default());
    log::info!("Starting app");
    yew::start_app::<PitayaApp>();
}
