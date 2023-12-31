use nannou::{
  prelude::*,
  wgpu::{ Backends, DeviceDescriptor, Limits }
};
use std::cell::RefCell;
use crate::console;
use super::model::{ Model, Constants, epicycle::compute_all_renders };

impl Constants for Model {}

static mut IS_FIRST_PASS: bool = true;	// !debug

fn update(_app: &App, _model: &mut Model, _update: Update) {
	if _model.is_repeat_period {
		// !Debug
		let win: Rect<f32> = _app.window_rect();
		// Every 2^8 frames, generate a new polygon
		_model.new_random_polygon(win.left(), win.bottom());
	}

	/*
	if unsafe { IS_FIRST_PASS } {
		_model.load_simple_square();
		unsafe { IS_FIRST_PASS = false };
		for (i, e) in _model.epicycles.iter().enumerate() {
			let fq = e.frequency;
			let r = e.radius;
			let ph = e.phase;
			console::log(format!("Epicycle {i}:\n\tfq:\t{fq}\n\tr:\t{r}\n\tph:\t{ph}\n", ));
		}
	}
	*/

	_model.advance_time(_update.since_last);
	compute_all_renders(_model);
}

fn view(app: &App, _model: &Model, frame: Frame) {
	let draw: Draw = app.draw();

	// Draw all components
	// Background
	draw.background().color(WHITE);
	// Epicycles
	for epicycle in _model.epicycles.iter() {
		let ellipse = epicycle.ellipse
			.as_ref()
			.unwrap()
			.clone();
		draw.a(ellipse).finish();
	}
	// Random polygon
	draw.polyline()
		.weight(Model::WEIGHT)
		.join_round()
		.points_closed(_model.sketch_vertices.clone())
		.finish();
	// Resampled vertices on polygon
	for v in _model.resampled_vertices.iter() {
		draw.ellipse()
			.xy(*v)
			.radius(1.0)
			.color(MAGENTA);
	}
	// Epicycle path
	draw.polyline()
		.weight(Model::WEIGHT)
		.join_round()
		.color(nannou::color::DODGERBLUE)
		.points(_model.epicycle_path.clone())
		.finish();

	draw.to_frame(app, &frame).unwrap();
}

async fn create_window(app: &App) {
	let device_desc = DeviceDescriptor {
		limits: Limits {
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
			"%charmonic deferents",
			"color: white; font-size: 42px; background-color: black;",
			"\nauthor:\tCyanMatter",
			"\nrepo:\thttps://github.com/CyanMatter/harmonic-deferents",
			"\n"
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