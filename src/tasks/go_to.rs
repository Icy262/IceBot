use crate::BLOCK_REGISTRY;
use crate::behaviour::actions::{self, Actions, DoNothing};
use crate::behaviour::behaviour::Behaviour;
use crate::behaviour::movements::{Jump, Movements, NoInput, Walk};
use crate::bot::PLAYER;
use crate::hierarchical_task_network::hierarchical_task_network::Next;
use crate::pathfinding::pathfind::Path;
use crate::registry::block_type::Collision;
use crate::tasks::mine_block;
use crate::tasks::place_block;
use crate::tasks::tasks::Tasks;
use crate::world::block::Coordinates;
use crate::world::entity::Position;
use crate::world::world::World;

//Pathfind to a position and generate the required actions to get there
pub(crate) struct GoTo {
	destination: Coordinates,
	path: Path,
}

impl GoTo {
	pub(crate) fn new(goal: &Coordinates) -> Self {
		let start = PLAYER.with_borrow(|player| {
			return Coordinates {
				x: player.position.x.floor() as i32,
				y: player.position.y.floor() as i32,
				z: player.position.z.floor() as i32,
			};
		});

		return Self {
			destination: *goal,
			path: Path::new(&start, goal),
		};
	}

	pub(crate) fn get_next(&mut self) -> Option<Next> {
		let current_position = PLAYER.with_borrow(|player| {
			return Position {
				x: player.position.x,
				y: player.position.y,
				z: player.position.z,
			};
		});

		let current_coordinate = Coordinates {
			x: current_position.x.floor() as i32,
			y: current_position.y.floor() as i32,
			z: current_position.z.floor() as i32,
		};

		self.path.update_position(&current_coordinate);
		let next_position = self.path.trace_path(&current_coordinate)?;

		Some(match next_position.y as f64 - current_position.y {
			//must jump up a block
			difference if difference > 0f64 => {
				let block_above = World::get_block(Coordinates {
					y: current_coordinate.y + 2,
					..current_coordinate
				})?;

				//check if block above head is free
				if BLOCK_REGISTRY.get(block_above.block_id.as_str())?.collision == Collision::Solid {
					//push task to break block above head
					Next::Task(Tasks::MineBlock(mine_block::MineBlock::new(
						&block_above.position,
						false,
					)))
				} else {
					//jump and place block below feet
					if current_position.y.floor() == current_position.y {
						Next::Behaviour(Behaviour {
							movement: Movements::Jump(Jump {}),
							action: Actions::DoNothing(DoNothing {}),
						})
					} else {
						//TODO: implement selecting a cheap block
						Next::Task(Tasks::PlaceBlock(place_block::PlaceBlock {
							position: current_coordinate,
							block: String::from("dirt"),
						}))
					}
				}
			}
			difference if difference == 0f64 => {
				//must walk forward
				//check if both blocks in front free and if block below is solid
				//get the blocks at eye, foot, and below the player to see if breaking or placing is necessary
				let position_head = Coordinates {
					y: current_coordinate.y + 1,
					..current_coordinate
				};
				let position_feet = Coordinates {
					..current_coordinate
				};
				let position_support = Coordinates {
					y: current_coordinate.y - 1,
					..current_coordinate
				};

				let block_head = World::get_block(position_head)?;
				let block_feet = World::get_block(position_feet)?;
				let block_support = World::get_block(position_support)?;

				//push tasks to break blocks in front and place solid block below
				if BLOCK_REGISTRY
					.get(block_head.block_id.as_str())
					.expect("block id should be in registry")
					.collision == Collision::Solid {
					Next::Task(Tasks::MineBlock(mine_block::MineBlock::new(
						&block_head.position,
						false,
					)))
				} else if BLOCK_REGISTRY
					.get(&block_feet.block_id)
					.expect("block id should be in registry")
					.collision == Collision::Solid {
					Next::Task(Tasks::MineBlock(mine_block::MineBlock::new(
						&block_feet.position,
						false,
					)))
				} else if BLOCK_REGISTRY
					.get(block_support.block_id.as_str())
					.expect("block id should be in registry")
					.collision != Collision::Solid {
					//TODO: implement selecting a cheap block
					Next::Task(Tasks::PlaceBlock(place_block::PlaceBlock {
						position: block_support.position,
						block: String::from("dirt"),
					}))
				//walk forward
				} else {
					Next::Behaviour(Behaviour {
						movement: Movements::Walk(Walk {}),
						action: Actions::DoNothing(DoNothing {}),
					})
				}
			}
			difference if difference < 0f64 => {
				//must drop/mine down a block
				//check if block below is non-solid
				let position_support = Coordinates {
					y: current_coordinate.y - 1,
					..current_coordinate
				};
				let block_support = World::get_block(position_support)?;

				if BLOCK_REGISTRY.get(&block_support.block_id)?.collision != Collision::Solid {
					//do nothing and fall
					Next::Behaviour(Behaviour {
						movement: Movements::NoInput(NoInput {}),
						action: Actions::DoNothing(DoNothing {}),
					})
				} else {
					//break block below
					Next::Task(Tasks::MineBlock(mine_block::MineBlock::new(
						&block_support.position,
						false,
					)))
				}
			}
			_ => panic!("next_position invalid (not within 1 block of current_pos)"),
		})

		//PLAYER.with_borrow_mut(|player| {
		//	//add 0.5 so we target center of block
		//	let dx = next_position.x as f64 + 0.5 - player.position.x;
		//	let dz = next_position.z as f64 + 0.5 - player.position.z;

		//	player.pitch = 0.0;
		//	player.yaw = -(dx.atan2(dz)).to_degrees();
		//});
	}

	pub(crate) fn complete(&self) -> bool {
		let current_pos = PLAYER.with_borrow(|player| {
			return Coordinates {
				x: player.position.x.floor() as i32,
				y: player.position.y.floor() as i32,
				z: player.position.z.floor() as i32,
			};
		});

		return current_pos == self.destination;
	}
}
