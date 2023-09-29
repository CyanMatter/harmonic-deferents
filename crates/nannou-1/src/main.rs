// native app entry_point
mod sketch;
mod console;
mod util;

use sketch::{ run_app, Model };
use async_std::task::block_on;

fn main() {
	let model = Model { ..Default::default() };
	block_on(async {
		run_app(model).await;
	});
}
