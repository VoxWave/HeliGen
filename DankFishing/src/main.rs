extern crate piston;
extern crate graphics;
extern crate sdl2_window;
extern crate opengl_graphics;

use std::rc::Rc;
use std::cell::RefCell;
use piston::window::WindowSettings;
use piston::event::{ events, RenderArgs, RenderEvent, UpdateArgs,UpdateEvent };

use graphics::{ Context, rectangle, RelativeTransform };
use sdl2_window::Sdl2Window as Window;
use opengl_graphics::{ GlGraphics, OpenGL };

pub struct App {
	gl: GlGraphics,
	rotation: f64
}
impl App {
	fn render(&mut self, args: &RenderArgs) {
		const DARKGREEN: [f32; 4] = [0.0, 0.5, 0.0, 1.0];
		const BLUE: [f32; 4] = [0.0, 0.0, 1.0, 1.0];

		let context = &Context::abs(args.width as f64, args.height as f64);

		let center_context = &context.trans((args.width/2) as f64, (args.height / 2) as f64)
									 .rot_rad(self.rotation)
									 .trans(-25.0, -25.0);
		let square = rectangle::square(0.0, 0.0, 50.0);

		self.gl.draw([0, 0, args.width as i32, args.height as i32], |_, gl| {
			graphics::clear(DARKGREEN, gl);

			graphics::rectangle(BLUE, square, center_context.transform, gl);
		});

	}

	fn update(&mut self, args: &UpdateArgs) {
		self.rotation += 2.0 * args.dt;
	}
}



fn main() {
	let ayy = 12;
	let mut lmao = 11;
	println!("so the value of this integer I call ayy is: {} and lmao is: {}", ayy, lmao);

	lmao += 1;
	println!("now the values are: {} and {}", ayy, lmao);

	println!("Oh btw I'm starting the awesomepart now");
	let window = Window::new(
		OpenGL::_3_2,
		WindowSettings::default()
	);
	let window = Rc::new(RefCell::new(window));

	let mut app = App{
		gl: GlGraphics::new(OpenGL::_3_2),
		rotation: 0.0
	};

	for e in events(window) {
		if let Some(r) = e.render_args() {
			app.render(&r);
		}

		if let Some(u) = e.update_args() {
			app.update(&u);
		}
	}
}
