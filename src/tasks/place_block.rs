use crate::{hierarchical_task_network::hierarchical_task_network::Next, world::block::Coordinates};

//Place a specified block at a specified location. handles finding the block in inventory or obtaining it from the world, selecting it in hand, and placing it
pub(crate) struct PlaceBlock {
	position: Coordinates,
	block: String,
}
