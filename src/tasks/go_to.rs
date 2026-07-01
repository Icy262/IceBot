use crate::behaviour::actions::{Actions, Look};
use crate::behaviour::behaviour::Behaviour;
use crate::behaviour::movements::{Jump, Movements, NoInput, Walk};
use crate::bot::PLAYER;
use crate::pathfinding::pathfind::Path;
use crate::world::block::Coordinates;

//Pathfind to a position and generate the required actions to get there
pub(crate) struct GoTo {
	destination: Coordinates,
	path: Path,
}

impl GoTo {
	pub(crate) fn new(goal: &Coordinates) -> Self {
		let start = PLAYER.with_borrow(|player| {
			return Coordinates {
				x: player.x.floor() as i32,
				y: player.y.floor() as i32,
				z: player.z.floor() as i32,
			};
		});

		return Self {
			destination: *goal,
			path: Path::new(&start, goal),
		};
	}

	pub(crate) fn get_next_behaviour(&mut self) -> Option<Behaviour> {
		let current_pos = PLAYER.with_borrow(|player| {
			return Coordinates {
				x: player.x.floor() as i32,
				y: player.y.floor() as i32,
				z: player.z.floor() as i32,
			};
		});

		self.path.update_position(&current_pos);
		let next_position = self.path.trace_path(&current_pos)?;

		let movement = if next_position.y > current_pos.y {
			Movements::Jump(Jump {})
		} else {
			Movements::Walk(Walk {})
		};

		return Some(Behaviour {
			movement: movement,
			action: Actions::Look(Look {
				target: next_position,
			}),
		});
	}

	pub(crate) fn complete(&self) -> bool {
		let current_pos = PLAYER.with_borrow(|player| {
			return Coordinates {
				x: player.x.floor() as i32,
				y: player.y.floor() as i32,
				z: player.z.floor() as i32,
			};
		});

		return current_pos == self.destination;
	}
}
