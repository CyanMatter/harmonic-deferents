use nannou::prelude::*;

use nannou::wgpu::{Backends, DeviceDescriptor, Limits};
use nannou::rand::{{distributions::Uniform, Rng}, thread_rng};
use std::cell::RefCell;

pub struct Model;

fn update(_app: &App, _model: &mut Model, _update: Update) {}

fn bound_rand_vec(n: usize, low: f32, high: f32) -> Vec<f32> {
	let bounds = Uniform::from(low..high);
	thread_rng()
		.sample_iter(&bounds)
		.take(n)
		.collect()
}

macro_rules! zip_2_vecs {
	($v:ident, $u:ident) => {
			$v.iter().zip($u.iter())
	};
}

fn random_vertices(n: usize, left: f32, bottom: f32) -> Vec<Point2> {
	let xs: Vec<f32> = bound_rand_vec(n, left, -left);
	let ys: Vec<f32> = bound_rand_vec(n, bottom, -bottom);
	zip_2_vecs!(xs, ys)
		.map(|(&x, &y)| pt2(x, y))
		.collect()
}

fn view(app: &App, _model: &Model, frame: Frame) {
	let win: Rect<f32> = app.window_rect();
	let draw: Draw = app.draw();

	// Clear the background
	draw.background().color(PALEGOLDENROD);

	const N_POINTS: usize = 8;
	let vertices: Vec<Point2> = random_vertices(N_POINTS, win.left(), win.bottom());

	const WEIGHT: f32 = 6.0;

	draw.polyline()
		.weight(WEIGHT)
		.join_round()
		.points_closed(vertices);

	// Write the result of our drawing to the window's frame.
	draw.to_frame(app, &frame).unwrap();
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
}
