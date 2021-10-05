pub mod yahtzee;

pub trait Game {
	fn start(&mut self) { self.run(); }
	fn run(&mut self);
	fn update(&mut self);
}
