use crate::behaviour::actions::{Actions, DoNothing};
use crate::behaviour::behaviour::Behaviour;
use crate::behaviour::movements::{Movements, NoInput};
use crate::bot::PLAYER;
use crate::pathfinding::pathfind::Path;
use crate::world::block::Coordinates;

//Pathfind to a position and generate the required actions to get there
pub(crate) struct GoTo {
	path: Path,
}

impl GoTo {
	pub(crate) fn new(goal: &Coordinates) -> Self {
		let start = PLAYER.with_borrow(|player| {
			return Coordinates {
				x: player.x.floor() as i32,
				y: player.y.floor() as i32,
				z: player.z.floor() as i32,
			}
		});
		
		return Self {
			path: Path::initialize(start, goal),
		};
	}

	pub(crate) fn get_next_behaviour(&self) -> Behaviour {
		return Behaviour {
			movement: Movements::NoInput(NoInput {}),
			action: Actions::DoNothing(DoNothing {}),
		};
	}

	pub(crate) fn complete(self) -> bool {
		return false;
	}
}
