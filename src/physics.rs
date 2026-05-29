use crate::{block::{self, Block, Coordinates}, bot::PLAYER, world::{WORLD_MODEL, World}};

//Not a perfect implementation, but good enough for now
//Does one tick of motion. Horizontal acceleration is handled elsewhere
pub(crate) fn process_motion() {
	PLAYER.with_borrow_mut(|player| {
		let last_tick_y = player.y;

		//From the minecraft wiki: https://minecraft.wiki/w/Entity#Motion
		//Update position
		player.x += player.vx;
		player.y += player.vy;
		player.z += player.vz;

		//Update acceleration
		player.vy -= 0.08;

		//Update drag
		player.vx *= 0.91;
		player.vy *= 0.98;
		player.vz *= 0.91;

		//TODO: implement fall damage
		//TODO: implement handling for parital hitboxes like slabs
		//check if player colliding with ground (block it is entering is not air)
		if player.on_ground {
			//check if should start falling
			let block_below = World::get_block(crate::block::Coordinates { x: player.x as i32, y: player.y as i32 - 1, z: player.z as i32 });
			if block_below.is_some() {
				//TODO: Implement support for falling through things that are not air but are solid (eg. grass or water)
				if block_below.expect("Should not be None because we checked it is some").block_type == "air" {
					player.on_ground = false;
				}
			}
		} else {
			//check if player should stop falling
			if player.y < last_tick_y.floor() && last_tick_y > last_tick_y.floor() {
				let block_below = World::get_block(crate::block::Coordinates { x: player.x as i32, y: player.y as i32 - 1, z: player.z as i32 });
				if block_below.is_some() {
					//TODO: Implement support for falling through things that are not air but are solid (eg. grass or water)
					if block_below.expect("Should not be None because we checked it is some").block_type != "air" {
						player.on_ground = true;
					}
				}
			}
		}

		//if on ground, stop vertical velocity and unclip the player from the block
		if player.on_ground {
			player.vy = 0.0;
			player.y = player.y.ceil()
		}

		//TODO: implement hitboxes on other axes
	})
}