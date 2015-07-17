use opengl_graphics::GlGraphics;
use piston_window::*;

struct View {
	g: GlGraphics
}

impl View {
	pub fn new(g: GlGraphics) -> View {
		View {
			g: g
		}
	}

	pub fn render<T: Renderable>(&mut self, c: Context, r: T) {

	}
}

pub trait Renderable {
	fn draw(&self, window: &PistonWindow);
}

