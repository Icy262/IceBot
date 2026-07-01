use crate::behaviour::actions::{Actions, DoNothing};
use crate::behaviour::behaviour::Behaviour;
use crate::behaviour::movements::{Movements, NoInput};
use crate::world::block::Coordinates;
use crate::world::world::{WORLD_MODEL, World};

//Remove all the blocks from a given region
pub(crate) struct ClearRegion {
	pub(crate) start_corner: Coordinates,
	pub(crate) end_corner: Coordinates,
	//y-coordinate of the highest block in the region
	current_highest_y: usize,
}

//we will start from the top layer and work down
//to prevent using excessive memory, only generate the tasks for a single layer at a time
//TODO: implement max number of tasks generated at once

impl ClearRegion {
	pub(crate) fn get_next_behaviour(&mut self) -> Behaviour {
		return Behaviour {
			movement: Movements::NoInput(NoInput {}),
			action: Actions::DoNothing(DoNothing {}),
		};
	}

	pub(crate) fn complete(&self) -> bool {
		return false;
	}

	//good enough for now. lawnmower aproach
	fn find_next_block() {}
}
