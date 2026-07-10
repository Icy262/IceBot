use crate::network::data_types::MCMetadata;

#[derive(Clone, Copy)]
pub(crate) struct Position {
	pub(crate) x: f64,
	pub(crate) y: f64,
	pub(crate) z: f64,
}

pub(crate) struct EntityPositionAndLook {
	pub(crate) position: Position,
	pub(crate) yaw: f64,
	pub(crate) pitch: f64,
	pub(crate) on_ground: bool,
}

pub(crate) struct Entity {
	//TODO: implement
	// pub(crate) block_id: String,
	// pub(crate) metadata: MCMetadata,
}
