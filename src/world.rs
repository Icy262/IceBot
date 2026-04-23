use crate::data_types::MCMetadata;
use crate::entity::Entity;
use crate::block::Block;
use std::collections::HashMap;

struct Chunk {
	//use u8s to store the local block id
	//TODO: implement support for using larger data types for local ids. consider using byte packing to reduce memory footprint
	x: i32, //chunk coordinate
	y: i32,
	palette: Vec<String>, //array of standard ids. The id at index n maps to the local id n
	local_ids: Vec<u8>, //array of local ids
	// metadata: Vec<MCMetadata>, //TODO: implement
}

impl Chunk {
	fn insert_block(block: Block) {

	}

	fn remove_block(block: Block) {

	}

	//TODO: implement
	// fn bulk_update() {}
}

pub(crate) struct World {
	//TODO: implement trimming the loaded blocks
	blocks: HashMap<String, Chunk>, //for performance, we don't use a regular Block as these have too much overhead. instead, break the world into identically sized chunks which can store blocks more efficiently using palettes. use a hashmap so we can store an arbitrary number of chunks that may be far apart in the world. the hashmap key is formed as "x chunk coordinate,y chunk coordinate"
	pub(crate) entities: Entity, //entities in the world
}