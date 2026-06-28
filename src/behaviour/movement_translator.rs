use crate::bot::PLAYER;
use crate::network::data_types::*;
use crate::network::packets::*;
use crate::world::physics::update_position;

use crate::behaviour::movements::Jump;
impl Jump {
	pub(crate) fn to_packets(movement: Jump) -> Vec<Packets> {
		let mut packets: Vec<Packets> = Vec::new();

		PLAYER.with_borrow_mut(|player| {
			//accelerate the player 0.42 b/t upward
			player.vy += 0.42;
		});

		//process motion
		update_position();

		PLAYER.with_borrow(|player| {
			packets.push(Packets::PlayerPositionandLook(PlayerPositionandLook {
				x: MCDouble { value: player.x },
				y: MCDouble { value: player.y },
				z: MCDouble { value: player.z },
				stance: MCDouble {
					value: player.y as f64 + 1.62,
				},
				yaw: MCFloat {
					value: player.yaw as f32,
				},
				pitch: MCFloat {
					value: player.pitch as f32,
				},
				on_ground: MCBool {
					value: player.on_ground,
				},
			}))
		});

		return packets;
	}
}

use crate::behaviour::movements::Walk;
impl Walk {
	pub(crate) fn to_packets(movement: Walk) -> Vec<Packets> {
		//TODO: implement stance properly
		let mut packets: Vec<Packets> = Vec::new();

		PLAYER.with_borrow_mut(|player| {
			//accelerate the player a total of 0.2 blocks/tick on the x/z axes
			player.vx += -0.2 * player.yaw.sin();
			player.vz += 0.2 * player.yaw.cos();
		});

		//process motion
		update_position();

		PLAYER.with_borrow(|player| {
			packets.push(Packets::PlayerPositionandLook(PlayerPositionandLook {
				x: MCDouble { value: player.x },
				y: MCDouble { value: player.y },
				z: MCDouble { value: player.z },
				stance: MCDouble {
					value: player.y as f64 + 1.62,
				},
				yaw: MCFloat {
					value: player.yaw as f32,
				},
				pitch: MCFloat {
					value: player.pitch as f32,
				},
				on_ground: MCBool {
					value: player.on_ground,
				},
			}))
		});

		return packets;
	}
}

use crate::behaviour::movements::NoInput;
impl NoInput {
	pub(crate) fn to_packets(movement: NoInput) -> Vec<Packets> {
		let mut packets: Vec<Packets> = Vec::new();

		//process motion
		update_position();

		PLAYER.with_borrow(|player| {
			packets.push(Packets::PlayerPositionandLook(PlayerPositionandLook {
				x: MCDouble { value: player.x },
				y: MCDouble { value: player.y },
				z: MCDouble { value: player.z },
				stance: MCDouble {
					value: player.y as f64 + 1.62,
				},
				yaw: MCFloat {
					value: player.yaw as f32,
				},
				pitch: MCFloat {
					value: player.pitch as f32,
				},
				on_ground: MCBool {
					value: player.on_ground,
				},
			}))
		});

		return packets;
	}
}
