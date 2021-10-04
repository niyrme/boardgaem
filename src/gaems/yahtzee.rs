#![allow(non_snake_case)]

use std::{collections::HashMap, io};

use handlebars::{Context, Handlebars, Helper, HelperResult, JsonRender, Output, RenderContext};

use self::hand::Hand;
use super::Game;

pub mod die;
pub mod hand;
pub mod rules;

pub struct Yahtzee {
	hand: Hand,

	ptsAces: u16,
	ptsTwos: u16,
	ptsThrees: u16,
	ptsFours: u16,
	ptsFives: u16,
	ptsSixes: u16,

	sumPreBonus: u16,
	bonus: u16,
	sumPostBonus: u16,

	threeOfAKind: u16,
	fourOfAKind: u16,
	fullHouse: u16,
	smallStraight: u16,
	bigStraight: u16,
	yahtzee: u16,
	chance: u16,

	sumBottom: u16,
	sumTop: u16,
	total: u16,
}

impl Yahtzee {
	pub fn new() -> Self {
		Yahtzee {
			hand: Hand::new(),
			ptsAces: Default::default(),
			ptsTwos: Default::default(),
			ptsThrees: Default::default(),
			ptsFours: Default::default(),
			ptsFives: Default::default(),
			ptsSixes: Default::default(),
			sumPreBonus: Default::default(),
			bonus: Default::default(),
			sumPostBonus: Default::default(),
			threeOfAKind: Default::default(),
			fourOfAKind: Default::default(),
			fullHouse: Default::default(),
			smallStraight: Default::default(),
			bigStraight: Default::default(),
			yahtzee: Default::default(),
			chance: Default::default(),
			sumBottom: Default::default(),
			sumTop: Default::default(),
			total: Default::default(),
		}
	}

	fn updateScoreboard(&mut self) -> HashMap<&str, u16> {
		let mut values: [u16; 5] = Default::default();

		for (i, die) in self.hand.dice.iter().enumerate() {
			values[i] = die.face as u16;
		}

		[
			("die1", values[0]),
			("die2", values[1]),
			("die3", values[2]),
			("die4", values[3]),
			("die5", values[4]),
			("aces", self.ptsAces),
			("twos", self.ptsTwos),
			("thrs", self.ptsThrees),
			("fors", self.ptsFours),
			("fivs", self.ptsFives),
			("sixs", self.ptsSixes),
			("preBonus", self.sumPreBonus),
			("bonus", self.bonus),
			("pstBonus", self.sumPostBonus),
			("thrKnd", self.threeOfAKind),
			("forKnd", self.fourOfAKind),
			("fullH", self.fullHouse),
			("smStr", self.smallStraight),
			("bgStr", self.bigStraight),
			("yhtz", self.yahtzee),
			("chnc", self.chance),
			("sumBot", self.sumBottom),
			("sumTop", self.sumTop),
			("total", self.total),
		]
		.iter()
		.cloned()
		.collect()
	}
}

impl Default for Yahtzee {
	fn default() -> Self {
		Yahtzee {
			hand: Default::default(),
			ptsAces: Default::default(),
			ptsTwos: Default::default(),
			ptsThrees: Default::default(),
			ptsFours: Default::default(),
			ptsFives: Default::default(),
			ptsSixes: Default::default(),
			sumPreBonus: Default::default(),
			bonus: Default::default(),
			sumPostBonus: Default::default(),
			threeOfAKind: Default::default(),
			fourOfAKind: Default::default(),
			fullHouse: Default::default(),
			smallStraight: Default::default(),
			bigStraight: Default::default(),
			yahtzee: Default::default(),
			chance: Default::default(),
			sumBottom: Default::default(),
			sumTop: Default::default(),
			total: Default::default(),
		}
	}
}

fn paddingHelper(h: &Helper, _: &Handlebars, _: &Context, _: &mut RenderContext, out: &mut dyn Output) -> HelperResult {
	let padDepth = h.param(0).unwrap();
	let padChar = h.param(1).unwrap();
	let padParam = h.param(2).unwrap();

	let paramLen = padParam.value().to_string().len() as u64;

	let mut i = ((padDepth.value().as_u64().unwrap() as i64) - (paramLen as i64)) as i64;

	if i < 0 {
		i = 0;
	}

	let mut pad = String::new();
	for _ in 0..i {
		pad.push_str(padChar.value().render().as_str());
	}

	pad.push_str(padParam.value().render().as_str());

	out.write(&pad)?;

	Ok(())
}

impl Game for Yahtzee {
	fn start(&mut self) {
		self.run()
	}

	fn run(&mut self) {
		let mut handlebars = Handlebars::new();
		handlebars.set_strict_mode(true);
		handlebars.register_helper("padder", Box::new(paddingHelper));

		#[cfg(debug_assertions)]
		// Load template dynamically from path
		handlebars.register_template_file("scoreboard", "src/gaems/templates/yahtzee.hbs").unwrap();

		#[cfg(not(debug_assertions))]
		// Compile template into binary
		handlebars.register_template_string("scoreboard", include_str!("templates/yahtzee.hbs")).unwrap();

		let mut rolls: u8 = 0;
		let mut msg = String::new();
		loop {
			println!("{esc}c{}\n{}", handlebars.render("scoreboard", &self.updateScoreboard()).unwrap(), msg, esc=(27 as char));
			println!("Choose an action:");
			println!("1. Re-roll all dice");
			println!("2. Re-roll specific dice");
			println!("3. Assign points to field");
			println!("4. Exit game (currently no saving available)");
			let mut choice = String::new();
			io::stdin().read_line(&mut choice).expect("failed to read line");

			let choice: u8 = match choice.trim().parse() {
				Ok(v) => v,
				Err(_) => continue,
			};

			match choice {
				1 => {
					if rolls >= 3 {
						msg = String::from("Can roll only 3 times per turn! Please set a value now");
						continue;
					}
					self.hand.rollAll();
					rolls += 1;
				}
				2 => {
					if rolls >= 3 {
						msg = String::from("Can roll only 3 times per turn! Please set a value now");
						continue;
					}
					println!("Which dice do you want to re-roll? (seperated by comma, without spaces; example: `1,3,4`)");
					let mut rerollDice = String::new();
					io::stdin().read_line(&mut rerollDice).expect("failed to read line");

					let rerollDice = rerollDice.trim().split(',');
					let mut rDice: Vec<u8> = Vec::new();
					for rerollDie in rerollDice {
						match rerollDie {
							"1" => {
								if !rDice.contains(&1) {
									rDice.push(0);
								}
							}
							"2" => {
								if !rDice.contains(&2) {
									rDice.push(1);
								}
							}
							"3" => {
								if !rDice.contains(&3) {
									rDice.push(2);
								}
							}
							"4" => {
								if !rDice.contains(&4) {
									rDice.push(3);
								}
							}
							"5" => {
								if !rDice.contains(&5) {
									rDice.push(4);
								}
							}
							_ => {
								msg = format!("\nDie {} does not exist!", rerollDie);
								continue;
							}
						}
					}
					self.hand.roll(rDice);
					rolls += 1;
				}
				3 => {
					println!("Insert current hand to which field");
					let mut field = String::new();
					io::stdin().read_line(&mut field).expect("failed to read line");

					let field: u16 = match field.trim().parse() {
						Ok(v) => v,
						Err(_) => {
							msg = String::from("failed to parse");
							continue;
						}
					};

					match field {
						1 => {
							if self.ptsAces != 0 {
								msg = String::from("Aces already filled in");
								continue;
							}
							let count = self.hand.countValue(1);
							if count != 0 {
								self.ptsAces = (self.hand.countValue(1) * 1) as u16;
							}
							else {
								msg = String::from("Hand does not meet requirements");
								continue;
							}
						}
						2 => {
							if self.ptsTwos != 0 {
								msg = String::from("Twos already filled in");
								continue;
							}
							let count = self.hand.countValue(2);
							if count != 0 {
								self.ptsTwos = (self.hand.countValue(2) * 2) as u16;
							}
							else {
								msg = String::from("Hand does not meet requirements");
								continue;
							}
						}
						3 => {
							if self.ptsThrees != 0 {
								msg = String::from("Threes already filled in");
								continue;
							}
							let count = self.hand.countValue(3);
							if count != 0 {
								self.ptsThrees = (self.hand.countValue(3) * 3) as u16;
							}
							else {
								msg = String::from("Hand does not meet requirements");
								continue;
							}
						}
						4 => {
							if self.ptsFours != 0 {
								msg = String::from("Fours already filled in");
								continue;
							}
							let count = self.hand.countValue(4);
							if count != 0 {
								self.ptsFours = (self.hand.countValue(4) * 4) as u16;
							}
							else {
								msg = String::from("Hand does not meet requirements");
								continue;
							}
						}
						5 => {
							if self.ptsFives != 0 {
								msg = String::from("Fives already filled in");
								continue;
							}
							let count = self.hand.countValue(5);
							if count != 0 {
								self.ptsFives = (self.hand.countValue(5) * 5) as u16;
							}
							else {
								msg = String::from("Hand does not meet requirements");
								continue;
							}
						}
						6 => {
							if self.ptsSixes != 0 {
								msg = String::from("Sixes already filled in");
								continue;
							}
							let count = self.hand.countValue(6);
							if count != 0 {
								self.ptsSixes = (self.hand.countValue(6) * 6) as u16;
							}
							else {
								msg = String::from("Hand does not meet requirements");
								continue;
							}
						}
						7 => {
							if self.threeOfAKind != 0 {
								msg = String::from("Three of a Kind already filled in");
								continue;
							}
							let mut fits = false;
							for i in 1..=6 {
								if self.hand.countValue(i) >= 3 {
									fits = true;
									self.threeOfAKind = self.hand.sum() as u16;
									break;
								}
							}
							if !fits {
								msg = String::from("Hand does not match reqirement");
								continue;
							}
						}
						8 => {
							if self.fourOfAKind != 0 {
								msg = String::from("Four of a Kind already filled in");
								continue;
							}
							let mut fits = false;
							for i in 1..=6 {
								if self.hand.countValue(i) >= 4 {
									fits = true;
									self.fourOfAKind = self.hand.sum() as u16;
									break;
								}
							}
							if !fits {
								msg = String::from("Hand does not match reqirement");
								continue;
							}
						}
						9 => {
							if self.fullHouse != 0 {
								msg = String::from("Full House already filled in");
								continue;
							}
							let mut double = 0;
							let mut triple = 0;

							for i in 1..=6 {
								if self.hand.countValue(i) == 2 {
									double = i;
								}
								else if self.hand.countValue(i) == 3 {
									triple = i;
								}
							}

							if !(double == 0 && triple == 0) {
								self.fullHouse = 25;
							}
							else {
								msg = String::from("Hand does not meet requirements");
							}
						}
						10 => {
							if self.smallStraight != 0 {
								msg = String::from("Small straight already filled in");
								continue;
							}
							for i in 1..=3 {
								if self.hand.countValue(i) >= 1
									&& self.hand.countValue(i + 1) >= 1
									&& self.hand.countValue(i + 2) >= 1
									&& self.hand.countValue(i + 3) >= 1 {
									self.smallStraight = 30;
									break;
								}
							}
							msg = String::from("Hand does not meet requirements");
							continue;
						}
						11 => {
							if self.bigStraight != 0 {
								msg = String::from("Big straight already filled in");
								continue;
							}
							for i in 1..=2 {
								if self.hand.countValue(i) >= 1
									&& self.hand.countValue(i + 1) >= 1
									&& self.hand.countValue(i + 2) >= 1
									&& self.hand.countValue(i + 3) >= 1
									&& self.hand.countValue(i + 4) >= 1 {
									self.bigStraight = 40;
									break;
								}
							}
							msg = String::from("Hand does not meet requirements");
							continue;
						}
						12 => {
							if self.yahtzee != 0 {
								msg = String::from("Yahtzee already filled in");
								continue;
							}
							let mut fits = false;
							for i in 1..=6 {
								if self.hand.countValue(i) == 5 {
									fits = true;
									self.yahtzee = 50;
									break;
								}
							}
							if !fits {
								msg = String::from("Hand does not match reqirement");
								continue;
							}
						}
						13 => {
							if self.chance != 0 {
								msg = String::from("Chance already filled in");
								continue;
							}
							self.chance = self.hand.sum() as u16;
						}
						_ => {
							msg = format!("\nField {} does not exist", field);
							continue;
						}
					}
					rolls = 0;
					self.hand.rollAll();
				}
				4 => {
					return;
				}
				_ => {
					msg = format!("\nOption {} is not valid", choice);
					continue;
				}
			}
			self.update();
		}
	}

	fn update(&mut self) {
		self.sumPreBonus = [
			self.ptsAces,
			self.ptsTwos,
			self.ptsThrees,
			self.ptsFours,
			self.ptsFives,
			self.ptsSixes,
		].iter().sum();

		if self.sumPreBonus > 63 {
			self.bonus = 35;
		}

		self.sumPostBonus = self.sumPreBonus + self.sumPostBonus;

		self.sumBottom = [
			self.threeOfAKind,
			self.fourOfAKind,
			self.fullHouse,
			self.smallStraight,
			self.bigStraight,
			self.yahtzee,
			self.chance,
		].iter().sum();
		self.sumTop = self.sumPostBonus;

		self.total = self.sumTop + self.sumBottom;
	}
}
