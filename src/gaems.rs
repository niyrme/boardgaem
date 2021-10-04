pub mod yahtzee;

pub trait Game {
	fn start(&mut self);
	fn run(&mut self);
	fn update(&mut self);
}
