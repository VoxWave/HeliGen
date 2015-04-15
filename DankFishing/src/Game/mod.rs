use sdl2::sdl::Sdl;
use sdl2::keycode::KeyCode;
use sdl2::video::{Window, WindowPos, OPENGL};
use sdl2::render::{RenderDriverIndex, ACCELERATED, Renderer};
use sdl2::pixels::Color;

//struct Game {
//	window: Window,
//	renderer: Renderer,
//}

pub fn create_window(sdl_context: Sdl) -> Window {
	match Window::new(&sdl_context, "DankFishing", WindowPos::PosCentered, WindowPos::PosCentered, 800, 600, OPENGL) {
		Ok(window) => window,
		Err(err) => panic!("failed to create window: {}", err)
	};
}

pub fn create_renderer(window: Window) -> Renderer {
	match Renderer::from_window(window, RenderDriverIndex::Auto, ACCELERATED) {
		Ok(renderer) => renderer,
		Err(err) => panic!("failed to create renderer: {}", err)
	};
}

pub fn loop_game(sdl_context: Sdl) {
	let window = create_window(sdl_context);

	//this might fuck things up if I use window in the future.
	//lets hope that doesnt happen.
	let mut renderer = create_renderer(window);

	let mut drawer = renderer.drawer();

	let mut running = true;
	let mut event_pump = sdl_context.event_pump();

	while running {
		drawer.set_draw_color(Color::RGB(0,0,0));
		drawer.clear();
		drawer.present();
		for event in event_pump.poll_iter() {
			use sdl2::event::Event;

			match event {
				Event::Quit {..} | Event::KeyDown { keycode: KeyCode::Escape, .. } => {
					running = false
				},
				_ => {}
			}
		}
		//pelilogiikkaa
	}
}
