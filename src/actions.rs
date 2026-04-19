use std::fs::DirBuilder;

//TODO: implement support for online servers
pub (crate) struct Join {
	address: String, //ip or srv record
	username: String, //player username
}

pub (crate) struct Walk {
	block: Block, //block that feet will occupy
}

pub (crate) struct Jump {
}

pub (crate) struct BreakBlock {
	position: Block, //block to break
	face: Direction, //face of block we are hitting
}

//For if we just want a block in a particular position
pub (crate) struct PlaceBlock {
	position: Block, //position of the block
	rotation: Direction, //direction of the placed block
}

//For if we want to specifiy the block we are placing against
pub (crate) struct PlaceBlockAgainst {
	position: Block, //position of the block we are placing against
	rotation: Direction, //direction of the placed block
}
