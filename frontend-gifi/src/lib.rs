use yew::prelude::*;
use wasm_bindgen::prelude::*;
mod app;
mod routes;
mod services;
mod types;

#[wasm_bindgen(start)]
pub fn run_app() {
    wasm_logger::init(wasm_logger::Config::default());
    log::info!("Starting app");
    yew::start_app::<App>();
}
