extern crate sdl2;

use sdl2::video::{Window, WindowPos, OPENGL};
use sdl2::render::{RenderDriverIndex, ACCELERATED, Renderer};
use sdl2::keycode::KeyCode;

mod Game;

fn main() {
	println!("Starting the Dankest Fishing game ever...");

	let sdl_context = sdl2::init(sdl2::INIT_EVERYTHING).unwrap();

	Game::loop_game(sdl_context);
}
