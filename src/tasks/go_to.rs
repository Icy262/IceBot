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
				x: player.x.floor() as i32,
				y: player.y.floor() as i32,
				z: player.z.floor() as i32,
			};
		});

		return Self {
			destination: *goal,
			path: Path::new(&start, goal),
		};
	}

	pub(crate) fn get_next_behaviour(&mut self) -> Option<Next> {
		println!("1");
		let current_pos = PLAYER.with_borrow(|player| {
			return Coordinates {
				x: player.x.floor() as i32,
				y: player.y.floor() as i32,
				z: player.z.floor() as i32,
			};
		});

		self.path.update_position(&current_pos);
		let next_position = self.path.trace_path(&current_pos)?;

		//TODO: add support for breaking blocks by pushing a block break to the queue and climbing ladders and stuff
		let movement = if next_position.y > current_pos.y {
			Movements::Jump(Jump {})
		} else {
			Movements::Walk(Walk {})
		};

		PLAYER.with_borrow_mut(|player| {
			//add 0.5 so we target center of block
			let dx = next_position.x as f64 + 0.5 - player.x;
			let dz = next_position.z as f64 + 0.5 - player.z;

			player.pitch = 0.0;
			player.yaw = -(dx.atan2(dz)).to_degrees();
		});

		Some(Next::Behaviour(Behaviour {
			movement: movement,
			action: Actions::DoNothing(DoNothing {}),
		}))
	}

	pub(crate) fn complete(&self) -> bool {
		let current_pos = PLAYER.with_borrow(|player| {
			return Coordinates {
				x: player.x.floor() as i32,
				y: player.y.floor() as i32,
				z: player.z.floor() as i32,
			};
		});

		return current_pos == self.destination;
	}
}
