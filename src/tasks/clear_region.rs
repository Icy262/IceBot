use crate::world::block::Coordinates;
use crate::behaviour::behaviour::Behaviour;

//Remove all the blocks from a given region
pub(crate) struct ClearRegion {
	pub(crate) start_corner: Coordinates,
	pub(crate) end_corner: Coordinates,
}

impl ClearRegion {
	pub(crate) fn get_next_behaviour(&self) -> Behaviour {
	}
	
	pub(crate) fn complete(self) -> bool {
	}
}
