use crate::behaviour::behaviour::Behaviour;
use crate::world::block::Coordinates;
use crate::behaviour::movements::{Movements, NoInput};
use crate::behaviour::actions::{Actions, DoNothing};

//Pathfind to a position and generate the required actions to get there
pub(crate) struct GoTo {
	pub(crate) position: Coordinates,
}

impl GoTo {
	pub(crate) fn get_next_behaviour(&self) -> Behaviour {
		return Behaviour { movement: Movements::NoInput(NoInput {}), action: Actions::DoNothing(DoNothing {})};
	}

	pub(crate) fn complete(self) -> bool {
		return false;
	}
}
