use sdl2::sdl::Sdl;
use sdl2::keycode::KeyCode;

pub fn loop_game(sdl_context: Sdl) {
	let mut running = true;
	let mut event_pump = sdl_context.event_pump();

	while running {
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