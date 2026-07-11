use crate::behaviour::actions::{Actions, DoNothing};
use crate::behaviour::behaviour::Behaviour;
use crate::behaviour::movements::{Jump, Movements, NoInput, Walk};
use crate::bot::PLAYER;
use crate::hierarchical_task_network::hierarchical_task_network::Next;
use crate::pathfinding::pathfind::Path;
use crate::world::block::Coordinates;

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
		println!("1");
		let current_pos = PLAYER.with_borrow(|player| {
			return Coordinates {
				x: player.position.x.floor() as i32,
				y: player.position.y.floor() as i32,
				z: player.position.z.floor() as i32,
			};
		});

		self.path.update_position(&current_pos);
		let next_position = self.path.trace_path(&current_pos)?;

		Some(
			match next_position.y - current_pos.y {
				1 => {
					//must jump up a block
					//check if block above head is free
						//check if in air
							//jump
							//push task to place block below feet
						//push task to break block above head
				},
				0 => {
					//must walk forward
					//check if both blocks in front free and if block below is solid
						//walk forward
						//push tasks to break blocks in front and place solid block below
				},
				-1 => {
					//must drop/mine down a block
					//check if block below is non-solid
						//do nothing and fall
						//break block below
				},
				_ => panic!("next_position invalid (not within 1 block of current_pos)"),
			}
		)

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
