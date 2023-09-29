use nannou::{
  prelude::*,
  wgpu::{ Backends, DeviceDescriptor, Limits }
};
use std::cell::RefCell;
use crate::console;

use crate::sketch::model::{ Model, Constants };
impl Constants for Model {}

fn update(_app: &App, _model: &mut Model, _update: Update) {
	let win: Rect<f32> = _app.window_rect();
	let frame: u64 = _app.elapsed_frames();

	if frame % 256 == 0 {
		// Every 2^8 frames, generate a new polygon
		_model.new_random_polygon(win.left(), win.bottom());
	}
}

fn view(app: &App, _model: &Model, frame: Frame) {
	let draw: Draw = app.draw();

	draw.background().color(WHITE);
	draw.polyline()
		.weight(Model::WEIGHT)
		.join_round()
		.points_closed(_model.vertices.clone());

	draw.to_frame(app, &frame).unwrap();
}

async fn create_window(app: &App) {
	let device_desc = DeviceDescriptor {
		limits: Limits {
			max_texture_dimension_2d: 8192,
			..Limits::downlevel_webgl2_defaults()
		},
		..Default::default()
	};

	app.new_window()
		.device_descriptor(device_desc)
		.title("harmonic-deferents")
		// .raw_event(raw_event)
		// .key_pressed(key_pressed)
		// .key_released(key_released)
		// .mouse_pressed(mouse_pressed)
		// .mouse_moved(mouse_moved)
		// .mouse_released(mouse_released)
		// .mouse_wheel(mouse_wheel)
		// .touch(touch)
		.view(view)
		.build_async()
		.await
		.unwrap();

	{
		let header: Vec<&str> = vec![
			&"%charmonic deferents",
			&"color: white; font-size: 42px; background-color: black;",
			&"\nauthor:\tCyanMatter",
			&"\nrepo:\thttps://github.com/CyanMatter/harmonic-deferents"
		];

		console::info_all(&header);
	}
}

pub async fn run_app(model: Model) {
	// Since ModelFn is not a closure we need this workaround to pass the calculated model
	thread_local!(static MODEL: RefCell<Option<Model>> = Default::default());

	MODEL.with(|m| m.borrow_mut().replace(model));

	app::Builder::new_async(|app| {
		Box::new(async move {
			create_window(app).await;
			MODEL.with(|m| m.borrow_mut().take().unwrap())
		})
	})
	.backends(Backends::PRIMARY | Backends::GL)
	.update(update)
	.run_async()
	.await;
}