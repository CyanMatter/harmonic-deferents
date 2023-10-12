pub mod console;
pub mod fourier;
pub mod sketch;
pub mod util;

use wasm_bindgen::prelude::wasm_bindgen;
use async_std::task::block_on;
use sketch::{ run_app, Model };

// web app entry_point
#[wasm_bindgen]
pub async fn main_web() {
	let mut model = Model { ..Default::default() };
	
	// !debugging
	console_error_panic_hook::set_once();
	model.set_period_duration(10_f32);
	model.is_repeat_period = true;
	// !debugging
	
	block_on(async {
		run_app(model).await;
	});
}