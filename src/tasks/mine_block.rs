use std::path::Path;

use crate::world::block::Coordinates;

//Pathfind to next to a block, break it, and (optionally) pick up the item it drops (if it drops one)
pub(crate) struct mine_block {
	position: Coordinates,
	path: Path,
	pickup_item: bool,
}
