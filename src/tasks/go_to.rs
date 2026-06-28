use crate::behaviour::actions::{Actions, DoNothing};
use crate::behaviour::behaviour::Behaviour;
use crate::behaviour::movements::{Movements, NoInput};
use crate::pathfinding::pathfind::Path;
use crate::world::block::Coordinates;

//Pathfind to a position and generate the required actions to get there
pub(crate) struct GoTo {
	path: Path,
}

impl GoTo {
	pub(crate) fn new(start: &Coordinates, goal: &Coordinates) -> Self {
		return Self {
			path: Path::initialize(start, goal),
		}
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
