#[derive(Hash, PartialEq, Eq, Clone, Copy, PartialOrd, Ord)]
pub(crate) struct Coordinates {
	pub(crate) x: i32,
	pub(crate) y: i32,
	pub(crate) z: i32,
}

pub(crate) struct Block {
	pub(crate) block_id: String,
	//pub(crate) metadata: Metadata,
	pub(crate) position: Coordinates,
}

pub(crate) enum Direction {
	Down,
	Up,
	North,
	South,
	West,
	East,
}
