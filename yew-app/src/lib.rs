mod app;
mod utils;

use wasm_bindgen::prelude::*;


#[wasm_bindgen]
pub fn init_logger() {
	crate::utils::logger::_init()
}

#[wasm_bindgen]
pub fn run_app() {
	crate::utils::logger::_init();
	yew::start_app::<crate::app::App>();
}