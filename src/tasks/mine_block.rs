use crate::BLOCK_REGISTRY;
use crate::behaviour::actions::Actions;
use crate::behaviour::actions::{BreakBlock, DoNothing};
use crate::behaviour::behaviour::Behaviour;
use crate::behaviour::movements::Movements;
use crate::behaviour::movements::NoInput;
use crate::hierarchical_task_network::hierarchical_task_network::Next;
use crate::registry::block_type::Collision;
use crate::tasks::go_to::GoTo;
use crate::tasks::tasks::Tasks;
use crate::world::world::World;
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

	pub(crate) fn get_next(&mut self) -> Option<Next> {
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
			return Some(Next::Task(Tasks::GoTo(GoTo::new(&self.position))));
		}

		//phase 2: break
		//TODO: implement targeting correct face
		Some(Next::Behaviour(Behaviour {
			movement: Movements::NoInput(NoInput {}),
			action: Actions::BreakBlock(BreakBlock {
				position: self.position,
				face: crate::world::block::Direction::North,
			}),
		}))

		//phase 3 (optional): pickup item
		//TODO: implement once GotoEntity is implemented
	}

	pub(crate) fn complete(&self) -> bool {
		World::get_block(self.position).is_some_and(|block| {
			match BLOCK_REGISTRY
				.get(&block.block_id)
				.expect("block should be in registry")
				.collision
			{
				Collision::Solid => false,
				_ => true,
			}
		})
	}
}
