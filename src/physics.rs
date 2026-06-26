use crate::movements::Movements;
use crate::network::packets::Packets;
use crate::{
	block::{self, Block, Coordinates},
	bot::PLAYER,
	player::Player,
	world::{WORLD_MODEL, World},
};

//Not a perfect implementation, but good enough for now
//Calculates one tick of motion. Horizontal acceleration is handled elsewhere
pub(crate) fn process_motion(old_player: &Player) -> Player {
	let mut new_player = (*old_player).clone();

	//From the minecraft wiki: https://minecraft.wiki/w/Entity#Motion
	//Update position
	new_player.x += new_player.vx;
	new_player.y += new_player.vy;
	new_player.z += new_player.vz;

	//Update acceleration
	new_player.vy -= 0.08;

	//Update drag
	new_player.vx *= 0.91;
	new_player.vy *= 0.98;
	new_player.vz *= 0.91;

	//TODO: implement fall damage
	//TODO: implement handling for parital hitboxes like slabs
	//check if player colliding with ground (block it is entering is not air)
	if new_player.on_ground {
		//check if should start falling
		let block_below = World::get_block(crate::block::Coordinates {
			x: new_player.x as i32,
			y: new_player.y as i32 - 1,
			z: new_player.z as i32,
		});
		if block_below.is_some() {
			//TODO: Implement support for falling through things that are not air but are solid (eg. grass or water)
			if block_below
				.expect("Should not be None because we checked it is some")
				.block_id == "air"
			{
				new_player.on_ground = false;
			} else {
				//if on ground, stop vertical velocity and unclip the old_player from the block below
				new_player.vy = 0.0;
				new_player.y = new_player.y.ceil()
			}
		}
	} else {
		//check if player should stop falling
		if new_player.y < old_player.y.floor() && old_player.y > old_player.y.floor() {
			let block_below = World::get_block(crate::block::Coordinates {
				x: new_player.x as i32,
				y: new_player.y as i32 - 1,
				z: new_player.z as i32,
			});
			if block_below.is_some() {
				//TODO: Implement support for falling through things that are not air but are solid (eg. grass or water)
				if block_below
					.expect("Should not be None because we checked it is some")
					.block_id != "air"
				{
					new_player.on_ground = true;
				}
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
pub(crate) fn predict_final_position() -> Player {
	let mut current_player = PLAYER.with_borrow(|player| (*player).clone());

	while current_player.vx.abs() < 0.01
		&& current_player.vy.abs() < 0.01
		&& current_player.vz.abs() < 0.01
	{
		current_player = process_motion(&current_player);
	}

	return current_player;
}
