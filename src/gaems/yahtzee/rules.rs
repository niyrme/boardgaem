#![allow(non_snake_case)]

use super::hand::Hand;

pub trait Rule {
	fn name(&self) -> String;
	fn points(&self, hand: Hand) -> u8;
}

pub struct SameValueRule {
	name:  String,
	value: u8,
}

impl SameValueRule {
	pub fn new(name: String, value: u8) -> Self { SameValueRule { name, value } }
}

impl Rule for SameValueRule {
	fn name(&self) -> String { self.name.to_string() }

	fn points(&self, hand: Hand) -> u8 { hand.countValue(self.value) * self.value }
}
