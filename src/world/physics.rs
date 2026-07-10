use crate::BLOCK_REGISTRY;
use crate::behaviour::movements::Movements;
use crate::network::packets::Packets;
use crate::registry::block_type::Collision;
use crate::world::entity::Position;
use crate::{
	bot::PLAYER,
	player::Player,
	world::block::{self, Block, Coordinates},
	world::world::{WORLD_MODEL, World},
};

//Not a perfect implementation, but good enough for now
//Calculates one tick of motion. Horizontal acceleration is handled elsewhere
pub(crate) fn process_motion(old_player: &Player) -> Player {
	let mut new_player = (*old_player).clone();

	//From the minecraft wiki: https://minecraft.wiki/w/Entity#Motion
	//Update position
	new_player.position.x += new_player.vx;
	new_player.position.y += new_player.vy;
	new_player.position.z += new_player.vz;

	//Update acceleration
	new_player.vy -= 0.08;

	//Update drag
	new_player.vx *= 0.91;
	new_player.vy *= 0.98;
	new_player.vz *= 0.91;

	//TODO: implement fall damage
	//TODO: implement handling for parital hitboxes like slabs

	if new_player.on_ground {
		//check if player in contact with ground (any of the blocks below are solid)
		//the player is 0.6 blocks wide, so we need to +- 0.3 blocks to get the corners
		let blocks_below = vec![
			World::get_block(Coordinates {
				x: (new_player.position.x + 0.3).floor() as i32,
				y: new_player.position.y.floor() as i32 - 1,
				z: (new_player.position.z + 0.3).floor() as i32,
			}),
			World::get_block(Coordinates {
				x: (new_player.position.x + 0.3).floor() as i32,
				y: new_player.position.y.floor() as i32 - 1,
				z: (new_player.position.z - 0.3).floor() as i32,
			}),
			World::get_block(Coordinates {
				x: (new_player.position.x - 0.3).floor() as i32,
				y: new_player.position.y.floor() as i32 - 1,
				z: (new_player.position.z + 0.3).floor() as i32,
			}),
			World::get_block(Coordinates {
				x: (new_player.position.x - 0.3).floor() as i32,
				y: new_player.position.y.floor() as i32 - 1,
				z: (new_player.position.z - 0.3).floor() as i32,
			}),
		];

		//check if should start falling. if no, set vy to 0 and revert any vertical motion. if yes, keep the falling and set on ground to false
		if blocks_below.into_iter().any(|block| {
			block.is_some_and(|block| {
				match BLOCK_REGISTRY
					.get(&block.block_id)
					.expect("block should be in registry")
					.collision
				{
					Collision::Solid => true,
					_ => false,
				}
			})
		}) {
			new_player.vy = 0.0;
			new_player.position.y = old_player.position.y;
		} else {
			new_player.on_ground = false
		}
	} else {
		//check if player should stop falling by checking each y-level it passes through
		for y_level in old_player.position.y.floor() as i32..new_player.position.y.floor() as i32 {
			//check if player in contact with ground (any of the blocks below are solid)
			//the player is 0.6 blocks wide, so we need to +- 0.3 blocks to get the corners
			//TODO: check how to properly check diagonal motion to avoid incorrect results when for example moving quickly horizontal while also falling
			let blocks_below = vec![
				World::get_block(Coordinates {
					x: (new_player.position.x + 0.3).floor() as i32,
					y: y_level - 1,
					z: (new_player.position.z + 0.3).floor() as i32,
				}),
				World::get_block(Coordinates {
					x: (new_player.position.x + 0.3).floor() as i32,
					y: y_level - 1,
					z: (new_player.position.z - 0.3).floor() as i32,
				}),
				World::get_block(Coordinates {
					x: (new_player.position.x - 0.3).floor() as i32,
					y: y_level - 1,
					z: (new_player.position.z + 0.3).floor() as i32,
				}),
				World::get_block(Coordinates {
					x: (new_player.position.x - 0.3).floor() as i32,
					y: y_level - 1,
					z: (new_player.position.z - 0.3).floor() as i32,
				}),
			];

			//check if should keep falling. if no, set vy to 0 and revert any vertical motion and set on ground to true. if yes, do nothing
			if blocks_below.into_iter().any(|block| {
				block.is_some_and(|block| {
					match BLOCK_REGISTRY
						.get(&block.block_id)
						.expect("block should be in registry")
						.collision
					{
						Collision::Solid => true,
						_ => false,
					}
				})
			}) {
				new_player.vy = 0.0;
				new_player.position.y = y_level as f64;
				new_player.on_ground = true;
			}
		}
	}

	//TODO: implement hitboxes on other axes

	return new_player;
}

//Calls process_motion to calculate the motion and then actually updates the PLAYER
pub(crate) fn update_position() {
	PLAYER.with_borrow_mut(|player| {
		*player = process_motion(player);
	});
}

//Returns the final position of the player if no further inputs are made
pub(crate) fn predict_final_position() -> Position {
	let mut current_player = PLAYER.with_borrow(|player| (*player).clone());

	while current_player.vx.abs() > 0.01
		|| current_player.vy.abs() > 0.01
		|| current_player.vz.abs() > 0.01
	{
		current_player = process_motion(&current_player);
	}

	return current_player.position;
}
