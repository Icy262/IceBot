use crate::data_types::MCMetadata;

pub(crate) struct EntityPositionAndLook {
	pub(crate) x: f64,
	pub(crate) y: f64,
	pub(crate) z: f64,
	pub(crate) yaw: f64,
	pub(crate) pitch: f64,
	pub(crate) on_ground: bool,
}

pub(crate) struct Entity {
	//TODO: implement
	// pub(crate) block_type: String,
	// pub(crate) metadata: MCMetadata,
}
