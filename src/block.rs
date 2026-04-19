pub(crate) struct Block {
	block_type: String,
	metadata: MCMetadata,
	x: int,
	y: int,
	z: int,
}

impl Block {
	
}

//converts the type to a user friendly name
fn type_to_friendly_name(block_type: String) {
	//TODO: implement	
}

//converts the user friendly name to its associated in game type
fn friendly_name_to_type(friendly_name: String) {
	//TODO: implement	
}

enum Direction {
	Down,
	Up,
	North,
	South,
	West,
	East,
}
