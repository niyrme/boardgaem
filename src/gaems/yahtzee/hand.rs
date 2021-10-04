#![allow(non_snake_case)]

use core::fmt;

use super::die::Die;

pub struct Hand {
	pub dice: Vec<Die>,
	pub hand: Vec<u8>,
}

// Core
impl Hand {
	pub fn new() -> Self {
		let mut hand: Vec<u8> = Vec::new();
		let mut dice: Vec<Die> = Vec::new();

		for _ in 0..5 {
			let mut die = Die::new();
			hand.push(die.roll());
			dice.push(die);
		}

		Self { dice, hand }
	}

	pub fn rollAll(&mut self) {
		for die in self.dice.iter_mut() {
			die.roll();
		}
	}

	pub fn roll(&mut self, dice: Vec<u8>) {
		// Because we can't index [Die] with u8?
		for (i, die) in self.dice.iter_mut().enumerate() {
			if dice.contains(&(i as u8)) {
				die.roll();
			}
		}
	}

	pub fn countValue(&self, value: u8) -> u8 { self.dice.iter().filter(|&n| n.face == value).count() as u8 }

	pub fn sum(&self) -> u8 {
		let mut sum: u8 = 0;

		self.dice.iter().for_each(|die| {
			sum += die.face;
		});

		return sum;
	}
}

// Testing
impl Hand {
	pub fn setFaces(&mut self, dice: Vec<Die>) { self.dice = dice; }
}

impl Default for Hand {
	fn default() -> Self {
		Hand {
			dice: vec![Default::default(), Default::default(), Default::default(), Default::default(), Default::default()],
			hand: vec![Default::default(), Default::default(), Default::default(), Default::default(), Default::default()],
		}
	}
}

impl fmt::Display for Hand {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let mut st = String::new();

		for (i, die) in self.dice.iter().enumerate() {
			st.push_str(format!("Die {} has value {}\n", i + 1, die.face).as_str());
		}

		write!(f, "{}", st)
	}
}
