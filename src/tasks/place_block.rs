use crate::hierarchical_task_network::hierarchical_task_network::Next;
use crate::world::block::Coordinates;
use crate::world::world::World;

//Place a specified block at a specified location. handles finding the block in inventory or obtaining it from the world, selecting it in hand, and placing it
pub(crate) struct PlaceBlock {
	position: Coordinates,
	block: String,
}

impl PlaceBlock {
	pub(crate) fn get_next(&mut self) -> Option<Next> {
		//phase 1: path to block
		//phase 2: select block
		//phase 3: place
		return None;
	}

	pub(crate) fn complete(&self) -> bool {
		World::get_block(self.position).is_some_and(|block| self.block == block.block_id)
	}
}
