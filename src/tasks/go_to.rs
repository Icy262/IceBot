use crate::behaviour::behaviour::Behaviour;
use crate::world::block::Coordinates;

//Pathfind to a position and generate the required actions to get there
pub(crate) struct GoTo {
	pub(crate) position: Coordinates,
}

impl GoTo {
	pub(crate) fn get_next_behaviour(&self) -> Behaviour {
	}

	pub(crate) fn complete(self) -> bool {
	}
}
