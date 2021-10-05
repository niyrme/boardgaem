use std::io;

use crate::gaems::{yahtzee::Yahtzee, Game};

pub mod gaems;

fn main() {
	println!("Select a game to play");
	println!(" 0. Exit program");
	println!(" 1. Yahtzee");

	let mut choice = String::new();
	io::stdin().read_line(&mut choice).expect("failed to read line");

	let choice: u16 = choice.trim().parse().expect("failed to parse line");

	match choice {
		0 => return,
		1 => Yahtzee::new().start(),
		_ => println!("Game {} does not exist", choice),
	}
}
