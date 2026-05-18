use crate::bot::PLAYER;

//Not a perfect implementation, but good enough for now
//Does one tick of motion. Horizontal acceleration is handled elsewhere
pub(crate) fn process_motion() {
	PLAYER.with_borrow_mut(|player| {
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
	})

	//TODO: implement hitboxes
}