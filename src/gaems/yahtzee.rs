#![allow(non_snake_case)]

use std::{collections::HashMap, io};

use handlebars::{Context, Handlebars, Helper, HelperResult, JsonRender, Output, RenderContext};

use self::hand::Hand;
use super::Game;

pub mod die;
pub mod hand;

const REQUIREMENTS_NOT_MET: &str = "Hand does not meet requirements!";

pub struct Yahtzee {
	gameFinished: bool,
	hand:         Hand,

	ptsTop:    HashMap<u16, u16>,
	ptsBottom: HashMap<u16, u16>,

	sumPreBonus:  u16,
	bonus:        u16,
	sumPostBonus: u16,

	sumBottom: u16,
	sumTop:    u16,
	total:     u16,
}

impl Yahtzee {
	pub fn new() -> Self { Yahtzee { hand: Hand::new(), gameFinished: false, ..Default::default() } }

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
			("aces", self.ptsTop[&1]),
			("twos", self.ptsTop[&2]),
			("thrs", self.ptsTop[&3]),
			("fors", self.ptsTop[&4]),
			("fivs", self.ptsTop[&5]),
			("sixs", self.ptsTop[&6]),
			("preBonus", self.sumPreBonus),
			("bonus", self.bonus),
			("pstBonus", self.sumPostBonus),
			("thrKnd", self.ptsBottom[&7]),
			("forKnd", self.ptsBottom[&8]),
			("fullH", self.ptsBottom[&9]),
			("smStr", self.ptsBottom[&10]),
			("bgStr", self.ptsBottom[&11]),
			("yhtz", self.ptsBottom[&12]),
			("chnc", self.ptsBottom[&13]),
			("sumBot", self.sumBottom),
			("sumTop", self.sumTop),
			("total", self.total),
		]
		.iter()
		.cloned()
		.collect()
	}

	fn xOfAKind(&mut self, x: u8) -> bool {
		for i in 1..=6 {
			if self.hand.countValue(i) >= x {
				return true;
			}
		}
		false
	}

	fn isStraight(&mut self, field: u16) -> bool {
		let mut straight: u8 = 0;

		for i in 1..=3 {
			if self.hand.countValue(i) >= 1 && self.hand.countValue(i + 1) >= 1 && self.hand.countValue(i + 2) >= 1 && self.hand.countValue(i + 3) >= 1 {
				straight = 1;
			}
			if straight == 1 && self.hand.countValue(i + 4) >= 1 {
				straight = 2;
				break;
			}
		}

		return (field == 10 && straight > 0) || (field == 11 && straight == 2);
	}

	fn assignPoints(&mut self, field: u16, value: u16) {
		if !(1..=13).contains(&field) {
			panic!("Field must be in range of 1 to 13");
		}

		if (1..=6).contains(&field) {
			self.ptsTop.insert(field, value);
		} else {
			self.ptsBottom.insert(field, value);
		}
	}
}

impl Default for Yahtzee {
	fn default() -> Self {
		let ptsTop: HashMap<u16, u16> = [(1, 0), (2, 0), (3, 0), (4, 0), (5, 0), (6, 0)].iter().cloned().collect();
		let ptsBottom: HashMap<u16, u16> = [(7, 0), (8, 0), (9, 0), (10, 0), (11, 0), (12, 0), (13, 0)].iter().cloned().collect();

		Yahtzee {
			ptsTop,
			ptsBottom,

			gameFinished: false,
			hand: Default::default(),

			sumPreBonus: Default::default(),
			bonus: Default::default(),
			sumPostBonus: Default::default(),

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
		'gameLoop: loop {
			println!("{esc}c{}\n{}", handlebars.render("scoreboard", &self.updateScoreboard()).unwrap(), msg, esc = (27 as char));
			msg = String::new();

			println!("Choose an action:");
			println!("0. Exit game (currently no saving available)");
			println!("1. Re-roll all dice");
			println!("2. Re-roll specific dice");
			println!("3. Assign points to field");
			let mut choice = String::new();
			io::stdin().read_line(&mut choice).expect("failed to read line");

			let choice: u8 = match choice.trim().parse() {
				Ok(v) => v,
				Err(_) => continue,
			};

			match choice {
				0 => {
					return;
				}
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
						continue 'gameLoop;
					}
					println!("Which dice do you want to re-roll? (seperated by comma, example: `1, 3, 4`); 0 to abort");
					let mut rerollDice = String::new();
					io::stdin().read_line(&mut rerollDice).expect("failed to read line");

					let rerollDice = rerollDice.trim().split(',');
					let mut rDice: Vec<u8> = Vec::new();
					for rerollDie in rerollDice {
						match rerollDie.trim().parse().expect(format!("failed to parse value {}", rerollDie).as_str()) {
							0 => {
								continue 'gameLoop;
							}
							1 => {
								if !rDice.contains(&1) {
									rDice.push(0);
								}
							}
							2 => {
								if !rDice.contains(&2) {
									rDice.push(1);
								}
							}
							3 => {
								if !rDice.contains(&3) {
									rDice.push(2);
								}
							}
							4 => {
								if !rDice.contains(&4) {
									rDice.push(3);
								}
							}
							5 => {
								if !rDice.contains(&5) {
									rDice.push(4);
								}
							}
							_ => {
								msg = format!("Die {} does not exist!", rerollDie);
								continue 'gameLoop;
							}
						}
					}
					self.hand.roll(rDice);
					rolls += 1;
				}
				3 => {
					println!("Insert current hand to which field; 0 to abort");
					let mut field = String::new();
					io::stdin().read_line(&mut field).expect("failed to read line");

					let field: u16 = match field.trim().parse() {
						Ok(v) => v,
						Err(_) => {
							msg = String::from("failed to parse");
							continue;
						}
					};

					if (1..=6).contains(&field) {
						if self.ptsTop[&field] != 0 {
							msg = format!("Field {} already filled in!", field);
							continue 'gameLoop;
						}
					} else if (7..=13).contains(&field) {
						if self.ptsBottom[&field] != 0 {
							msg = format!("Field {} already filled in!", field);
							continue 'gameLoop;
						}
					} else {
						msg = format!("Field {} does not exist!", field);
						continue 'gameLoop;
					}

					match field {
						0 => {
							continue 'gameLoop;
						}
						1..=6 => {
							let count = self.hand.countValue(field as u8) as u16;
							if count != 0 {
								self.assignPoints(field, count * field);
							} else {
								msg = format!("Need at least one of {}!", field);
								continue 'gameLoop;
							}
						}
						7 | 8 | 12 => {
							let fits = match field {
								7 => self.xOfAKind(3),
								8 => self.xOfAKind(4),
								12 => self.xOfAKind(5),
								_ => unreachable!(),
							};

							if fits {
								self.ptsBottom.insert(field, match field {
									7 | 8 => self.hand.sum() as u16,
									12 => 50,
									_ => unreachable!(),
								});
							} else {
								msg = String::from(REQUIREMENTS_NOT_MET);
								continue 'gameLoop;
							}
						}
						9 => {
							let mut double = false;
							let mut triple = false;

							for i in 1..=6 {
								match self.hand.countValue(i) {
									2 => {
										double = true;
									}
									3 => {
										triple = true;
									}
									_ => {}
								}
							}

							if double && triple {
								self.assignPoints(9, 25);
							} else {
								msg = String::from(REQUIREMENTS_NOT_MET);
								continue 'gameLoop;
							}
						}
						10 | 11 => {
							if self.isStraight(field) {
								self.assignPoints(field, match field {
									10 => 30,
									11 => 40,
									_ => unreachable!(),
								});
							} else {
								msg = String::from(REQUIREMENTS_NOT_MET);
								continue 'gameLoop;
							}
						}
						13 => {
							self.assignPoints(13, self.hand.sum() as u16);
						}
						_ => unreachable!(),
					}

					rolls = 0;
					self.hand.rollAll();
				}
				_ => {
					msg = format!("Option {} is not valid", choice);
					continue 'gameLoop;
				}
			}
			self.update();

			if self.gameFinished {
				println!("GAME FINISHED!");
				println!("Your score: {}", self.total);
				return;
			}
		}
	}

	fn update(&mut self) {
		self.sumPreBonus = self.ptsTop.values().sum();

		if self.sumPreBonus >= 63 {
			self.bonus = 35;
		}

		self.sumPostBonus = self.sumPreBonus + self.bonus;

		self.sumBottom = self.ptsBottom.values().sum();
		self.sumTop = self.sumPostBonus;

		self.total = self.sumTop + self.sumBottom;

		self.gameFinished = self.ptsTop.values().all(|&v| v != 0) && self.ptsBottom.values().all(|&v| v != 0);
	}
}
