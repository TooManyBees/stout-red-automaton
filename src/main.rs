use nannou::prelude::*;
use cells::{Automata};

const SCALE_FACTOR: u32 = 4;

fn main() {
	nannou::app(model).update(update).view(view).run();
}

struct Model {
	automata: Automata,
}

fn model(app: &App) -> Model {
	let size = 64usize;
	app.new_window()
		.with_dimensions(size as u32 * SCALE_FACTOR, size as u32 * SCALE_FACTOR)
		.build()
		.unwrap();
//    app.set_loop_mode(LoopMode::loop_ntimes(size as usize));
	app.set_loop_mode(LoopMode::loop_once());
	Model {
		automata: Automata::new(size as usize),
	}
}

fn update(_app: &App, model: &mut Model, _update: Update) {
	for _ in 0..model.automata.size() {
		model.automata.advance();
	}
}

fn view(app: &App, model: &Model, frame: &Frame) {
	const PIXEL_SIZE: f32 = SCALE_FACTOR as f32;
	let draw = app.draw();
	let win = app.window_rect();
	frame.clear(WHITE);
	for (y, generation) in model.automata.generations().enumerate() {
		for (x, &alive) in generation.iter().enumerate() {
			if alive {
				draw.rect()
					.x_y(win.left() + x as f32 * PIXEL_SIZE + PIXEL_SIZE / 2.0, win.top() - y as f32 * PIXEL_SIZE - PIXEL_SIZE / 2.0)
					.w_h(PIXEL_SIZE, PIXEL_SIZE)
					.color(BLACK);
			}
		}
	}

	draw.to_frame(app, &frame).unwrap();
}
