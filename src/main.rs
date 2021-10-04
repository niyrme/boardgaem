use gaems::Game;

use crate::gaems::yahtzee::Yahtzee;

pub mod gaems;

fn main() { Yahtzee::new().start(); }
