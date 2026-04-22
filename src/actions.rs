use crate::block::{Block, Direction};

//TODO: implement support for online servers
pub (crate) struct Join {
	pub(crate) address: String, //ip or srv record
	pub(crate) username: String, //player username
}

pub (crate) struct Walk {
	pub(crate) block: Block, //block that feet will occupy
}

pub (crate) struct Look {
	pub(crate) block: Block, //block that bot will look at
}

pub (crate) struct Jump {
}

pub (crate) struct BreakBlock {
	pub(crate) position: Block, //block to break
	pub(crate) face: Direction, //face of block we are hitting
}

//For if we just want a block in a particular position
pub (crate) struct PlaceBlock {
	pub(crate) position: Block, //position of the block
	pub(crate) rotation: Direction, //direction of the placed block
}

//For if we want to specifiy the block we are placing against
pub (crate) struct PlaceBlockAgainst {
	pub(crate) position: Block, //position of the block we are placing against
	pub(crate) rotation: Direction, //direction of the placed block
}
