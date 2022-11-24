#![recursion_limit = "256"]
mod services;
mod models;
mod types;
mod error;
mod routes;
mod components;
mod app;

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
