use crate::data_types::MCMetadata;

pub(crate) struct Coordinates {
	pub(crate) x: i32,
	pub(crate) y: i32,
	pub(crate) z: i32,
}

pub(crate) struct Block {
	pub(crate) block_type: String,
	pub(crate) metadata: MCMetadata,
	pub(crate) position: Coordinates,
}

impl Block {}

//converts the type to a user friendly name
fn type_to_friendly_name(block_type: String) {
	//TODO: implement
}

//converts the user friendly name to its associated in game type
fn friendly_name_to_type(friendly_name: String) {
	//TODO: implement
}

pub(crate) enum Direction {
	Down,
	Up,
	North,
	South,
	West,
	East,
}
