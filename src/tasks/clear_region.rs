use crate::world::block::Coordinates;
use crate::behaviour::behaviour::Behaviour;
use crate::world::world::{WORLD_MODEL, World};
use crate::behaviour::movements::{Movements, NoInput};
use crate::behaviour::actions::{Actions, DoNothing};

//Remove all the blocks from a given region
pub(crate) struct ClearRegion {
	pub(crate) start_corner: Coordinates,
	pub(crate) end_corner: Coordinates,
}

impl ClearRegion {
	pub(crate) fn get_next_behaviour(&self) -> Behaviour {
		return Behaviour { movement: Movements::NoInput(NoInput {}), action: Actions::DoNothing(DoNothing {})};
	}
	
	pub(crate) fn complete(self) -> bool {
		return false;
	}
}
