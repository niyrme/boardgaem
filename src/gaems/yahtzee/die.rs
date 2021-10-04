#![allow(non_snake_case)]

use core::fmt;

use rand::{self, prelude::SliceRandom};

pub struct Die {
	pub face: u8,
	sides: Vec<u8>,
}

// Core
impl Die {
	pub fn new() -> Self {
		Die { face: 0, sides: vec![1, 2, 3, 4, 5, 6] }
	}

	pub fn roll(&mut self) -> u8 {
		let face = *self.sides.choose(&mut rand::thread_rng()).unwrap();
		self.face = face;
		return face;
	}
}

// Testing
impl Die {
	pub fn setFace(&mut self, face: u8) {
		self.face = face;
	}
}

impl Default for Die {
	fn default() -> Self {
		let mut die = Die { face: 0, sides: vec![1, 2, 3, 4, 5, 6] };

		die.roll();
		return die;
	}
}

impl fmt::Display for Die {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "Faces: {:?} | Face value: {}", self.sides, self.face)
	}
}
