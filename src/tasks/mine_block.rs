use crate::{bot::PLAYER, pathfinding::pathfind::Path, world::block::Coordinates};

//Pathfind to next to a block, break it, and (optionally) pick up the item it drops (if it drops one)
pub(crate) struct MineBlock {
	position: Coordinates,
	path: Path,
	pickup_item: bool,
}

impl MineBlock {
	pub(crate) fn new(position: &Coordinates, pickup_item: bool) -> Self {
		let start = PLAYER.with_borrow(|player| {
			return Coordinates {
				x: player.x.floor() as i32,
				y: player.y.floor() as i32,
				z: player.z.floor() as i32,
			};
		});

		return Self {
			position: *position,
			path: Path::new(&start, position),
			pickup_item: pickup_item,
		};
	}

	pub(crate) fn get_next_behaviour(&mut self) -> Option<Behaviour> {
		//phase 1: path to block
		//phase 2: break
		//phase 3 (optional): pickup item
	}
}
