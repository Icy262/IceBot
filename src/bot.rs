use std::cell::RefCell;

use crate::player::Player;

thread_local! {
	pub(crate) static PLAYER: RefCell<Player> = RefCell::new(
		Player {
			x: 0f64,
			y: 0f64,
			z: 0f64,
			falling: false,
			facing: 0f64,
			elevation: 0f64,
		}
	);
}

pub(crate) fn bot_main(username: String, server: String) {
	println!("aa");
}
