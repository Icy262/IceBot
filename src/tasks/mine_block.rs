use crate::{pathfinding::pathfind::Path, world::block::Coordinates};

//Pathfind to next to a block, break it, and (optionally) pick up the item it drops (if it drops one)
pub(crate) struct MineBlock {
	position: Coordinates,
	path: Path,
	pickup_item: bool,
}

impl MineBlock {
	pub(crate) fn new(position: &Coordinates, pickup_item: bool) -> Self {
		return Self {
			position: *position,
			path: Path::new(position),
			pickup_item: pickup_item,
		};
	}
}
