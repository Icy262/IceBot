use crate::behaviour::behaviour::Behaviour;
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
				x: player.position.x.floor() as i32,
				y: player.position.y.floor() as i32,
				z: player.position.z.floor() as i32,
			};
		});

		return Self {
			position: *position,
			path: Path::new(&start, position),
			pickup_item: pickup_item,
		};
	}

	pub(crate) fn get_next_behaviour(&mut self) {
		//-> Option<Behaviour> {
		//phase 1: path to block
		let current_pos = PLAYER.with_borrow(|player| {
			return Coordinates {
				x: player.position.x.floor() as i32,
				y: player.position.y.floor() as i32,
				z: player.position.z.floor() as i32,
			};
		});

		//TODO: implement actual line of sight and distance check instead of just checking +- on all axes
		if (self.position.x - current_pos.x).abs() >= 1
			&& (self.position.y - current_pos.y).abs() >= 1
			&& (self.position.z - current_pos.z).abs() >= 1
		{
			//push goto
		}

		//phase 2: break
		//phase 3 (optional): pickup item
	}
}
